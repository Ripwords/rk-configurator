use crate::models::Mode;
use serde::{Deserialize, Serialize};

/// RGB lighting modes (for RGB keyboards)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum RgbMode {
    Custom = 0,
    NeonStream = 1,
    RipplesShining = 2,
    RotatingWindmill = 3,
    SineWave = 4,
    RainbowRoulette = 5,
    StarsTwinkle = 6,
    LayerUponLayer = 7,
    RichAndHonored = 8,
    MarqueeEffect = 9,
    RotatingStorm = 10,
    SerpentineHorseRace = 11,
    RetroSnake = 12,
    DiagonalTransformation = 13,
    Ambilight = 14,
    Streamer = 15,
    Steady = 16,
    Breathing = 17,
    Neon = 18,
    ShadowDisappear = 19,
    FlashAway = 20,
}

/// Non-RGB lighting modes (for single-color keyboards)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum SingleColorMode {
    Steady = 1,
    Custom = 2,
    Breathing = 3,
    PressAndDestroy = 4,
    NeonStream = 5,
    Streamer = 6,
    Ambilight = 7,
    DripplingRipples = 8,
    BrilliantPoint = 9,
    FlashAway = 10,
    ShadowDisappear = 11,
    RipplesShining = 12,
    RichAndHonored = 13,
    MarqueeEffect = 14,
    RotatingStorm = 15,
    SerpentineHorseRace = 16,
    StarsTwinkle = 17,
    RetroSnake = 18,
    DiagonalTransformation = 19,
    SineWave = 20,
}

/// Get all RGB lighting modes
pub fn get_rgb_modes() -> Vec<Mode> {
    vec![
        Mode {
            name: "Custom".to_string(),
            mode_bit: RgbMode::Custom as u8,
        },
        Mode {
            name: "Ambilight".to_string(),
            mode_bit: RgbMode::Ambilight as u8,
        },
        Mode {
            name: "Breathing".to_string(),
            mode_bit: RgbMode::Breathing as u8,
        },
        Mode {
            name: "Diagonal Transformation".to_string(),
            mode_bit: RgbMode::DiagonalTransformation as u8,
        },
        Mode {
            name: "Flash Away".to_string(),
            mode_bit: RgbMode::FlashAway as u8,
        },
        Mode {
            name: "Layer Upon Layer".to_string(),
            mode_bit: RgbMode::LayerUponLayer as u8,
        },
        Mode {
            name: "Marquee Effect".to_string(),
            mode_bit: RgbMode::MarqueeEffect as u8,
        },
        Mode {
            name: "Neon".to_string(),
            mode_bit: RgbMode::Neon as u8,
        },
        Mode {
            name: "Neon Stream".to_string(),
            mode_bit: RgbMode::NeonStream as u8,
        },
        Mode {
            name: "Rainbow Roulette".to_string(),
            mode_bit: RgbMode::RainbowRoulette as u8,
        },
        Mode {
            name: "Retro Snake".to_string(),
            mode_bit: RgbMode::RetroSnake as u8,
        },
        Mode {
            name: "Rich And Honored".to_string(),
            mode_bit: RgbMode::RichAndHonored as u8,
        },
        Mode {
            name: "Ripples Shining".to_string(),
            mode_bit: RgbMode::RipplesShining as u8,
        },
        Mode {
            name: "Rotating Storm".to_string(),
            mode_bit: RgbMode::RotatingStorm as u8,
        },
        Mode {
            name: "Rotating Windmill".to_string(),
            mode_bit: RgbMode::RotatingWindmill as u8,
        },
        Mode {
            name: "Serpentine Horse Race".to_string(),
            mode_bit: RgbMode::SerpentineHorseRace as u8,
        },
        Mode {
            name: "Shadow Disappear".to_string(),
            mode_bit: RgbMode::ShadowDisappear as u8,
        },
        Mode {
            name: "Sine Wave".to_string(),
            mode_bit: RgbMode::SineWave as u8,
        },
        Mode {
            name: "Stars Twinkle".to_string(),
            mode_bit: RgbMode::StarsTwinkle as u8,
        },
        Mode {
            name: "Steady".to_string(),
            mode_bit: RgbMode::Steady as u8,
        },
        Mode {
            name: "Streamer".to_string(),
            mode_bit: RgbMode::Streamer as u8,
        },
    ]
}

/// Get all single-color lighting modes
pub fn get_single_color_modes() -> Vec<Mode> {
    vec![
        Mode {
            name: "Steady".to_string(),
            mode_bit: SingleColorMode::Steady as u8,
        },
        Mode {
            name: "Custom".to_string(),
            mode_bit: SingleColorMode::Custom as u8,
        },
        Mode {
            name: "Breathing".to_string(),
            mode_bit: SingleColorMode::Breathing as u8,
        },
        Mode {
            name: "Press And Destroy".to_string(),
            mode_bit: SingleColorMode::PressAndDestroy as u8,
        },
        Mode {
            name: "Neon Stream".to_string(),
            mode_bit: SingleColorMode::NeonStream as u8,
        },
        Mode {
            name: "Streamer".to_string(),
            mode_bit: SingleColorMode::Streamer as u8,
        },
        Mode {
            name: "Ambilight".to_string(),
            mode_bit: SingleColorMode::Ambilight as u8,
        },
        Mode {
            name: "Drippling Ripples".to_string(),
            mode_bit: SingleColorMode::DripplingRipples as u8,
        },
        Mode {
            name: "Brilliant Point".to_string(),
            mode_bit: SingleColorMode::BrilliantPoint as u8,
        },
        Mode {
            name: "Flash Away".to_string(),
            mode_bit: SingleColorMode::FlashAway as u8,
        },
        Mode {
            name: "Shadow Disappear".to_string(),
            mode_bit: SingleColorMode::ShadowDisappear as u8,
        },
        Mode {
            name: "Ripples Shining".to_string(),
            mode_bit: SingleColorMode::RipplesShining as u8,
        },
        Mode {
            name: "Rich And Honored".to_string(),
            mode_bit: SingleColorMode::RichAndHonored as u8,
        },
        Mode {
            name: "Marquee Effect".to_string(),
            mode_bit: SingleColorMode::MarqueeEffect as u8,
        },
        Mode {
            name: "Rotating Storm".to_string(),
            mode_bit: SingleColorMode::RotatingStorm as u8,
        },
        Mode {
            name: "Serpentine Horse Race".to_string(),
            mode_bit: SingleColorMode::SerpentineHorseRace as u8,
        },
        Mode {
            name: "Stars Twinkle".to_string(),
            mode_bit: SingleColorMode::StarsTwinkle as u8,
        },
        Mode {
            name: "Retro Snake".to_string(),
            mode_bit: SingleColorMode::RetroSnake as u8,
        },
        Mode {
            name: "Diagonal Transformation".to_string(),
            mode_bit: SingleColorMode::DiagonalTransformation as u8,
        },
        Mode {
            name: "Sine Wave".to_string(),
            mode_bit: SingleColorMode::SineWave as u8,
        },
    ]
}

/// Check if a mode bit represents a custom mode
pub fn is_custom_mode(mode_bit: u8, is_rgb: bool) -> bool {
    if is_rgb {
        mode_bit == RgbMode::Custom as u8
    } else {
        mode_bit == SingleColorMode::Custom as u8
    }
}
