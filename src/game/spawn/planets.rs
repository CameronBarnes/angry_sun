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

//#[derive(Component, Debug)]
//pub struct PlanetShadow;

static MESH_RESOLUTION: usize = 100;

static PLANET_SCALE: f32 = 10.;
static RADIUS_SCALE: f32 = 0.125;
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
    );

    // TODO: Add moons
    spawn_planet(
        &mut commands,
        &mut meshes,
        &mut materials,
        Name::new("Mars"),
        scale(288_000_000. * RADIUS_SCALE),
        scale(6_790. * PLANET_SCALE),
        687.,
        Color::srgb(240. / 255., 231. / 255., 231. / 255.),
        circle_color.clone(),
        vec![],
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
        Color::srgb(235. / 255., 243. / 255., 246. / 255.),
        circle_color.clone(),
        vec![],
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
    );

    // TODO: Add moonsca
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
) -> (Entity, Entity) {
    let orbit_id = commands
        .spawn((
            Name::new("Mercury - Orbit Circle"),
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(
                    meshes.add(
                        Annulus::new(scaled_radius - 5., scaled_radius + 5.)
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
        planet: Planet,
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

fn scale(original: f32) -> f32 {
    static FACTOR: f32 = 0.0005;
    (original / 2.) * FACTOR
}
