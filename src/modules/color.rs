#[derive(Debug, Clone, Copy)]
pub enum WallColor {
    Red,
    Green,
    Blue,
    Purple,
}

impl WallColor {
    /// Converts an integer map value to a `WallColor` variant.
    ///
    /// # Errors
    /// Returns `None` if the integer doesn't match a known color variant.
    pub fn from_val(val: i32) -> Option<Self> {
        match val {
            1 => Some(WallColor::Red),
            2 => Some(WallColor::Green),
            3 => Some(WallColor::Blue),
            4 => Some(WallColor::Purple),
            _ => None,
        }
    }

    /// Retrieves the ARGB8888 color corresponding to the variant.
    pub fn to_u32(self) -> u32 {
        match self {
            WallColor::Red => 0xFF0000FF,
            WallColor::Green => 0xFF00FF00,
            WallColor::Blue => 0xFFFF0000,
            WallColor::Purple => 0xFFFF00FF,
        }
    }
}

/// Adjusts the brightness of a color in ARGB8888 format.
///
/// # Parameters
/// - `color`: 0xAARRGGBB color format
/// - `factor`: A byte between 0x00 (dark) and 0xFF (no change).
///
/// # Returns
/// - A new color with the brightness applied.
pub fn dim_color(color: u32, factor: u8) -> u32 {
    let br = ((color & 0xFF00FF) * factor as u32) >> 8;
    let g = ((color & 0x00FF00) * factor as u32) >> 8;
    0xFF000000 | (br & 0xFF00FF) | (g & 0x00FF00)
}
