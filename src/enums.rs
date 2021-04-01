

// Supported missions
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mission {
    MSL,
    MARS2020
}

// Supported instruments
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Instrument {
    MslMAHLI,
    MslMastcamLeft,
    MslMastcamRight,
    M20MastcamZLeft,
    M20MastcamZRight,
    None
}

// Image data value range. Doesn't enforce actual
// value data types in the structs
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ImageMode {
    U8BIT,
    U12BIT,
    U16BIT
}



impl ImageMode {

    pub fn maxvalue(mode:ImageMode) -> f32 {
        match mode {
            ImageMode::U8BIT => 255.0,
            ImageMode::U12BIT => 2033.0,
            ImageMode::U16BIT => 65535.0
        }
    }
}