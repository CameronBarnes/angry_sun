//! Game mechanics and content.

use bevy::prelude::*;

//mod animation;
pub mod assets;
pub mod audio;
pub mod planets;
pub mod sun;
pub mod spawn;
pub mod scale;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        //animation::plugin,
        audio::plugin,
        assets::plugin,
        spawn::plugin,
        planets::plugin,
        sun::plugin,
        scale::plugin,
    ));
}
