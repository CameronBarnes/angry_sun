use std::ops::Mul;

use bevy::prelude::*;
use bevy_mod_picking::prelude::PickSelection;

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

#[derive(Debug, Component)]
pub struct FinishZoom {
    finished: bool,
    target: f32,
}

impl Default for FinishZoom {
    fn default() -> Self {
        Self {
            finished: false,
            target: 15.,
        }
    }
}

impl FinishZoom {
    pub const fn new_with_target(target: f32) -> Self {
        Self {
            finished: false,
            target,
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (scale_with_zoom, camera_follow).run_if(in_state(Screen::Playing)),
    );
}

fn scale_with_zoom(
    camera_projection: Query<&OrthographicProjection, With<OrthographicProjection>>,
    mut scaled_objects: Query<(&mut Transform, &ScaleWithZoom), With<ScaleWithZoom>>,
) {
    if let Ok(projection) = camera_projection.get_single() {
        if projection.scale > 15. {
            for (mut transform, scale) in &mut scaled_objects {
                transform.scale =
                    Vec3::splat((projection.scale / 50.).mul(scale.ratio).clamp(1., 5.));
            }
        } else {
            for (mut transform, _scale) in &mut scaled_objects {
                transform.scale = Vec3::ONE;
            }
        }
    }
}

fn camera_follow(
    mut query_selectables: Query<
        (&GlobalTransform, &PickSelection, &mut FinishZoom),
        With<PickSelection>,
    >,
    mut camera_query: Query<
        (&mut Transform, &mut OrthographicProjection),
        (With<Camera>, Without<PickSelection>),
    >,
) {
    for (transform, selected, mut finish_zoom) in &mut query_selectables {
        if selected.is_selected {
            // Make the camera follow that object, zoom to the max unscaled size
            if let Ok((mut camera_transform, mut projection)) = camera_query.get_single_mut() {
                camera_transform.translation = camera_transform
                    .translation
                    .lerp(transform.translation().xy().extend(0.), 0.1);
                if !finish_zoom.finished && projection.scale > finish_zoom.target + 1.
                    || projection.scale < 1.
                {
                    projection.scale = projection.scale.lerp(finish_zoom.target, 0.2);
                } else {
                    finish_zoom.finished = true;
                }
            }
        }
    }
}
