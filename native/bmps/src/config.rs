use std::ops::Rem;

#[derive(Clone, Debug)]
pub struct Config {
    pub source_file: String,
    pub dest_file: String,
    pub font: Option<String>,
    pub size: Size,
    pub white_bg: bool,
}

#[derive(Clone, Debug)]
pub struct Size {
    // aspect_ratio 为 true 时用于宽高比
    // aspect_ratio 为 false 时为水平方向像素数
    pub width: u32,
    // aspect_ratio 为 true 时用于宽高比
    // aspect_ratio 为 false 时为竖直方向像素数
    pub height: u32,
    // 表明 width height 是否是宽高比
    pub aspect_ratio: bool,
    // 背景图高斯模糊半径(像素数)
    pub blur_radius: u32,
    // 圆角半径(像素数)
    pub round_radius: u32,
    pub padding: f64,
    // 阴影模糊半径
    pub shadow: u32,
    // 阴影水平偏移量，同 CSS box-shadow
    pub shadow_offset_x: i32,
    // 阴影竖直偏移量，同 CSS box-shadow
    pub shadow_offset_y: i32,
}
impl Default for Size {
    fn default() -> Self {
        Size {
            width: 1920,
            height: 1080,
            aspect_ratio: false,
            blur_radius: 50,
            round_radius: 45,
            padding: 0.1,
            shadow: 40,
            shadow_offset_x: 30,
            shadow_offset_y: 30,
        }
    }
}
fn calc_gcd<T>(mut n: T, mut m: T) -> T
where
    T: Rem<Output = T> + Default + PartialEq + Copy,
{
    let zero = T::default();
    while n != zero {
        let r = m % n;
        m = n;
        n = r;
    }
    m
}
impl Size {
    pub(crate) fn calc_bg(&self, width: u32, height: u32) -> (u32, u32) {
        if !self.aspect_ratio {
            return (self.width, self.height);
        }
        let g = calc_gcd(self.width, self.height);
        // 约分背景宽高比
        let bg_width = self.width / g;
        let bg_height = self.height / g;
        // 约分原图宽高比
        let g = calc_gcd(width, height);
        let w = width / g;
        let h = height / g;
        if bg_width * h >= w * bg_height {
            let res_h = (height as f64 / (1.0 - self.padding * 2.0)) as u32;
            (bg_width * res_h / bg_height, res_h)
        } else {
            let res_w = (width as f64 / (1.0 - self.padding * 2.0)) as u32;
            (res_w, bg_height * res_w / bg_width)
        }
    }
}
