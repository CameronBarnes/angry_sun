use bevy::prelude::*;

use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PreUpdate, clear_decay.run_if(in_state(Screen::Playing)));
    app.add_systems(Update, update_decay.run_if(in_state(Screen::Playing)));
}

#[derive(Component, Debug)]
pub struct Decay {
    passed: f32,
    duration_secs: f32,
}

impl Decay {
    pub const fn new(seconds: f32) -> Self {
        Self {
            passed: 0.,
            duration_secs: seconds,
        }
    }
}

fn clear_decay(mut commands: Commands, query: Query<(Entity, &Decay), With<Decay>>) {
    for (entity, decay) in &query {
        if decay.passed >= decay.duration_secs {
            if let Some(entity_commands) = commands.get_entity(entity) {
                entity_commands.despawn_recursive();
            }
        }
    }
}

fn update_decay(time: Res<Time>, mut query: Query<&mut Decay, With<Decay>>) {
    for mut decay in &mut query {
        decay.passed += time.delta_seconds();
    }
}
