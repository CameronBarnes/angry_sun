use bevy::prelude::*;

use super::unlocks::TechUnlocks;

#[derive(Component, Debug)]
pub struct PlanetResources(pub Vec<RawResource>);

#[derive(Debug)]
pub struct RawResource {
    name: &'static str,
    levels: Vec<(f32, &'static str)>,
    consumed: f32,
}

impl RawResource {
    pub const fn new(name: &'static str, levels: Vec<(f32, &'static str)>) -> Self {
        Self {
            name,
            levels,
            consumed: 0.,
        }
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub fn apply_scale(&mut self, size: f32) {
        self.levels.iter_mut().for_each(|pair| pair.0 *= size);
    }

    fn get_current(&self, techs: &TechUnlocks) -> Option<f32> {
        if !techs.check(self.levels[0].1) {
            return None;
        }
        self.levels
            .iter()
            .filter(|(_value, tech)| techs.check(tech))
            .last()
            .map(|(value, _tech)| *value)
    }

    pub const fn get_consumed(&self) -> f32 {
        self.consumed
    }

    pub fn get_available(&self, techs: &TechUnlocks) -> f32 {
        self.get_current(techs)
            .map_or(0., |available| available - self.consumed)
    }

    pub fn get_next(&self, techs: &TechUnlocks) -> Option<&'static str> {
        for (_, tech) in &self.levels {
            if !techs.check(tech) {
                return Some(*tech);
            }
        }
        None
    }

    pub fn increment_consumed(&mut self, consumed: f32) {
        self.consumed += consumed;
    }
}
