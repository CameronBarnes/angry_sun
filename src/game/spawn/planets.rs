use bevy::{
    app::App,
    asset::Assets,
    color::{Color, Luminance},
    core::Name,
    prelude::{
        Annulus, BuildChildren, Circle, Commands, Event, Mesh, ResMut, StateScoped, Trigger,
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
    static PLANET_SCALE: f32 = 10.;
    static RADIUS_SCALE: f32 = 0.125;
    static MOON_SCALE: f32 = 5.;
    static MOON_RADIUS_SCALE: f32 = 1.5;

    let circle_color = materials.add(Color::WHITE.darker(0.75));

    commands
        .spawn((
            Name::new("Sun"),
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(scale(1_400_000.)))),
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
                mesh: Mesh2dHandle(meshes.add(Circle::new(scale(4_879. * PLANET_SCALE)))),
                material: materials.add(Color::srgb(183. / 255., 184. / 255., 185. / 255.)),
                ..Default::default()
            },
            orbit: Orbit::circle(mercury_radius, 88.),
        })
        .insert(StateScoped(Screen::Playing));
    commands.spawn((
        Name::new("Mercury - Orbit Circle"),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Annulus::new(mercury_radius - 5., mercury_radius + 5.))),
            material: circle_color,
            ..Default::default()
        },
        StateScoped(Screen::Playing),
    ));

    commands
        .spawn(PlanetBundle {
            planet: Planet,
            name: Name::new("Venus"),
            mat_mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(scale(12_104. * PLANET_SCALE)))),
                material: materials.add(Color::srgb(165. / 255., 124. / 255., 27. / 255.)),
                ..Default::default()
            },
            orbit: Orbit::circle(scale(108_000_000. * RADIUS_SCALE), 224.7),
        })
        .insert(StateScoped(Screen::Playing));

    let moon = commands
        .spawn(PlanetBundle {
            planet: Planet,
            name: Name::new("Moon"),
            mat_mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(scale(3_475. * MOON_SCALE)))),
                material: materials.add(Color::srgb(246. / 255., 241. / 255., 213. / 255.)),
                ..Default::default()
            },
            orbit: Orbit::circle(scale(384_400. * MOON_RADIUS_SCALE), 27.3),
        })
        .insert(StateScoped(Screen::Playing))
        .id();
    commands
        .spawn(PlanetBundle {
            planet: Planet,
            name: Name::new("Earth"),
            mat_mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(scale(12_756. * PLANET_SCALE)))),
                material: materials.add(Color::srgb(79. / 255., 76. / 255., 176. / 255.)),
                ..Default::default()
            },
            orbit: Orbit::circle(scale(149_000_000. * RADIUS_SCALE), 365.25),
        })
        .insert(StateScoped(Screen::Playing))
        .add_child(moon);

    // TODO: Add moons
    commands
        .spawn(PlanetBundle {
            planet: Planet,
            name: Name::new("Mars"),
            mat_mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(scale(6_790. * PLANET_SCALE)))),
                material: materials.add(Color::srgb(240. / 255., 231. / 255., 231. / 255.)),
                ..Default::default()
            },
            orbit: Orbit::circle(scale(288_000_000. * RADIUS_SCALE), 687.),
        })
        .insert(StateScoped(Screen::Playing));

    // TODO: Add moons
    commands
        .spawn(PlanetBundle {
            planet: Planet,
            name: Name::new("Jupiter"),
            mat_mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(scale(143_000. * PLANET_SCALE)))),
                material: materials.add(Color::srgb(235. / 255., 243. / 255., 246. / 255.)),
                ..Default::default()
            },
            orbit: Orbit::circle(scale(780_000_000. * RADIUS_SCALE), 4_330.6),
        })
        .insert(StateScoped(Screen::Playing));

    // TODO: Add moons
    commands
        .spawn(PlanetBundle {
            planet: Planet,
            name: Name::new("Saturn"),
            mat_mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(scale(120_536. * PLANET_SCALE)))),
                material: materials.add(Color::srgb(206. / 255., 184. / 255., 184. / 255.)),
                ..Default::default()
            },
            orbit: Orbit::circle(scale(1_437_000_000. * RADIUS_SCALE), 10_756.),
        })
        .insert(StateScoped(Screen::Playing));

    // TODO: Add moons
    commands
        .spawn(PlanetBundle {
            planet: Planet,
            name: Name::new("Uranus"),
            mat_mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(scale(51_118. * PLANET_SCALE)))),
                material: materials.add(Color::srgb(172. / 255., 229. / 255., 238. / 255.)),
                ..Default::default()
            },
            orbit: Orbit::circle(scale(2_871_000_000. * RADIUS_SCALE), 30_687.),
        })
        .insert(StateScoped(Screen::Playing));

    // TODO: Add moons
    commands
        .spawn(PlanetBundle {
            planet: Planet,
            name: Name::new("Neptune"),
            mat_mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(scale(49_528. * PLANET_SCALE)))),
                material: materials.add(Color::srgb(120. / 255., 192. / 255., 168. / 255.)),
                ..Default::default()
            },
            orbit: Orbit::circle(scale(4_530_000_000. * RADIUS_SCALE), 60_190.),
        })
        .insert(StateScoped(Screen::Playing));
}

fn scale(original: f32) -> f32 {
    static FACTOR: f32 = 0.0005;
    (original / 2.) * FACTOR
}
