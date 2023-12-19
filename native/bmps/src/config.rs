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
