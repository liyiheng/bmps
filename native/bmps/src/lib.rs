pub use config::Config;
use image::{imageops::FilterType, DynamicImage, GenericImage, GenericImageView, Rgba};
use std::{sync::OnceLock, time::Instant};

pub mod config;
pub use log;

pub fn round_img(origin: String, output: String, radius: u32) -> anyhow::Result<()> {
    let mut img = image::open(origin)?;
    round(radius, &mut img);
    img.save(output)?;
    Ok(())
}

fn round(radius: u32, img: &mut DynamicImage) {
    const EMPTY: Rgba<u8> = Rgba([255, 255, 255, 0]);
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
    let img = image::open(origin)?;
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

pub fn go(cfg: Config) -> anyhow::Result<()> {
    let img = image::open(cfg.source_file.as_str())?;
    let mut bg_img = img.resize_to_fill(cfg.size.width, cfg.size.height, FilterType::Nearest);
    let r = 1.0 - cfg.size.padding * 2.0;
    let width = bg_img.width() as f64 * r;
    let height = bg_img.height() as f64 * r;
    let mut img = img.resize(width as u32, height as u32, FilterType::Nearest);
    let dist_v = (bg_img.height() - img.height()) / 2;
    let dist_h = (bg_img.width() - img.width()) / 2;
    //  draw shadow
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
    // draw main content
    round(cfg.size.round_radius, &mut img);
    for x in 0..img.width() {
        for y in 0..img.height() {
            let mut p = img.get_pixel(x, y);
            if p.0[..3] == [255, 255, 255] || p.0[3] == 0 {
                continue;
            }
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
        };
        go(cfg.clone()).unwrap();
        std::mem::swap(&mut cfg.size.width, &mut cfg.size.height);
        cfg.dest_file = "./output_l.jpg".to_owned();
        go(cfg).unwrap();
        log::info!("cost {}ms", s.elapsed().as_millis());
    }
}
