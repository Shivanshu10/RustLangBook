//! # Art
//! 
//! A lib for modelling artistic concepts

// export so people have to not know internal module structure to access PrimaryColor
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;


pub mod kinds {
    /// The primary color according to RGB model
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue
    }

    // The secondary color according to RYB
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple
    }
}

pub mod utils {
    use crate::kinds::*;

    /// combines two primary color in equal amt to create secondary color
    pub fn mix(x1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
        // ANCHOR_END: here
        return SecondaryColor::Orange;
        // ANCHOR: here
    }
}