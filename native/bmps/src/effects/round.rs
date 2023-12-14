pub struct Rounded<'a> {
    origin: &'a image::DynamicImage,
    checker: Checker,
}
pub struct Checker {
    pub width: u32,
    pub height: u32,
    pub radius: u32,
}
#[cfg(test)]
mod test {
    use image::GenericImageView;
    use image::Pixel;

    use super::apply;
    use super::Checker;
    use super::Rounded;

    #[test]
    fn round() {
        let img = image::open("./hello.jpg").unwrap();
        let mut img = image::DynamicImage::ImageRgba8(
            img.resize_to_fill(1920, 1080, image::imageops::FilterType::Nearest)
                .to_rgba8(),
        );
        let mut tmp = image::RgbaImage::from_pixel(img.width(), img.height(), image::Rgba([0; 4]));
        let r = Rounded::new(&img, 40);
        tmp.enumerate_pixels_mut().for_each(|(x, y, p)| {
            let v = r.get_pixel(x, y);
            p.blend(&v);
        });
        apply(&mut img, 40);
        img.save("output_round.png").unwrap();
        tmp.save("output_round1.png").unwrap();
    }
    #[test]
    fn checker() {
        let c = Checker {
            width: 100,
            height: 100,
            radius: 30,
        };
        assert!(!c.contains(0, 0));
        assert!(!c.contains(8, 8));
        assert!(!c.contains(99, 99));
        assert!(!c.contains(93, 93));
        assert!(!c.contains(99, 0));
        assert!(!c.contains(0, 99));
        assert!(c.contains(50, 50));
    }
}
pub fn apply(img: &mut image::DynamicImage, radius: u32) {
    let checker = Checker {
        width: img.width(),
        height: img.height(),
        radius,
    };
    let walk = |mut cb: Box<dyn FnMut(u32, u32)>| {
        for x in 0..checker.width {
            for y in 0..checker.height {
                if checker.contains(x, y) {
                    break;
                }
                cb(x, y); // 左上
                cb(checker.width - x - 1, y); // 右上
                cb(x, checker.height - y - 1); // 左下
                cb(checker.width - x - 1, checker.height - y - 1); // 右下
            }
        }
    };
    match img {
        image::DynamicImage::ImageRgb8(v) => walk(Box::new(|x, y| {
            v.get_pixel_mut(x, y).0 = [255; 3];
        })),
        image::DynamicImage::ImageRgba8(v) => walk(Box::new(|x, y| {
            v.get_pixel_mut(x, y).0 = [0; 4];
        })),
        image::DynamicImage::ImageRgb16(v) => walk(Box::new(|x, y| {
            v.get_pixel_mut(x, y).0 = [255; 3];
        })),
        image::DynamicImage::ImageRgba16(v) => walk(Box::new(|x, y| {
            v.get_pixel_mut(x, y).0 = [0; 4];
        })),
        image::DynamicImage::ImageRgb32F(v) => walk(Box::new(|x, y| {
            v.get_pixel_mut(x, y).0 = [1.0; 3];
        })),
        image::DynamicImage::ImageRgba32F(v) => walk(Box::new(|x, y| {
            v.get_pixel_mut(x, y).0 = [0.0; 4];
        })),
        image::DynamicImage::ImageLuma8(_) => {}   // TODO
        image::DynamicImage::ImageLumaA8(_) => {}  // TODO
        image::DynamicImage::ImageLuma16(_) => {}  // TODO
        image::DynamicImage::ImageLumaA16(_) => {} // TODO
        _ => {}
    }
}
impl<'a> Rounded<'a> {
    pub fn new(img: &image::DynamicImage, radius: u32) -> Rounded<'_> {
        Rounded {
            origin: img,
            checker: Checker {
                width: img.width(),
                height: img.height(),
                radius,
            },
        }
    }
}
impl Checker {
    pub fn contains(&self, x: u32, y: u32) -> bool {
        let radius = self.radius;
        let w = self.width;
        let h = self.height;
        if x >= radius && x <= w - radius {
            return true;
        }
        if y >= radius && y <= h - radius {
            return true;
        }
        let squre = radius * radius;
        let positions = [
            (radius, radius),
            (w - radius, radius),
            (radius, h - radius),
            (w - radius, h - radius),
        ];
        for pos in positions {
            if dist(pos, (x, y)) <= squre {
                return true;
            }
        }
        false
    }
}
impl<'a> image::GenericImageView for Rounded<'a> {
    type Pixel = image::Rgba<u8>;
    fn get_pixel(&self, x: u32, y: u32) -> Self::Pixel {
        if self.checker.contains(x, y) {
            self.origin.get_pixel(x, y)
        } else {
            image::Rgba([0; 4])
        }
    }
    fn dimensions(&self) -> (u32, u32) {
        self.origin.dimensions()
    }
    fn bounds(&self) -> (u32, u32, u32, u32) {
        self.origin.bounds()
    }
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
