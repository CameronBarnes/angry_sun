use bevy::{
    app::{App, Update},
    core::Name,
    math::{Quat, Vec3Swizzles},
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
    time::Time,
};

use crate::screen::Screen;

use super::spawn::planets::PlanetShadow;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PreUpdate, clear_transients);
    app.add_systems(
        Update,
        (
            move_things_with_orbits,
            create_moon_shadows,
            move_moon_shadows,
        )
            .chain()
            .run_if(in_state(Screen::Playing)),
    );
}

#[derive(Component)]
pub struct Transient;

fn clear_transients(mut commands: Commands, query: Query<Entity, With<Transient>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component, Debug)]
pub struct Planet {
    pub is_moon: bool,
    pub has_magnetic_field: bool,
    pub shadow: Entity,
    pub size: f32,
    pub absorbed_power: f32,
}

#[derive(Component, Debug)]
pub struct Orbit {
    radius: f32,
    eccentricity: f32,
    degrees: f32,
    period: f32,
}

impl Orbit {
    pub const fn circle(radius: f32, period: f32) -> Self {
        Self {
            radius,
            degrees: 0.,
            eccentricity: 0.,
            period,
        }
    }

    pub fn increment_orbit(&mut self, passed: f32) {
        self.degrees += 360. * (passed / self.period);
    }

    pub const fn angle(&self) -> f32 {
        self.degrees
    }

    pub fn to_x_y(&self) -> (f32, f32) {
        if self.eccentricity > 0. {
            unimplemented!()
        } else {
            (
                self.radius * self.degrees.to_radians().cos(),
                self.radius * self.degrees.to_radians().sin(),
            )
        }
    }
}

fn create_moon_shadows(
    planet_query: Query<(&GlobalTransform, &Planet), With<Orbit>>,
    mut shadow_query: Query<&mut Transform, With<PlanetShadow>>,
) {
    for (transform, planet) in &planet_query {
        if planet.is_moon {
            let translation = transform.translation();
            let sun_angle = (planet.size / translation.x.hypot(translation.y).abs()).atan();
            let angle_around_sun = translation.xy().to_angle() + sun_angle.abs() / 2.;
            if let Ok(mut shadow) = shadow_query.get_mut(planet.shadow) {
                shadow.rotation =
                    Quat::from_rotation_z(angle_around_sun - std::f32::consts::PI / 2.);
            }
        }
    }
}

#[derive(Bundle)]
pub struct PlanetBundle<M: Material2d> {
    pub planet: Planet,
    pub name: Name,
    pub mat_mesh: MaterialMesh2dBundle<M>,
    pub orbit: Orbit,
}

fn move_things_with_orbits(
    time: Res<Time>,
    mut planet_query: Query<(&mut Transform, &mut Orbit), With<Orbit>>,
) {
    for (mut transform, mut orbit) in &mut planet_query {
        orbit.increment_orbit(time.delta_seconds());
        let (x, y) = orbit.to_x_y();
        transform.translation.x = x;
        transform.translation.y = y;
    }
}

fn move_moon_shadows(
    planet_query: Query<(&Orbit, &Planet), With<Orbit>>,
    mut shadow_query: Query<&mut Transform, (With<PlanetShadow>, Without<Orbit>)>,
) {
    for (orbit, planet) in &planet_query {
        // We have a separate function for handling moon shadows because it requires the global
        // transform and doesnt use the orbit
        if !planet.is_moon {
            let angle = orbit.angle().to_radians();
            if let Ok(mut shadow) = shadow_query.get_mut(planet.shadow) {
                shadow.rotation = Quat::from_rotation_z(angle - std::f32::consts::PI / 2.);
            }
        }
    }
}
