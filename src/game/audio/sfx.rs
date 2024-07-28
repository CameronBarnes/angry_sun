use bevy::{audio::{PlaybackMode, Volume}, prelude::*};

use crate::game::assets::{HandleMap, SfxKey};

pub(super) fn plugin(app: &mut App) {
    app.observe(play_sfx);
}

fn play_sfx(
    trigger: Trigger<PlaySfx>,
    mut commands: Commands,
    sfx_handles: Res<HandleMap<SfxKey>>,
) {
    let (sfx_key, volume) = match trigger.event() {
        PlaySfx::Key(key) => (*key, 1.0),
        PlaySfx::KeyVol(key, volume) => (*key, *volume),
    };
    commands.spawn(AudioSourceBundle {
        source: sfx_handles[&sfx_key].clone_weak(),
        settings: PlaybackSettings {
            mode: PlaybackMode::Despawn,
            volume: Volume::new(volume),
            ..default()
        },
    });
}

/// Trigger this event to play a single sound effect.
#[derive(Event)]
pub enum PlaySfx {
    Key(SfxKey),
    KeyVol(SfxKey, f32),
}
