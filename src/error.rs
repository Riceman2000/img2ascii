#[derive(Debug)]
pub enum AsciiError {
    ImageError,
    ConversionError,
    CharLookupError,
}
impl std::fmt::Display for AsciiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ImageError => write!(f, "Failed to import image"),
            Self::ConversionError => write!(f, "Failed to convert image"),
            Self::CharLookupError => write!(
                f,
                "Failed to lookup ASCII representation for a pixel/pixel group"
            ),
        }
    }
}
impl std::error::Error for AsciiError {}
