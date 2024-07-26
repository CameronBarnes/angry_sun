use std::ops::Mul;

use bevy::prelude::*;

use crate::screen::Screen;

#[derive(Component, Debug)]
pub struct ScaleWithZoom {
    pub ratio: f32,
}

impl Default for ScaleWithZoom {
    fn default() -> Self {
        Self { ratio: 1. }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, scale_with_zoom.run_if(in_state(Screen::Playing)));
}

fn scale_with_zoom(
    camera_projection: Query<&OrthographicProjection, With<OrthographicProjection>>,
    mut scaled_objects: Query<(&mut Transform, &ScaleWithZoom), With<ScaleWithZoom>>,
) {
    if let Ok(projection) = camera_projection.get_single() {
        if projection.scale > 15. {
            for (mut transform, scale) in &mut scaled_objects {
                transform.scale =
                    Vec3::splat((projection.scale / 30.).mul(scale.ratio).clamp(1., 5.));
            }
        } else {
            for (mut transform, _scale) in &mut scaled_objects {
                transform.scale = Vec3::ONE;
            }
        }
    }
}
