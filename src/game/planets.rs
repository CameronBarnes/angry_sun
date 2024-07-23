use bevy::{
    app::{App, Update},
    core::Name,
    prelude::{in_state, Bundle, Component, IntoSystemConfigs, Query, Res, Transform, With},
    sprite::{Material2d, MaterialMesh2dBundle},
    time::Time,
};

use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (move_planets).run_if(in_state(Screen::Playing)));
}

#[derive(Component, Debug)]
pub struct Planet;

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

#[derive(Bundle)]
pub struct PlanetBundle<M: Material2d> {
    pub planet: Planet,
    pub name: Name,
    pub mat_mesh: MaterialMesh2dBundle<M>,
    pub orbit: Orbit,
}

fn move_planets(
    time: Res<Time>,
    mut planet_query: Query<(&mut Transform, &mut Orbit), With<Orbit>>,
) {
    if planet_query.is_empty() {
        return;
    }

    for (mut transform, mut orbit) in &mut planet_query {
        orbit.increment_orbit(time.delta_seconds());
        let (x, y) = orbit.to_x_y();
        transform.translation.x = x;
        transform.translation.y = y;
    }
}
