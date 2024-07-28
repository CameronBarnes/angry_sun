use std::sync::LazyLock;

use bevy::{
    app::App,
    asset::{Assets, Handle},
    color::{Color, Luminance},
    core::Name,
    prelude::*,
    sprite::{ColorMaterial, Material2d, MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_mod_picking::PickableBundle;

use crate::{
    game::{
        camera::{FinishZoom, ScaleWithZoom},
        highlight::{HasHighlightObject, HighlightObject, LinkSelectObject},
        planets::{Orbit, Planet, PlanetBundle},
        sun::SpawnSun,
    },
    screen::Screen,
};

#[derive(Event, Debug)]
pub struct SpawnSolarSystem;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_solar_system);
}

#[derive(Component, Debug)]
pub struct PlanetShadow;

#[derive(Component, Debug)]
pub struct OrbitRing;

static MESH_RESOLUTION: usize = 100;

static PLANET_SCALE: f32 = 12.;
pub static RADIUS_SCALE: f32 = 0.1;
static MOON_SCALE: f32 = 7.;
static MOON_RADIUS_SCALE: f32 = 1.5;

pub static LAST_PLANET_DISTANCE: LazyLock<f32> =
    LazyLock::new(|| scale(4_530_000_000. * RADIUS_SCALE * 0.7));

// FIXME: Fix the too many lines issue by breaking this up
#[allow(clippy::too_many_lines)]
fn spawn_solar_system(
    _trigger: Trigger<SpawnSolarSystem>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shadow_color = materials.add(Color::BLACK.with_alpha(0.5));
    let circle_color = materials.add(Color::WHITE.darker(0.8));
    commands.trigger(SpawnSun);

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
                material: materials.add(Color::srgb(
                    (253. / 255.) * 10.,
                    (184. / 255.) * 10.,
                    (19. / 255.) * 10.,
                )),
                ..Default::default()
            },
            ScaleWithZoom { ratio: 0.1 },
            PickableBundle::default(),
        ))
        .insert(StateScoped(Screen::Playing));

    spawn_planet(
        &mut commands,
        &mut meshes,
        &mut materials,
        Name::new("Mercury"),
        scale(57_000_000. * RADIUS_SCALE * 1.2), // We've adjusted mercury specifically because
        // it's so close to the sun
        scale(4_879. * PLANET_SCALE),
        88.,
        Color::srgb(183. / 255., 184. / 255., 185. / 255.),
        circle_color.clone(),
        shadow_color.clone(),
        vec![],
        false,
        Some(2.5),
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
        shadow_color.clone(),
        vec![],
        false,
        None,
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
        shadow_color.clone(),
        vec![],
        true,
        Some(0.5),
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
        shadow_color.clone(),
        vec![moon.0, moon.1],
        false,
        None,
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
        shadow_color.clone(),
        vec![],
        false,
        None,
    );

    // TODO: Add moons
    spawn_planet(
        &mut commands,
        &mut meshes,
        &mut materials,
        Name::new("Jupiter"),
        scale(780_000_000. * RADIUS_SCALE * 0.8),
        scale(143_000. * PLANET_SCALE),
        4_330.6 * 0.8,
        Color::srgb(148. / 255., 105. / 255., 86. / 255.),
        circle_color.clone(),
        shadow_color.clone(),
        vec![],
        false,
        Some(0.3),
    );

    // TODO: Add moons
    spawn_planet(
        &mut commands,
        &mut meshes,
        &mut materials,
        Name::new("Saturn"),
        scale(1_437_000_000. * RADIUS_SCALE * 0.8),
        scale(120_536. * PLANET_SCALE),
        10_756. * 0.8,
        Color::srgb(206. / 255., 184. / 255., 184. / 255.),
        circle_color.clone(),
        shadow_color.clone(),
        vec![],
        false,
        Some(0.3),
    );

    // TODO: Add moons
    spawn_planet(
        &mut commands,
        &mut meshes,
        &mut materials,
        Name::new("Uranus"),
        scale(2_871_000_000. * RADIUS_SCALE * 0.7),
        scale(51_118. * PLANET_SCALE),
        30_687. * 0.7,
        Color::srgb(172. / 255., 229. / 255., 238. / 255.),
        circle_color.clone(),
        shadow_color.clone(),
        vec![],
        false,
        Some(0.6),
    );

    // TODO: Add moons
    spawn_planet(
        &mut commands,
        &mut meshes,
        &mut materials,
        Name::new("Neptune"),
        *LAST_PLANET_DISTANCE,
        scale(49_528. * PLANET_SCALE),
        60_190. * 0.7,
        Color::srgb(120. / 255., 192. / 255., 168. / 255.),
        circle_color,
        shadow_color,
        vec![],
        false,
        Some(0.6),
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
    shadow_color: Handle<A>,
    children: Vec<Entity>,
    moon: bool,
    zoom_scale: Option<f32>,
) -> (Entity, Entity) {
    // Spawn planet shadow
    let shadow = commands
        .spawn((
            PlanetShadow,
            StateScoped(Screen::Playing),
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(
                    meshes.add(CircularSector::new(scaled_size, std::f32::consts::PI / 2.)),
                ),
                material: shadow_color,
                transform: Transform::from_xyz(0., 0., 1.),
                ..Default::default()
            },
        ))
        //.insert(ScaleWithZoom {
        //    ratio: zoom_scale.unwrap_or(1.),
        //})
        .id();

    // Spawn the highlight circle
    let highlight = commands
        .spawn((
            HighlightObject,
            StateScoped(Screen::Playing),
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(
                    meshes.add(
                        Circle::new(scaled_size * 1.4)
                            .mesh()
                            .resolution(MESH_RESOLUTION)
                            .build(),
                    ),
                ),
                material: materials.add(Color::WHITE),
                visibility: Visibility::Hidden,
                transform: Transform::from_xyz(0., 0., -3.),
                ..Default::default()
            },
        ))
        .id();

    // Spawn the planet
    let mut planet = commands.spawn(PlanetBundle {
        planet: Planet {
            is_moon: moon,
            shadow,
            size: scaled_size,
            absorbed_power: 0.,
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
    planet.add_child(shadow);
    planet.add_child(highlight);
    // Handle it being StateScoped and handle ScaleWithZoom
    planet.insert((
        StateScoped(Screen::Playing),
        ScaleWithZoom {
            ratio: zoom_scale.unwrap_or(1.),
        },
        PickableBundle::default(),
        HasHighlightObject(highlight),
        FinishZoom::default(),
    ));
    // Add supplied children, usually moons
    for child in children {
        planet.add_child(child);
    }

    let planet = planet.id();
    let border_width = if moon { 6. } else { 60. };

    // Spawn the orbit circle
    let orbit_id = commands
        .spawn((
            OrbitRing,
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
            LinkSelectObject(planet),
            PickableBundle::default(),
        ))
        .id();

    (planet, orbit_id)
}

pub fn scale(original: f32) -> f32 {
    static FACTOR: f32 = 0.005;
    (original / 2.) * FACTOR
}
