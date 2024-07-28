use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

use crate::game::assets::{HandleMap, SfxKey};

pub(super) fn plugin(app: &mut App) {
    app.observe(play_sfx);
}

fn play_sfx(
    trigger: Trigger<PlaySfx>,
    mut commands: Commands,
    sfx_handles: Res<HandleMap<SfxKey>>,
) {
    let (sfx_key, volume, speed) = match trigger.event() {
        PlaySfx::Key(key) => (*key, 1.0, 1.0),
        PlaySfx::KeyVolSpeed(key, volume, speed) => (*key, *volume, *speed),
    };
    commands.spawn(AudioSourceBundle {
        source: sfx_handles[&sfx_key].clone_weak(),
        settings: PlaybackSettings {
            mode: PlaybackMode::Despawn,
            volume: Volume::new(volume),
            speed,
            ..default()
        },
    });
}

/// Trigger this event to play a single sound effect.
#[derive(Event)]
pub enum PlaySfx {
    Key(SfxKey),
    KeyVolSpeed(SfxKey, f32, f32),
}
