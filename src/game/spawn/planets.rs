use bevy::{
    app::App,
    asset::Assets,
    color::{Color, Luminance},
    core::Name,
    prelude::{
        Annulus, BuildChildren, Circle, Commands, Event, Mesh, MeshBuilder, Meshable, ResMut,
        StateScoped, Transform, Trigger,
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
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

// FIXME: Fix the too many lines issue by breaking this up
#[allow(clippy::too_many_lines)]
fn spawn_solar_system(
    _trigger: Trigger<SpawnSolarSystem>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    static MESH_RESOLUTION: usize = 100;

    static PLANET_SCALE: f32 = 10.;
    static RADIUS_SCALE: f32 = 0.125;
    static MOON_SCALE: f32 = 5.;
    static MOON_RADIUS_SCALE: f32 = 1.5;

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

    let mercury_radius = scale(57_000_000. * RADIUS_SCALE);
    commands
        .spawn(PlanetBundle {
            planet: Planet,
            name: Name::new("Mercury"),
            mat_mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(
                    meshes.add(
                        Circle::new(scale(4_879. * PLANET_SCALE))
                            .mesh()
                            .resolution(MESH_RESOLUTION)
                            .build(),
                    ),
                ),
                material: materials.add(Color::srgb(183. / 255., 184. / 255., 185. / 255.)),
                ..Default::default()
            },
            orbit: Orbit::circle(mercury_radius, 88.),
        })
        .insert(StateScoped(Screen::Playing));
    commands.spawn((
        Name::new("Mercury - Orbit Circle"),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(
                meshes.add(
                    Annulus::new(mercury_radius - 5., mercury_radius + 5.)
                        .mesh()
                        .resolution(MESH_RESOLUTION)
                        .build(),
                ),
            ),
            material: circle_color.clone(),
            transform: Transform::from_xyz(0., 0., -2.),
            ..Default::default()
        },
        StateScoped(Screen::Playing),
    ));

    let venus_radius = scale(108_000_000. * RADIUS_SCALE);
    commands
        .spawn(PlanetBundle {
            planet: Planet,
            name: Name::new("Venus"),
            mat_mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(
                    meshes.add(
                        Circle::new(scale(12_104. * PLANET_SCALE))
                            .mesh()
                            .resolution(MESH_RESOLUTION)
                            .build(),
                    ),
                ),
                material: materials.add(Color::srgb(165. / 255., 124. / 255., 27. / 255.)),
                ..Default::default()
            },
            orbit: Orbit::circle(venus_radius, 224.7),
        })
        .insert(StateScoped(Screen::Playing));
    commands.spawn((
        Name::new("Venus - Orbit Circle"),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(
                meshes.add(
                    Annulus::new(venus_radius - 5., venus_radius + 5.)
                        .mesh()
                        .resolution(MESH_RESOLUTION)
                        .build(),
                ),
            ),
            material: circle_color.clone(),
            transform: Transform::from_xyz(0., 0., -2.),
            ..Default::default()
        },
        StateScoped(Screen::Playing),
    ));

    let moon_radius = scale(384_400. * MOON_RADIUS_SCALE);
    let moon = commands
        .spawn(PlanetBundle {
            planet: Planet,
            name: Name::new("Moon"),
            mat_mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(
                    meshes.add(
                        Circle::new(scale(3_475. * MOON_SCALE))
                            .mesh()
                            .resolution(MESH_RESOLUTION)
                            .build(),
                    ),
                ),
                material: materials.add(Color::srgb(246. / 255., 241. / 255., 213. / 255.)),
                ..Default::default()
            },
            orbit: Orbit::circle(moon_radius, 27.3),
        })
        .id();
    let moon_orbit_circle = commands
        .spawn((
            Name::new("Moon - Orbit Circle"),
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(
                    meshes.add(
                        Annulus::new(moon_radius - 2., moon_radius + 2.)
                            .mesh()
                            .resolution(MESH_RESOLUTION)
                            .build(),
                    ),
                ),
                material: circle_color.clone(),
                transform: Transform::from_xyz(0., 0., -2.),
                ..Default::default()
            },
        ))
        .id();
    let earth_radius = scale(149_000_000. * RADIUS_SCALE);
    commands
        .spawn(PlanetBundle {
            planet: Planet,
            name: Name::new("Earth"),
            mat_mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(
                    meshes.add(
                        Circle::new(scale(12_756. * PLANET_SCALE))
                            .mesh()
                            .resolution(MESH_RESOLUTION)
                            .build(),
                    ),
                ),
                material: materials.add(Color::srgb(79. / 255., 76. / 255., 176. / 255.)),
                ..Default::default()
            },
            orbit: Orbit::circle(earth_radius, 365.25),
        })
        .insert(StateScoped(Screen::Playing))
        .add_child(moon)
        .add_child(moon_orbit_circle);
    commands.spawn((
        Name::new("Earth - Orbit Circle"),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(
                meshes.add(
                    Annulus::new(earth_radius - 5., earth_radius + 5.)
                        .mesh()
                        .resolution(MESH_RESOLUTION)
                        .build(),
                ),
            ),
            material: circle_color.clone(),
            transform: Transform::from_xyz(0., 0., -2.),
            ..Default::default()
        },
        StateScoped(Screen::Playing),
    ));

    // TODO: Add moons
    let mars_radius = scale(288_000_000. * RADIUS_SCALE);
    commands
        .spawn(PlanetBundle {
            planet: Planet,
            name: Name::new("Mars"),
            mat_mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(
                    meshes.add(
                        Circle::new(scale(6_790. * PLANET_SCALE))
                            .mesh()
                            .resolution(MESH_RESOLUTION)
                            .build(),
                    ),
                ),
                material: materials.add(Color::srgb(240. / 255., 231. / 255., 231. / 255.)),
                ..Default::default()
            },
            orbit: Orbit::circle(mars_radius, 687.),
        })
        .insert(StateScoped(Screen::Playing));
    commands.spawn((
        Name::new("Mars - Orbit Circle"),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(
                meshes.add(
                    Annulus::new(mars_radius - 5., mars_radius + 5.)
                        .mesh()
                        .resolution(MESH_RESOLUTION)
                        .build(),
                ),
            ),
            material: circle_color.clone(),
            transform: Transform::from_xyz(0., 0., -2.),
            ..Default::default()
        },
        StateScoped(Screen::Playing),
    ));

    // TODO: Add moons
    let jupiter_radius = scale(780_000_000. * RADIUS_SCALE);
    commands
        .spawn(PlanetBundle {
            planet: Planet,
            name: Name::new("Jupiter"),
            mat_mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(
                    meshes.add(
                        Circle::new(scale(143_000. * PLANET_SCALE))
                            .mesh()
                            .resolution(MESH_RESOLUTION)
                            .build(),
                    ),
                ),
                material: materials.add(Color::srgb(235. / 255., 243. / 255., 246. / 255.)),
                ..Default::default()
            },
            orbit: Orbit::circle(jupiter_radius, 4_330.6),
        })
        .insert(StateScoped(Screen::Playing));
    commands.spawn((
        Name::new("Jupiter - Orbit Circle"),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(
                meshes.add(
                    Annulus::new(jupiter_radius - 5., jupiter_radius + 5.)
                        .mesh()
                        .resolution(MESH_RESOLUTION)
                        .build(),
                ),
            ),
            material: circle_color.clone(),
            transform: Transform::from_xyz(0., 0., -2.),
            ..Default::default()
        },
        StateScoped(Screen::Playing),
    ));

    // TODO: Add moons
    let saturn_radius = scale(1_437_000_000. * RADIUS_SCALE);
    commands
        .spawn(PlanetBundle {
            planet: Planet,
            name: Name::new("Saturn"),
            mat_mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(
                    meshes.add(
                        Circle::new(scale(120_536. * PLANET_SCALE))
                            .mesh()
                            .resolution(MESH_RESOLUTION)
                            .build(),
                    ),
                ),
                material: materials.add(Color::srgb(206. / 255., 184. / 255., 184. / 255.)),
                ..Default::default()
            },
            orbit: Orbit::circle(saturn_radius, 10_756.),
        })
        .insert(StateScoped(Screen::Playing));
    commands.spawn((
        Name::new("Saturn - Orbit Circle"),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(
                meshes.add(
                    Annulus::new(saturn_radius - 5., saturn_radius + 5.)
                        .mesh()
                        .resolution(MESH_RESOLUTION)
                        .build(),
                ),
            ),
            material: circle_color.clone(),
            transform: Transform::from_xyz(0., 0., -2.),
            ..Default::default()
        },
        StateScoped(Screen::Playing),
    ));

    // TODO: Add moons
    let uranus_radius = scale(2_871_000_000. * RADIUS_SCALE);
    commands
        .spawn(PlanetBundle {
            planet: Planet,
            name: Name::new("Uranus"),
            mat_mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(
                    meshes.add(
                        Circle::new(scale(51_118. * PLANET_SCALE))
                            .mesh()
                            .resolution(MESH_RESOLUTION)
                            .build(),
                    ),
                ),
                material: materials.add(Color::srgb(172. / 255., 229. / 255., 238. / 255.)),
                ..Default::default()
            },
            orbit: Orbit::circle(uranus_radius, 30_687.),
        })
        .insert(StateScoped(Screen::Playing));
    commands.spawn((
        Name::new("Uranus - Orbit Circle"),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(
                meshes.add(
                    Annulus::new(uranus_radius - 5., uranus_radius + 5.)
                        .mesh()
                        .resolution(MESH_RESOLUTION)
                        .build(),
                ),
            ),
            material: circle_color.clone(),
            transform: Transform::from_xyz(0., 0., -2.),
            ..Default::default()
        },
        StateScoped(Screen::Playing),
    ));

    // TODO: Add moons
    let neptune_radius = scale(4_530_000_000. * RADIUS_SCALE);
    commands
        .spawn(PlanetBundle {
            planet: Planet,
            name: Name::new("Neptune"),
            mat_mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(
                    meshes.add(
                        Circle::new(scale(49_528. * PLANET_SCALE))
                            .mesh()
                            .resolution(MESH_RESOLUTION)
                            .build(),
                    ),
                ),
                material: materials.add(Color::srgb(120. / 255., 192. / 255., 168. / 255.)),
                ..Default::default()
            },
            orbit: Orbit::circle(neptune_radius, 60_190.),
        })
        .insert(StateScoped(Screen::Playing));
    commands.spawn((
        Name::new("Neptune - Orbit Circle"),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(
                meshes.add(
                    Annulus::new(neptune_radius - 5., neptune_radius + 5.)
                        .mesh()
                        .resolution(MESH_RESOLUTION)
                        .build(),
                ),
            ),
            material: circle_color,
            transform: Transform::from_xyz(0., 0., -2.),
            ..Default::default()
        },
        StateScoped(Screen::Playing),
    ));
}

fn scale(original: f32) -> f32 {
    static FACTOR: f32 = 0.0005;
    (original / 2.) * FACTOR
}
