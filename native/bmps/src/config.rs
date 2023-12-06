#[derive(Clone, Debug)]
pub struct Config {
    pub source_file: String,
    pub dest_file: String,
    pub font: Option<String>,
    pub size: Size,
}

#[derive(Clone, Debug)]
pub struct Size {
    pub width: u32,
    pub height: u32,
    pub blur_radius: u32,
    pub round_radius: u32,
    pub padding: f64,
    pub shadow: u32,
}
impl Default for Size {
    fn default() -> Self {
        Size {
            width: 1920,
            height: 1080,
            blur_radius: 50,
            round_radius: 45,
            padding: 0.1,
            shadow: 40,
        }
    }
}
