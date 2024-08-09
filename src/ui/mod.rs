//! Reusable UI widgets & theming.

// Unused utilities and re-exports may trigger these lints undesirably.
#![allow(dead_code, unused_imports)]

pub mod interaction;
pub mod multi_progress_bar;
pub mod palette;
pub mod planet_ui;
pub mod resource_ui;
mod widgets;

pub mod prelude {
    pub use super::{
        interaction::{InteractionPalette, InteractionQuery},
        palette as ui_palette,
        widgets::{Containers as _, Widgets as _},
    };
}

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        interaction::plugin,
        planet_ui::plugin,
        resource_ui::plugin,
        multi_progress_bar::plugin,
    ));
}
