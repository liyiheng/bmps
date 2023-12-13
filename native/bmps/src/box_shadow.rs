#![allow(unused)]
use std::time::Instant;

use image::{imageops, DynamicImage, GenericImageView, Rgb, RgbImage, Rgba, RgbaImage};

// https://html.spec.whatwg.org/multipage/canvas.html#when-shadows-are-drawn
// UNSURPPORTED spread_radius: i32,
// UNSURPPORTED inset: bool,
// https://developer.mozilla.org/en-US/docs/Web/CSS/box-shadow#try_it
#[derive(Debug, Clone)]
pub struct Effects {
    offset_x: i32,
    offset_y: i32,
    blur_radius: u32,
    // rgba
    color: [u8; 4],
}

impl Default for Effects {
    fn default() -> Self {
        Effects {
            offset_x: 10,
            offset_y: 10,
            blur_radius: 5,
            color: [0, 0, 0, 255],
        }
    }
}

#[derive(Default, Debug)]
pub struct EffectBuilder {
    effects: Effects,
}
impl EffectBuilder {
    pub fn new() -> EffectBuilder {
        EffectBuilder {
            effects: Default::default(),
        }
    }
    pub fn offset(mut self, x: i32, y: i32) -> EffectBuilder {
        self.effects.offset_x = x;
        self.effects.offset_y = y;
        self
    }
    pub fn blur_radius(mut self, r: u32) -> EffectBuilder {
        self.effects.blur_radius = r;
        self
    }
    pub fn color(mut self, rgba: [u8; 4]) -> EffectBuilder {
        self.effects.color = rgba;
        self
    }
    pub fn build(self) -> Effects {
        self.effects
    }
}
impl Effects {
    fn nop(&self) -> bool {
        self.color[3] == 0 || (self.blur_radius == 0 && self.offset_x == 0 && self.offset_y == 0)
    }
    /// 生成包含阴影的图层，以及原图偏移量（ (0,0)在此图层的位置）
    /// 详见 [When shadows are drawn](https://html.spec.whatwg.org/multipage/canvas.html#when-shadows-are-drawn)
    pub fn gen_bg(&self, img: &DynamicImage) -> (RgbaImage, u32, u32) {
        let get_size = |origin: i32, offset: i32, blur: i32| -> (u32, u32) {
            let s = origin.max(origin + offset + blur) - 0.min(offset - blur);
            let x = if offset < 0 {
                -offset + blur
            } else {
                (offset - blur).min(0).abs()
            };
            (s as u32, x as u32)
        };
        let (bg_width, combine_offset_x) =
            get_size(img.width() as i32, self.offset_x, self.blur_radius as i32);
        let (bg_height, combine_offset_y) =
            get_size(img.height() as i32, self.offset_y, self.blur_radius as i32);
        let mut b = RgbaImage::from_pixel(
            bg_width,
            bg_height,
            Rgba({
                let mut p = self.color;
                p[3] = 0;
                p
            }),
        );
        if self.nop() {
            return (b, combine_offset_x, combine_offset_y);
        }
        for x in 0..img.width() {
            for y in 0..img.height() {
                let p = img.get_pixel(x, y);
                let bx = if self.offset_x < 0 {
                    x + combine_offset_x - (-self.offset_x as u32)
                } else {
                    x + combine_offset_x + (self.offset_x as u32)
                };
                let by = if self.offset_y < 0 {
                    y + combine_offset_y - (-self.offset_y as u32)
                } else {
                    y + combine_offset_y + (self.offset_y as u32)
                };
                let tmp = b.get_pixel_mut(bx, by);
                tmp.0[3] = p[3];
            }
        }

        let tt0 = Instant::now();
        let mut b = imageops::blur(&b, self.blur_radius as f32 / 2.0);
        log::info!("imageops::blur cost {}ms", tt0.elapsed().as_millis());
        let alpha = self.color[3] as f64 / 255.0;
        let full = self.color[3] == 255;
        if !full {
            b.pixels_mut().for_each(|p| {
                let v = p.0[3] as f64 / 255.0;
                let v = alpha * v * 255.0;
                p.0[3] = v.ceil() as u8;
            });
        }
        (b, combine_offset_x, combine_offset_y)
    }
}
