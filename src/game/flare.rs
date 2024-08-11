use std::{
    f32::consts::PI,
    ops::{Div, Mul, Sub},
};

use bevy::{
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
};
use rand::{thread_rng, Rng};

use crate::{
    game::{assets::SfxKey, audio::sfx::PlaySfx, camera::ScaleWithZoom},
    screen::Screen,
};

use super::{decay::Decay, planets::Planet, spawn::planets::LAST_PLANET_DISTANCE};

#[derive(Event, Debug)]
pub struct SpawnFlare {
    pub power: f32,
    pub size: f32,
}

#[derive(Component, Debug)]
pub struct Velocity(Vec2);

#[derive(Component, Debug)]
pub struct Flare(f32);

#[derive(Bundle)]
pub struct FlareBundle<M: Material2d> {
    flare: Flare,
    mat_mesh: MaterialMesh2dBundle<M>,
    velocity: Velocity,
    decay: Decay,
}

#[derive(Resource)]
struct FlareResources(Handle<Mesh>, Handle<ColorMaterial>);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, build_flare_mesh);
    app.observe(spawn_flare);
    app.add_systems(Update, update_flares.run_if(in_state(Screen::Playing)));
}

fn build_flare_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Circle::new(50.));
    let color = materials.add(
        Color::srgb((253. / 255.) * 20., (184. / 255.) * 20., (19. / 255.) * 20.).with_alpha(0.4),
    );
    commands.insert_resource(FlareResources(mesh, color));
}

fn spawn_flare(
    trigger: Trigger<SpawnFlare>,
    resources: Res<FlareResources>,
    mut commands: Commands,
) {
    static NUMBER_OF_FLARES: usize = 5_000;
    static FLARE_SPEED: f32 = 2_000.;
    static NORMAL_WIDTH: f32 = 0.5;

    // Pick an angle
    let primary_angle = f32::from(thread_rng().gen_range(0..359_u16));
    let width = trigger.event().size * NORMAL_WIDTH;

    for _ in 0..NUMBER_OF_FLARES {
        let angle = width
            .mul_add(
                f32::from(thread_rng().gen_range(1..100_u8)).div(100.),
                primary_angle,
            )
            .to_radians();

        let speed_mult = f32::from(thread_rng().gen_range(800..1200_u16)).div(1000.);
        let flare_speed = speed_mult * FLARE_SPEED;

        commands.spawn((
            FlareBundle {
                flare: Flare(trigger.event().power),
                mat_mesh: MaterialMesh2dBundle {
                    mesh: bevy::sprite::Mesh2dHandle(resources.0.clone()),
                    material: resources.1.clone(),
                    transform: Transform::from_xyz(0., 0., 2.),
                    ..Default::default()
                },
                velocity: Velocity(Vec2::new(
                    flare_speed * angle.cos(),
                    flare_speed * angle.sin(),
                )),
                decay: Decay::new(90. / speed_mult),
            },
            StateScoped(Screen::Playing),
            ScaleWithZoom { ratio: 0.1 },
        ));
    }

    let mut speed_mod = f32::from(thread_rng().gen_range(0..100_u8)).div(100.);
    if thread_rng().gen_bool(0.7) {
        speed_mod *= -1.;
    }
    let speed = speed_mod.mul_add(0.5, 1.0);
    commands.trigger(PlaySfx::KeyVolSpeed(
        SfxKey::Thunder,
        0.05 * trigger.event().power,
        speed,
    ));
}

fn update_flares(
    time: Res<Time>,
    mut commands: Commands,
    mut flare_query: Query<(&Flare, &mut Velocity, Entity, &mut Transform), With<Flare>>,
    mut planet_query: Query<(&mut Planet, &GlobalTransform), (With<Planet>, Without<Flare>)>,
) {
    for (flare, mut velocity, entity, mut transform) in &mut flare_query {
        if transform.translation.length() > *LAST_PLANET_DISTANCE * 1.25 {
            if let Some(entity_commands) = commands.get_entity(entity) {
                entity_commands.despawn_recursive();
            }
            continue;
        }
        for (mut planet, planet_transform) in &mut planet_query {
            let distance = transform
                .translation
                .xy()
                .distance(planet_transform.translation().xy());
            // Handle collision with the planet
            if distance < planet.size * 2. {
                let direction = planet_transform
                    .translation()
                    .xy()
                    .sub(transform.translation.xy())
                    .normalize();
                if distance < planet.size {
                    planet.absorbed_power += flare.0;
                    if let Some(entity_commands) = commands.get_entity(entity) {
                        entity_commands.despawn_recursive();
                    }
                    continue;
                } else if distance < planet.size * 1.1 {
                    velocity.0 = velocity
                        .0
                        .lerp(direction.mul(velocity.0.length() * 1.1), 0.2);
                    continue;
                } else if planet.has_magnetic_field {
                    let mut perp = direction.perp();
                    // We need to makee sure that it doesnt just suddenly make the flare go
                    // backwards
                    if direction.angle_between(velocity.0) < 0. {
                        perp = Vec2::from_angle(perp.to_angle() + PI);
                    }
                    perp = perp.mul(velocity.0.length() * 1.1);
                    velocity.0 = velocity.0.lerp(perp, 0.15);
                    // Planets with magnetic fields still absorb solar flare energy, just at a
                    // reduced rate
                    planet.absorbed_power += flare.0 / 20.;
                }
                let force = (planet.size * 50.) / distance.sub(planet.size).div(30.).powi(2);
                velocity.0 += direction.mul(force).mul(time.delta_seconds());
            }
            transform.translation += velocity.0.extend(0.).mul(time.delta_seconds());
        }
    }
}
