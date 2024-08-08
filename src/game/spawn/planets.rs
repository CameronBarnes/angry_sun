use std::sync::LazyLock;

use bevy::{
    app::App,
    asset::{Assets, Handle},
    color::{Color, Luminance},
    core::Name,
    prelude::*,
    sprite::{ColorMaterial, Material2d, MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_mod_picking::{
    events::{Click, Pointer},
    prelude::On,
    PickableBundle,
};

use crate::{
    game::{
        camera::{ClearFinishZoomEvent, FinishZoom, ScaleWithZoom},
        highlight::{HighlightObject, LinkSelectionObject},
        planets::{Orbit, Planet, PlanetBundle},
        resources::{PlanetResources, RawResource, RawResourceType, StationType},
        sun::Sun,
        unlocks::Technology,
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

fn spawn_solar_system(
    _trigger: Trigger<SpawnSolarSystem>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shadow_color = materials.add(Color::BLACK.with_alpha(0.5));
    let orbit_circle = materials.add(Color::WHITE.darker(0.8));

    spawn_sun(&mut commands, &mut meshes, &mut materials);

    spawn_mercury(
        &mut commands,
        &mut meshes,
        &mut materials,
        orbit_circle.clone(),
        shadow_color.clone(),
    );

    spawn_venus(
        &mut commands,
        &mut meshes,
        &mut materials,
        orbit_circle.clone(),
        shadow_color.clone(),
    );

    spawn_earth(
        &mut commands,
        &mut meshes,
        &mut materials,
        orbit_circle.clone(),
        shadow_color.clone(),
    );

    spawn_mars(
        &mut commands,
        &mut meshes,
        &mut materials,
        orbit_circle.clone(),
        shadow_color.clone(),
    );

    spawn_jupiter(
        &mut commands,
        &mut meshes,
        &mut materials,
        orbit_circle.clone(),
        shadow_color.clone(),
    );

    spawn_saturn(
        &mut commands,
        &mut meshes,
        &mut materials,
        orbit_circle.clone(),
        shadow_color.clone(),
    );

    spawn_uranus(
        &mut commands,
        &mut meshes,
        &mut materials,
        orbit_circle.clone(),
        shadow_color.clone(),
    );

    spawn_neptune(
        &mut commands,
        &mut meshes,
        &mut materials,
        orbit_circle,
        shadow_color,
    );
}

fn spawn_sun(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Name::new("Sun"),
        Sun::default(),
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
        FinishZoom::new_with_target(35.),
        PlanetResources(vec![RawResource::new(
            RawResourceType::Hydrogen,
            StationType::Orbit,
            vec![(0.71 * scale(1_400_000.), Technology::StellarLifting)],
        )]),
        StateScoped(Screen::Playing),
    ));
}

fn spawn_mercury<A: Material2d>(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    orbit_circle: Handle<A>,
    shadow_color: Handle<A>,
) {
    spawn_planet(
        commands,
        meshes,
        materials,
        Name::new("Mercury"),
        scale(57_000_000. * RADIUS_SCALE * 1.2), // We've adjusted mercury specifically because
        // it's so close to the sun
        scale(4_879. * PLANET_SCALE),
        88.,
        Color::srgb(183. / 255., 184. / 255., 185. / 255.),
        orbit_circle,
        shadow_color,
        vec![],
        false,
        false,
        Some(2.5),
        PlanetResources(vec![
            RawResource::new(
                RawResourceType::Metals,
                StationType::Surface,
                vec![
                    (0.07, Technology::ExtraTerrestrialMining),
                    (0.7, Technology::DeepCrustMining),
                ],
            ),
            RawResource::new(
                RawResourceType::Silicate,
                StationType::Surface,
                vec![
                    (0.03, Technology::ExtraTerrestrialMining),
                    (0.3, Technology::DeepCrustMining),
                ],
            ),
        ]),
    );
}

fn spawn_venus<A: Material2d>(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    orbit_circle: Handle<A>,
    shadow_color: Handle<A>,
) {
    spawn_planet(
        commands,
        meshes,
        materials,
        Name::new("Venus"),
        scale(108_000_000. * RADIUS_SCALE),
        scale(12_104. * PLANET_SCALE),
        224.7,
        Color::srgb(165. / 255., 124. / 255., 27. / 255.),
        orbit_circle,
        shadow_color,
        vec![],
        false,
        false,
        None,
        PlanetResources(vec![
            RawResource::new(
                RawResourceType::Metals,
                StationType::Surface,
                vec![
                    (0.05, Technology::HotSurfaceMining),
                    (0.5, Technology::DeepCrustMining),
                ],
            ),
            RawResource::new(
                RawResourceType::Silicate,
                StationType::Surface,
                vec![
                    (0.015, Technology::HotSurfaceMining),
                    (0.15, Technology::DeepCrustMining),
                ],
            ),
        ]),
    );
}

fn spawn_earth<A: Material2d>(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    orbit_circle: Handle<A>,
    shadow_color: Handle<A>,
) {
    let moon = spawn_planet(
        commands,
        meshes,
        materials,
        Name::new("Moon"),
        scale(384_400. * MOON_RADIUS_SCALE),
        scale(3_475. * MOON_SCALE),
        27.3,
        Color::srgb(246. / 255., 241. / 255., 213. / 255.),
        orbit_circle.clone(),
        shadow_color.clone(),
        vec![],
        true,
        false,
        Some(0.5),
        PlanetResources(vec![
            RawResource::new(
                RawResourceType::Metals,
                StationType::Surface,
                vec![
                    (0.03, Technology::ExtraTerrestrialMining),
                    (0.3, Technology::DeepCrustMining),
                ],
            ),
            RawResource::new(
                RawResourceType::Silicate,
                StationType::Surface,
                vec![
                    (0.02, Technology::ExtraTerrestrialMining),
                    (0.2, Technology::DeepCrustMining),
                ],
            ),
            RawResource::new(
                RawResourceType::Oxygen,
                StationType::Surface,
                vec![
                    (0.043, Technology::SurfaceMineralDecomposition),
                    (0.43, Technology::DeepCrustMining),
                ],
            ),
        ]),
    );
    spawn_planet(
        commands,
        meshes,
        materials,
        Name::new("Earth"),
        scale(149_000_000. * RADIUS_SCALE),
        scale(12_756. * PLANET_SCALE),
        365.25,
        Color::srgb(79. / 255., 76. / 255., 176. / 255.),
        orbit_circle,
        shadow_color,
        moon,
        false,
        true,
        None,
        PlanetResources(vec![
            RawResource::new(
                RawResourceType::Metals,
                StationType::Surface,
                vec![
                    (0.025, Technology::None),
                    (0.05, Technology::DeepSeaMining),
                    (0.5, Technology::DeepCrustMining),
                ],
            ),
            RawResource::new(
                RawResourceType::Silicate,
                StationType::Surface,
                vec![
                    (0.00725, Technology::None),
                    (0.015, Technology::DeepSeaMining),
                    (0.15, Technology::DeepCrustMining),
                ],
            ),
            RawResource::new(
                RawResourceType::Oxygen,
                StationType::Ocean,
                vec![
                    (0.003, Technology::None),
                    (0.03, Technology::SeaWaterElectrolysis),
                ],
            ),
            RawResource::new(
                RawResourceType::Hydrogen,
                StationType::Ocean,
                vec![
                    (0.0005, Technology::None),
                    (0.03, Technology::SeaWaterElectrolysis),
                ],
            ),
        ]),
    );
}

fn spawn_mars<A: Material2d>(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    orbit_circle: Handle<A>,
    shadow_color: Handle<A>,
) {
    spawn_planet(
        commands,
        meshes,
        materials,
        Name::new("Mars"),
        scale(288_000_000. * RADIUS_SCALE),
        scale(6_790. * PLANET_SCALE),
        687.,
        Color::srgb(193. / 255., 68. / 255., 14. / 255.),
        orbit_circle,
        shadow_color,
        vec![],
        false,
        false,
        None,
        PlanetResources(vec![
            RawResource::new(
                RawResourceType::Metals,
                StationType::Surface,
                vec![
                    (0.025, Technology::ExtraTerrestrialMining),
                    (0.25, Technology::DeepCrustMining),
                ],
            ),
            RawResource::new(
                RawResourceType::Silicate,
                StationType::Surface,
                vec![
                    (0.023, Technology::ExtraTerrestrialMining),
                    (0.23, Technology::DeepCrustMining),
                ],
            ),
            RawResource::new(
                RawResourceType::Oxygen,
                StationType::Surface,
                vec![
                    (0.043, Technology::SurfaceMineralDecomposition),
                    (0.43, Technology::DeepCrustMining),
                ],
            ),
        ]),
    );
}

fn spawn_jupiter<A: Material2d>(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    orbit_circle: Handle<A>,
    shadow_color: Handle<A>,
) {
    // TODO: Add moons
    spawn_planet(
        commands,
        meshes,
        materials,
        Name::new("Jupiter"),
        scale(780_000_000. * RADIUS_SCALE * 0.8),
        scale(143_000. * PLANET_SCALE),
        4_330.6 * 0.8,
        Color::srgb(148. / 255., 105. / 255., 86. / 255.),
        orbit_circle,
        shadow_color,
        vec![],
        false,
        true,
        Some(0.3),
        PlanetResources(vec![RawResource::new(
            RawResourceType::Hydrogen,
            StationType::Orbit,
            vec![(0.9, Technology::GasGiantMining)],
        )]),
    );
}

fn spawn_saturn<A: Material2d>(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    orbit_circle: Handle<A>,
    shadow_color: Handle<A>,
) {
    // TODO: Add moons
    spawn_planet(
        commands,
        meshes,
        materials,
        Name::new("Saturn"),
        scale(1_437_000_000. * RADIUS_SCALE * 0.8),
        scale(120_536. * PLANET_SCALE),
        10_756. * 0.8,
        Color::srgb(206. / 255., 184. / 255., 184. / 255.),
        orbit_circle,
        shadow_color,
        vec![],
        false,
        true,
        Some(0.3),
        PlanetResources(vec![RawResource::new(
            RawResourceType::Hydrogen,
            StationType::Orbit,
            vec![(0.96, Technology::GasGiantMining)],
        )]),
    );
}

fn spawn_uranus<A: Material2d>(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    orbit_circle: Handle<A>,
    shadow_color: Handle<A>,
) {
    // TODO: Add moons
    spawn_planet(
        commands,
        meshes,
        materials,
        Name::new("Uranus"),
        scale(2_871_000_000. * RADIUS_SCALE * 0.7),
        scale(51_118. * PLANET_SCALE),
        30_687. * 0.7,
        Color::srgb(172. / 255., 229. / 255., 238. / 255.),
        orbit_circle,
        shadow_color,
        vec![],
        false,
        true,
        Some(0.6),
        PlanetResources(vec![RawResource::new(
            RawResourceType::Hydrogen,
            StationType::Orbit,
            vec![(0.83, Technology::GasGiantMining)],
        )]),
    );
}

fn spawn_neptune<A: Material2d>(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    orbit_circle: Handle<A>,
    shadow_color: Handle<A>,
) {
    // TODO: Add moons
    spawn_planet(
        commands,
        meshes,
        materials,
        Name::new("Neptune"),
        *LAST_PLANET_DISTANCE,
        scale(49_528. * PLANET_SCALE),
        60_190. * 0.7,
        Color::srgb(120. / 255., 192. / 255., 168. / 255.),
        orbit_circle,
        shadow_color,
        vec![],
        false,
        true,
        Some(0.6),
        PlanetResources(vec![RawResource::new(
            RawResourceType::Hydrogen,
            StationType::Orbit,
            vec![(0.80, Technology::GasGiantMining)],
        )]),
    );
}

// FIXME: Fix the too many lines issue by breaking this up
#[allow(clippy::too_many_lines)]
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
    magnetic_field: bool,
    zoom_scale: Option<f32>,
    resources: PlanetResources,
) -> Vec<Entity> {
    let (border_width, highlight_circle, width_modifier) =
        if moon { (6., 1.6, 20.) } else { (60., 1.4, 5.) };

    // Spawn the planet
    let mut planet = commands.spawn((
        PlanetBundle {
            planet: Planet {
                is_moon: moon,
                has_magnetic_field: magnetic_field,
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
        },
        StateScoped(Screen::Playing),
        ScaleWithZoom {
            ratio: zoom_scale.unwrap_or(1.),
        },
        PickableBundle::default(),
        FinishZoom::new_with_target(15. / zoom_scale.unwrap_or(1.)),
        resources,
        On::<Pointer<Click>>::commands_mut(|_input, commands: &mut Commands| {
            commands.trigger(ClearFinishZoomEvent);
        }),
    ));
    planet.with_children(|parent| {
        parent.spawn((
            // Spawn the highlight circle
            HighlightObject,
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(
                    meshes.add(
                        Circle::new(scaled_size * highlight_circle)
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
            PickableBundle::default(),
            LinkSelectionObject(parent.parent_entity()),
            On::<Pointer<Click>>::commands_mut(|_input, commands: &mut Commands| {
                commands.trigger(ClearFinishZoomEvent);
            }),
        ));

        parent.spawn((
            // Spawn planet shadow
            PlanetShadow,
            PickableBundle::default(),
            LinkSelectionObject(parent.parent_entity()),
            On::<Pointer<Click>>::commands_mut(|_input, commands: &mut Commands| {
                commands.trigger(ClearFinishZoomEvent);
            }),
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(
                    meshes.add(CircularSector::new(scaled_size, std::f32::consts::PI / 2.)),
                ),
                material: shadow_color,
                transform: Transform::from_xyz(0., 0., 1.),
                ..Default::default()
            },
        ));
    });

    // Add supplied children, usually moons
    for child in children {
        planet.add_child(child);
    }

    let planet = planet.id();

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
                material: orbit_circle.clone(),
                transform: Transform::from_xyz(0., 0., -2.),
                ..Default::default()
            },
            StateScoped(Screen::Playing),
            LinkSelectionObject(planet),
            PickableBundle::default(),
            On::<Pointer<Click>>::commands_mut(|_input, commands: &mut Commands| {
                commands.trigger(ClearFinishZoomEvent);
            }),
        ))
        .id();

    // We want a second bigger orbit circle for selection purposes
    let orbit_selection_circle = commands
        .spawn((
            OrbitRing,
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(
                    meshes.add(
                        Annulus::new(
                            border_width.mul_add(-width_modifier, scaled_radius),
                            border_width.mul_add(width_modifier, scaled_radius),
                        )
                        .mesh()
                        .resolution(MESH_RESOLUTION)
                        .build(),
                    ),
                ),
                material: orbit_circle,
                transform: Transform::from_xyz(0., 0., -2.),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            StateScoped(Screen::Playing),
            LinkSelectionObject(planet),
            PickableBundle::default(),
            On::<Pointer<Click>>::commands_mut(|_input, commands: &mut Commands| {
                commands.trigger(ClearFinishZoomEvent);
            }),
        ))
        .id();

    vec![planet, orbit_id, orbit_selection_circle]
}

pub fn scale(original: f32) -> f32 {
    static FACTOR: f32 = 0.005;
    (original / 2.) * FACTOR
}
