pub struct BytesConvert;
impl BytesConvert {
    pub fn from_kb(x: f64) -> f64 {
        x * 1024.0
    }

    pub fn from_mb(x: f64) -> f64 {
        x * 1048576.0
    }
}
