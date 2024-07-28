//! Game mechanics and content.

use bevy::prelude::*;

//mod animation;
pub mod assets;
pub mod audio;
pub mod camera;
pub mod decay;
pub mod flare;
pub mod highlight;
pub mod planets;
pub mod spawn;
pub mod sun;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        //animation::plugin,
        audio::plugin,
        assets::plugin,
        spawn::plugin,
        planets::plugin,
        sun::plugin,
        camera::plugin,
        flare::plugin,
        decay::plugin,
        highlight::plugin,
    ));
}
