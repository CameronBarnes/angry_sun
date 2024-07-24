use bevy::{
    app::{App, Update},
    asset::Assets,
    color::{Color, Luminance},
    core::Name,
    math::{Quat, Vec2, Vec3, Vec3Swizzles},
    prelude::{
        in_state, Bundle, Commands, Component, DespawnRecursiveExt, Entity, GlobalTransform,
        IntoSystemConfigs, Mesh, Query, Res, ResMut, StateScoped, Transform, Triangle2d, With,
        Without,
    },
    sprite::{ColorMaterial, Material2d, MaterialMesh2dBundle, Mesh2dHandle},
    time::Time,
};

use crate::screen::Screen;

use super::spawn::planets::{scale, PlanetShadow, PlanetShadowCone, RADIUS_SCALE};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (clear_transients, move_planets, create_moon_shadows)
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
    pub triangle: Option<Entity>,
    pub shadow: Entity,
    pub size: f32,
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
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    planet_query: Query<(&GlobalTransform, &Planet), With<Orbit>>,
    mut shadow_query: Query<&mut Transform, With<PlanetShadow>>,
) {
    for (transform, planet) in &planet_query {
        if planet.triangle.is_none() {
            let translation = transform.translation();
            let sun_angle = (planet.size / translation.x.hypot(translation.y).abs()).atan();
            let scaled_distance = scale(5_000_000_000. * RADIUS_SCALE);
            let first_point = (
                scaled_distance * sun_angle.abs().cos(),
                scaled_distance * sun_angle.abs().sin(),
            );
            let second_point = (
                scaled_distance * (sun_angle.abs() * -1.).cos(),
                scaled_distance * (sun_angle.abs() * -1.).sin(),
            );
            let angle_around_sun = translation.xy().to_angle() + sun_angle.abs() / 2.;
            if let Ok(mut shadow) = shadow_query.get_mut(planet.shadow) {
                shadow.rotation =
                    Quat::from_rotation_z(angle_around_sun - std::f32::consts::PI / 2.);
            }
            commands.spawn((
                PlanetShadowCone,
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Triangle2d::new(
                        Vec2::splat(0.),
                        Vec2::new(first_point.0, first_point.1),
                        Vec2::new(second_point.0, second_point.1),
                    ))),
                    material: materials.add(Color::WHITE.darker(0.9)),
                    transform: Transform::from_rotation(Quat::from_rotation_z(angle_around_sun))
                        .with_translation(Vec3::new(0., 0., -3.)),
                    ..Default::default()
                },
                StateScoped(Screen::Playing),
                Transient,
            ));
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
    mut planet_query: Query<(&mut Transform, &mut Orbit, &Planet), With<Orbit>>,
    mut shadow_query: Query<&mut Transform, (With<PlanetShadow>, Without<Orbit>)>,
    mut triangle_query: Query<
        &mut Transform,
        (
            With<PlanetShadowCone>,
            Without<Orbit>,
            Without<PlanetShadow>,
        ),
    >,
) {
    if planet_query.is_empty() {
        return;
    }

    for (mut transform, mut orbit, planet) in &mut planet_query {
        orbit.increment_orbit(time.delta_seconds());
        let (x, y) = orbit.to_x_y();
        transform.translation.x = x;
        transform.translation.y = y;

        if let Some(id) = planet.triangle {
            let angle = orbit.angle().to_radians();
            if let Ok(mut triangle) = triangle_query.get_mut(id) {
                triangle.rotation = Quat::from_rotation_z(angle);
            }
            if let Ok(mut shadow) = shadow_query.get_mut(planet.shadow) {
                shadow.rotation = Quat::from_rotation_z(angle - std::f32::consts::PI / 2.);
            }
        }
    }
}
