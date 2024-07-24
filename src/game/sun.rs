use std::ops::Div;

use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::screen::Screen;

#[derive(Event, Debug)]
pub struct SpawnSun;

pub(super) fn plugin(app: &mut App) {
    app.observe(setup_sun);
    app.add_systems(Update, (update_sun).run_if(in_state(Screen::Playing)));
}

static CYCLE_PEAK: f32 = 4017.75 / 2.;
static FLARE_FREQUENCY: f32 = 2.;

#[derive(Component, Debug)]
pub struct Sun {
    relative_power: f32,
    cycle_state: f32,
    increasing: bool,
    last_flare: f32,
}

impl Sun {
    pub fn power_scale(&self) -> f32 {
        self.relative_power * self.cycle_power()
    }

    fn cycle_power(&self) -> f32 {
        self.cycle_state / CYCLE_PEAK + 0.5
    }

    pub fn increment(&mut self, delta: f32) {
        self.last_flare += delta;
        self.relative_power += delta * 0.000_001;
        if self.increasing {
            self.cycle_state += delta;
            self.increasing = self.cycle_state < CYCLE_PEAK;
        } else {
            self.cycle_state -= delta;
            self.increasing = self.cycle_state < 0.;
        }
    }

    #[allow(clippy::cast_precision_loss)]
    fn should_flare(&self) -> bool {
        self.last_flare
            > (FLARE_FREQUENCY / self.power_scale() + thread_rng().gen_range(0..2) as f32)
    }

    #[allow(clippy::cast_precision_loss)]
    fn flare(&mut self) -> Option<(f32, f32)> {
        if self.should_flare() {
            let flare_scale = self.last_flare
                * self.power_scale()
                * (thread_rng().gen_range(10..200) as f32).div(100.);
            self.last_flare = 0.;
            let size_power_modifier = (thread_rng().gen_range(33..150) as f32).div(100.);
            Some((flare_scale / size_power_modifier, size_power_modifier))
        } else {
            None
        }
    }
}

impl Default for Sun {
    fn default() -> Self {
        Self {
            relative_power: 1.,
            cycle_state: CYCLE_PEAK / 2.,
            increasing: true,
            last_flare: 0.,
        }
    }
}

fn setup_sun(_trigger: Trigger<SpawnSun>, mut commands: Commands) {
    commands.spawn((
        Name::new("Sun"),
        Sun::default(),
        StateScoped(Screen::Playing),
    ));
}

fn update_sun(
    time: Res<Time>,
    mut query: Query<&mut Sun, With<Sun>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok(mut sun) = query.get_single_mut() {
        sun.increment(time.delta_seconds());
        if let Some((power, size)) = sun.flare() {
            todo!();
        }
    }
}
