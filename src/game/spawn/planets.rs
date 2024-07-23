use bevy::{
    app::App,
    asset::{Assets, Handle},
    color::{Color, Luminance},
    core::Name,
    prelude::*,
    sprite::{ColorMaterial, Material2d, MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{
    game::planets::{Orbit, Planet, PlanetBundle},
    screen::Screen,
};

#[derive(Event, Debug)]
pub struct SpawnSolarSystem;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_solar_system);
}

#[derive(Component, Debug)]
pub struct PlanetShadow;

static MESH_RESOLUTION: usize = 100;

static PLANET_SCALE: f32 = 10.;
pub static RADIUS_SCALE: f32 = 0.125;
static MOON_SCALE: f32 = 5.;
static MOON_RADIUS_SCALE: f32 = 1.5;

// FIXME: Fix the too many lines issue by breaking this up
#[allow(clippy::too_many_lines)]
fn spawn_solar_system(
    _trigger: Trigger<SpawnSolarSystem>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let circle_color = materials.add(Color::WHITE.darker(0.75));

    commands
        .spawn((
            Name::new("Sun"),
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(
                    meshes.add(
                        Circle::new(scale(1_400_000.))
                            .mesh()
                            .resolution(MESH_RESOLUTION)
                            .build(),
                    ),
                ),
                material: materials.add(Color::srgb(253. / 255., 184. / 255., 19. / 255.)),
                ..Default::default()
            },
        ))
        .insert(StateScoped(Screen::Playing));

    spawn_planet(
        &mut commands,
        &mut meshes,
        &mut materials,
        Name::new("Mercury"),
        scale(57_000_000. * RADIUS_SCALE),
        scale(4_879. * PLANET_SCALE),
        88.,
        Color::srgb(183. / 255., 184. / 255., 185. / 255.),
        circle_color.clone(),
        vec![],
        false,
    );

    spawn_planet(
        &mut commands,
        &mut meshes,
        &mut materials,
        Name::new("Venus"),
        scale(108_000_000. * RADIUS_SCALE),
        scale(12_104. * PLANET_SCALE),
        224.7,
        Color::srgb(165. / 255., 124. / 255., 27. / 255.),
        circle_color.clone(),
        vec![],
        false,
    );

    let moon = spawn_planet(
        &mut commands,
        &mut meshes,
        &mut materials,
        Name::new("Moon"),
        scale(384_400. * MOON_RADIUS_SCALE),
        scale(3_475. * MOON_SCALE),
        27.3,
        Color::srgb(246. / 255., 241. / 255., 213. / 255.),
        circle_color.clone(),
        vec![],
        true,
    );
    spawn_planet(
        &mut commands,
        &mut meshes,
        &mut materials,
        Name::new("Earth"),
        scale(149_000_000. * RADIUS_SCALE),
        scale(12_756. * PLANET_SCALE),
        365.25,
        Color::srgb(79. / 255., 76. / 255., 176. / 255.),
        circle_color.clone(),
        vec![moon.0, moon.1],
        false,
    );

    spawn_planet(
        &mut commands,
        &mut meshes,
        &mut materials,
        Name::new("Mars"),
        scale(288_000_000. * RADIUS_SCALE),
        scale(6_790. * PLANET_SCALE),
        687.,
        Color::srgb(193. / 255., 68. / 255., 14. / 255.),
        circle_color.clone(),
        vec![],
        false,
    );

    // TODO: Add moons
    spawn_planet(
        &mut commands,
        &mut meshes,
        &mut materials,
        Name::new("Jupiter"),
        scale(780_000_000. * RADIUS_SCALE),
        scale(143_000. * PLANET_SCALE),
        4_330.6,
        Color::srgb(148. / 255., 105. / 255., 86. / 255.),
        circle_color.clone(),
        vec![],
        false,
    );

    // TODO: Add moons
    spawn_planet(
        &mut commands,
        &mut meshes,
        &mut materials,
        Name::new("Saturn"),
        scale(1_437_000_000. * RADIUS_SCALE),
        scale(120_536. * PLANET_SCALE),
        10_756.,
        Color::srgb(206. / 255., 184. / 255., 184. / 255.),
        circle_color.clone(),
        vec![],
        false,
    );

    // TODO: Add moons
    spawn_planet(
        &mut commands,
        &mut meshes,
        &mut materials,
        Name::new("Uranus"),
        scale(2_871_000_000. * RADIUS_SCALE),
        scale(51_118. * PLANET_SCALE),
        30_687.,
        Color::srgb(172. / 255., 229. / 255., 238. / 255.),
        circle_color.clone(),
        vec![],
        false,
    );

    // TODO: Add moons
    spawn_planet(
        &mut commands,
        &mut meshes,
        &mut materials,
        Name::new("Neptune"),
        scale(4_530_000_000. * RADIUS_SCALE),
        scale(49_528. * PLANET_SCALE),
        60_190.,
        Color::srgb(120. / 255., 192. / 255., 168. / 255.),
        circle_color,
        vec![],
        false,
    );
}

fn spawn_planet<A: Material2d>(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    name: Name,
    scaled_radius: f32,
    scaled_size: f32,
    orbital_period: f32,
    color: Color,
    orbit_circle: Handle<A>,
    children: Vec<Entity>,
    moon: bool,
) -> (Entity, Entity) {
    let (border_width, triangle_id) = if moon {
        (2., None)
    } else {
        let sun_angle = (scaled_size / scaled_radius).atan();
        let scaled_distance = scale(5_000_000_000. * RADIUS_SCALE);
        let first_point = (
            scaled_distance * sun_angle.abs().cos(),
            scaled_distance * sun_angle.abs().sin(),
        );
        let second_point = (
            scaled_distance * (sun_angle.abs() * -1.).cos(),
            scaled_distance * (sun_angle.abs() * -1.).sin(),
        );
        let id = commands
            .spawn((
                PlanetShadow,
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Triangle2d::new(
                        Vec2::splat(0.),
                        Vec2::new(first_point.0, first_point.1),
                        Vec2::new(second_point.0, second_point.1),
                    ))),
                    material: orbit_circle.clone(),
                    transform: Transform::from_xyz(0., 0., -3.),
                    ..Default::default()
                },
                StateScoped(Screen::Playing),
            ))
            .id();

        (5., Some(id))
    };

    let orbit_id = commands
        .spawn((
            Name::new("Mercury - Orbit Circle"),
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(
                    meshes.add(
                        Annulus::new(scaled_radius - border_width, scaled_radius + border_width)
                            .mesh()
                            .resolution(MESH_RESOLUTION)
                            .build(),
                    ),
                ),
                material: orbit_circle,
                transform: Transform::from_xyz(0., 0., -2.),
                ..Default::default()
            },
            StateScoped(Screen::Playing),
        ))
        .id();
    let mut planet = commands.spawn(PlanetBundle {
        planet: Planet {
            shadow: triangle_id,
            size: scaled_size,
        },
        name,
        mat_mesh: MaterialMesh2dBundle {
            mesh: Mesh2dHandle(
                meshes.add(
                    Circle::new(scaled_size)
                        .mesh()
                        .resolution(MESH_RESOLUTION)
                        .build(),
                ),
            ),
            material: materials.add(color),
            ..Default::default()
        },
        orbit: Orbit::circle(scaled_radius, orbital_period),
    });
    planet.insert(StateScoped(Screen::Playing));
    for child in children {
        planet.add_child(child);
    }
    (planet.id(), orbit_id)
}

pub fn scale(original: f32) -> f32 {
    static FACTOR: f32 = 0.005;
    (original / 2.) * FACTOR
}
