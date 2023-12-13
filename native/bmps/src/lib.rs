pub use config::Config;
use image::{imageops::FilterType, DynamicImage, GenericImage, GenericImageView, Rgba};
use std::{path::Path, sync::OnceLock, time::Instant};

pub mod box_shadow;
pub mod config;
pub use log;

pub fn round_img(origin: String, output: String, radius: u32) -> anyhow::Result<()> {
    let mut img = open_img(origin)?;
    round(radius, &mut img);
    img.save(output)?;
    Ok(())
}
struct RoundChecker {
    width: u32,
    height: u32,
    radius: u32,
}

fn dist(p1: (u32, u32), p2: (u32, u32)) -> u32 {
    let a = if p1.0 > p2.0 {
        p1.0 - p2.0
    } else {
        p2.0 - p1.0
    };
    let b = if p1.1 > p2.1 {
        p1.1 - p2.1
    } else {
        p2.1 - p1.1
    };
    a * a + b * b
}
impl RoundChecker {
    fn new(radius: u32, width: u32, height: u32) -> Self {
        RoundChecker {
            width,
            height,
            radius,
        }
    }
    fn contains(&self, x: u32, y: u32) -> bool {
        let radius = self.radius;
        if x >= radius && x <= self.width - radius {
            return true;
        }
        if y >= radius && y <= self.height - radius {
            return true;
        }
        let squre = radius * radius;
        let positions = [
            (radius, radius),
            (self.width - radius, radius),
            (radius, self.height - radius),
            (self.width - radius, self.height - radius),
        ];
        for pos in positions {
            if dist(pos, (x, y)) <= squre {
                return true;
            }
        }
        false
    }
}

fn round(radius: u32, img: &mut DynamicImage) {
    const EMPTY: Rgba<u8> = Rgba([255, 255, 255, 0]);
    let width = img.width();
    let height = img.height();
    let squre = radius * radius;
    let pos = (radius, radius);
    for x in 0..radius {
        for y in 0..radius {
            let tmp = dist(pos, (x, y));
            if tmp >= squre {
                img.put_pixel(x, y, EMPTY);
            }
            if tmp <= squre {
                break;
            }
        }
    }
    let pos = (width - radius, radius);
    for x in ((width - radius)..width).rev() {
        for y in 0..radius {
            let tmp = dist(pos, (x, y));
            if tmp >= squre {
                img.put_pixel(x, y, EMPTY);
            }
            if tmp <= squre {
                break;
            }
        }
    }
    let pos = (radius, height - radius);
    for x in 0..radius {
        for y in ((height - radius)..height).rev() {
            let tmp = dist(pos, (x, y));
            if tmp >= squre {
                img.put_pixel(x, y, EMPTY);
            }
            if tmp <= squre {
                break;
            }
        }
    }
    let pos = (width - radius, height - radius);
    for x in (pos.0..width).rev() {
        for y in (pos.1..height).rev() {
            let tmp = dist(pos, (x, y));
            if tmp >= squre {
                img.put_pixel(x, y, EMPTY);
            }
            if tmp <= squre {
                break;
            }
        }
    }
}

fn blur(radius: f32, img: DynamicImage) -> DynamicImage {
    let s = Instant::now();
    let w = img.width();
    let h = img.height();
    let mut data: Vec<[u8; 3]> = img.clone().into_rgb8().pixels().map(|p| p.0).collect();
    fastblur::gaussian_blur(&mut data, w as usize, h as usize, radius);
    let i = image::RgbImage::from_raw(w, h, data.into_iter().flatten().collect()).unwrap();
    log::info!("blur cost: {}ms", s.elapsed().as_millis());
    DynamicImage::ImageRgb8(i)
}

pub fn blur_img(radius: f32, origin: String, out: String) -> anyhow::Result<()> {
    let img = open_img(origin)?;
    let bg = blur(radius, img);
    bg.save(out)?;
    Ok(())
}

static FONT_FAMILIES: OnceLock<Vec<String>> = OnceLock::new();

pub fn font_families() -> Vec<String> {
    let res = FONT_FAMILIES.get_or_init(|| {
        vec!["hello".to_owned()]
        // font_kit::source::SystemSource::new()
        //     .all_families()
        //     .unwrap()
    });
    res.clone()
}
// https://magnushoff.com/articles/jpeg-orientation/
fn get_orientation<P: AsRef<Path>>(path: P) -> anyhow::Result<u32> {
    use exif::In;
    use exif::Tag;
    let file = std::fs::File::open(path)?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader)?;
    match exif.get_field(Tag::Orientation, In::PRIMARY) {
        Some(orientation) => match orientation.value.get_uint(0) {
            Some(v @ 1..=8) => Ok(v),
            _ => Err(anyhow::Error::msg("orientation invalid")),
        },
        None => Err(anyhow::Error::msg("orientation missing")),
    }
}
fn open_img<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<DynamicImage> {
    let mut img = image::open(path.as_ref())?;
    if let Ok(v) = get_orientation(path.as_ref()) {
        match v {
            1 => {} // noting to do
            2 => {
                img = img.fliph();
            }
            3 => {
                img = img.rotate180();
            }
            4 => {
                img = img.flipv();
            }
            5 => {
                img = img.rotate90();
                img = img.fliph();
            }
            6 => {
                img = img.rotate90();
            }
            7 => {
                img = img.flipv();
                img = img.rotate270();
            }
            8 => {
                img = img.rotate270();
            }
            _ => {}
        }
    }
    Ok(img)
}

pub fn go(cfg: Config) -> anyhow::Result<()> {
    let img = open_img(cfg.source_file.as_str())?;
    let mut bg_img = if cfg.white_bg {
        DynamicImage::ImageRgb8(image::RgbImage::from_pixel(
            cfg.size.width,
            cfg.size.height,
            image::Rgb([255, 255, 255]),
        ))
    } else {
        img.resize_to_fill(cfg.size.width, cfg.size.height, FilterType::Nearest)
    };
    let r = 1.0 - cfg.size.padding * 2.0;
    let width = bg_img.width() as f64 * r;
    let height = bg_img.height() as f64 * r;
    let img = img.resize(width as u32, height as u32, FilterType::Nearest);
    let dist_v = (bg_img.height() - img.height()) / 2;
    let dist_h = (bg_img.width() - img.width()) / 2;
    //  draw shadow
    //  FIXME: https://html.spec.whatwg.org/multipage/canvas.html#when-shadows-are-drawn
    //  FIXME: https://stackoverflow.com/questions/4151896/what-is-the-precise-explanation-of-box-shadow-and-moz-box-shadow-in-css
    let darker = |p: Rgba<u8>| {
        const D: u8 = 100;
        let d = p.0.map(|v| if v < D { 0 } else { v - D });
        Rgba(d)
    };
    let shadow = cfg.size.shadow;
    let draw_shadow_cost = Instant::now();
    for x in (dist_h - shadow)..(bg_img.width() - dist_h + shadow) {
        for y in (dist_v - shadow)..(bg_img.height() - dist_v + shadow) {
            let p = bg_img.get_pixel(x, y);
            bg_img.put_pixel(x, y, darker(p));
        }
    }
    log::info!(
        "draw_shadow_cost: {}ms",
        draw_shadow_cost.elapsed().as_millis()
    );

    let mut bg_img = blur(cfg.size.blur_radius as f32, bg_img);
    let checker = RoundChecker::new(cfg.size.blur_radius, img.width(), img.height());
    // draw main content
    for x in 0..img.width() {
        for y in 0..img.height() {
            if !checker.contains(x, y) {
                continue;
            }
            let mut p = img.get_pixel(x, y);
            p.0[3] = 255;
            bg_img.put_pixel(x + dist_h, y + dist_v, p);
        }
    }
    if cfg.source_file == cfg.dest_file || cfg.dest_file.is_empty() {
        let mut pb = std::path::PathBuf::from(cfg.source_file.as_str());
        let name = format!(
            "bmps_{}",
            pb.file_name().unwrap_or_default().to_string_lossy()
        );
        pb.pop();
        pb.push(name);
        log::info!("saving to {:?}", pb.as_path());
        bg_img.save(pb.as_path())?;
    }
    log::info!("saving to {}", cfg.dest_file);
    bg_img.save(cfg.dest_file.as_str())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use image::{Pixel, RgbaImage};

    use super::*;

    #[test]
    fn fonts() {
        let _ = font_families();
    }
    #[test]
    fn gogogo() {
        let s = Instant::now();
        let mut cfg = Config {
            source_file: "./hello.jpg".to_owned(),
            dest_file: "./output_p.jpg".to_owned(),
            font: None,
            size: Default::default(),
            white_bg: false,
        };
        go(cfg.clone()).unwrap();
        std::mem::swap(&mut cfg.size.width, &mut cfg.size.height);
        cfg.dest_file = "./output_l.jpg".to_owned();
        go(cfg).unwrap();
        log::info!("cost {}ms", s.elapsed().as_millis());
    }

    #[test]
    fn shadow() {
        let _ = env_logger::try_init();
        let s = Instant::now();
        let img = open_img("./hello.jpg").unwrap();
        let img = img.resize_to_fill(1920, 1080, FilterType::Nearest);
        let (bg, dx, dy) = box_shadow::EffectBuilder::new()
            .color([100, 100, 100, 255])
            .blur_radius(50)
            .offset(100, 100)
            .build()
            .gen_bg(&img);
        log::info!("cost {}ms, offset({dx},{dy})", s.elapsed().as_millis());
        let mut res = RgbaImage::from_pixel(
            bg.width() + 200,
            bg.height() + 200,
            Rgba([255, 255, 255, 255]),
        );
        for x in 0..bg.width() {
            for y in 0..bg.height() {
                let p1 = res.get_pixel_mut(x + 100, y + 100);
                let p2 = bg.get_pixel(x, y);
                p1.blend(p2);
            }
        }
        for x in 0..img.width() {
            for y in 0..img.height() {
                let p = img.get_pixel(x, y);
                res.put_pixel(x + 100 + dx, y + 100 + dy, p);
            }
        }
        res.save("./output_shadow.png").unwrap();
    }
}
