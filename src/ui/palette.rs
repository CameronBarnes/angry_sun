use std::sync::LazyLock;

use bevy::prelude::*;

use super::prelude::InteractionPalette;

pub const BUTTON_HOVERED_BACKGROUND: Color = Color::srgb(0.186, 0.328, 0.573);
pub const BUTTON_PRESSED_BACKGROUND: Color = Color::srgb(0.286, 0.478, 0.773);

pub const BUTTON_TEXT: Color = Color::srgb(0.925, 0.925, 0.925);
pub const LABEL_TEXT: Color = Color::srgb(0.867, 0.827, 0.412);
pub const HEADER_TEXT: Color = Color::srgb(0.867, 0.827, 0.412);

pub const NODE_BACKGROUND: Color = Color::srgb(0.286, 0.478, 0.773);

pub const BUTTON_PALETTE: InteractionPalette = InteractionPalette {
    none: NODE_BACKGROUND,
    hovered: BUTTON_HOVERED_BACKGROUND,
    pressed: BUTTON_PRESSED_BACKGROUND,
};

pub static BUTTON_PALETTE_DISABLED: LazyLock<InteractionPalette> =
    LazyLock::new(|| InteractionPalette {
        none: NODE_BACKGROUND.with_luminance(0.2),
        hovered: BUTTON_HOVERED_BACKGROUND.with_luminance(0.15),
        pressed: BUTTON_PRESSED_BACKGROUND.with_luminance(0.15),
    });
