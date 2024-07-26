use std::ops::{Div, Mul, Sub};

use bevy::{
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
};
use rand::{thread_rng, Rng};

use crate::{game::scale::ScaleWithZoom, screen::Screen};

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

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_flare);
    app.add_systems(Update, update_flares);
}

fn spawn_flare(
    trigger: Trigger<SpawnFlare>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    static NUMBER_OF_FLARES: usize = 3_000;
    static FLARE_SPEED: f32 = 1_000.;
    static NORMAL_WIDTH: f32 = 0.5;

    info! {"Spawing flare"};
    // Pick an angle
    let primary_angle = f32::from(thread_rng().gen_range(0..359_u16));
    let width = trigger.event().size * NORMAL_WIDTH;

    let mesh = meshes.add(Circle::new(50.));
    let color = materials.add(
        Color::srgb((253. / 255.) * 20., (184. / 255.) * 20., (19. / 255.) * 20.).with_alpha(0.4),
    );

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
                    mesh: bevy::sprite::Mesh2dHandle(mesh.clone()),
                    material: color.clone(),
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
}

fn update_flares(
    time: Res<Time>,
    mut commands: Commands,
    mut flare_query: Query<(&Flare, &mut Velocity, Entity, &mut Transform), With<Flare>>,
    mut planet_query: Query<(&mut Planet, &GlobalTransform), (With<Planet>, Without<Flare>)>,
) {
    for (flare, mut velocity, entity, mut transform) in &mut flare_query {
        if transform.translation.length() > *LAST_PLANET_DISTANCE * 1.1 {
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
            if distance < planet.size {
                planet.absorbed_power += flare.0;
                if let Some(entity_commands) = commands.get_entity(entity) {
                    entity_commands.despawn_recursive();
                }
                continue;
            } else if distance < planet.size * 3. {
                let direction = planet_transform
                    .translation()
                    .xy()
                    .sub(transform.translation.xy())
                    .normalize();
                if distance < planet.size * 1.1 {
                    velocity.0 = velocity
                        .0
                        .lerp(direction.mul(velocity.0.length() * 1.1), 0.2);
                    continue;
                }
                let force = (planet.size * 30.) / distance.sub(planet.size).div(20.).powi(2);
                info! {"Gravity: {force}"};
                velocity.0 += direction.mul(force);
            }
            transform.translation += velocity.0.extend(0.).mul(time.delta_seconds());
        }
    }
}
