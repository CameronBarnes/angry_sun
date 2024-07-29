use std::ops::Div;

use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::screen::Screen;

use super::flare::SpawnFlare;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PreUpdate, update_sun_labels);
    app.add_systems(Update, (update_sun).run_if(in_state(Screen::Playing)));
}

#[derive(Debug, Component)]
pub struct SunCycleLabel;

#[derive(Debug, Component)]
pub struct SunPowerLabel;

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

    pub fn cycle_power(&self) -> f32 {
        self.raw_cycle_state() + 0.5
    }

    pub fn raw_cycle_state(&self) -> f32 {
        self.cycle_state / CYCLE_PEAK
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

fn update_sun(time: Res<Time>, mut query: Query<&mut Sun, With<Sun>>, mut commands: Commands) {
    if let Ok(mut sun) = query.get_single_mut() {
        sun.increment(time.delta_seconds());
        if let Some((power, size)) = sun.flare() {
            commands.trigger(SpawnFlare { power, size });
        }
    }
}

fn update_sun_labels(
    sun_query: Query<&Sun, With<Sun>>,
    mut sun_power_query: Query<&mut Text, With<SunPowerLabel>>,
    mut sun_cycle_query: Query<&mut Text, (With<SunCycleLabel>, Without<SunPowerLabel>)>,
) {
    if let Ok(sun) = sun_query.get_single() {
        if let Ok(mut power_label) = sun_power_query.get_single_mut() {
            power_label.sections[0].value = format!("{:.4}", sun.power_scale());
        }

        if let Ok(mut cycle_label) = sun_cycle_query.get_single_mut() {
            cycle_label.sections[0].value = format!("{:.4}", sun.raw_cycle_state());
        }
    }
}
