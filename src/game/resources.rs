use std::fmt::Display;

use bevy::prelude::*;

use super::unlocks::TechUnlocks;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(HarvestedResources::default());
    app.add_systems(PreUpdate, update_resource_text);
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum RawResourceType {
    Metals,
    Silicate,
    Hydrogen,
    Oxygen,
    Power,
}

impl Display for RawResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Metals => write!(f, "Metals"),
            Self::Silicate => write!(f, "Silicate"),
            Self::Hydrogen => write!(f, "Hydrogen"),
            Self::Oxygen => write!(f, "Oxygen"),
            Self::Power => write!(f, "Power"),
        }
    }
}

#[derive(Debug, Component)]
pub struct ResourceLabel(pub RawResourceType);

#[derive(Resource, Debug, Default)]
pub struct HarvestedResources {
    pub metals: f32,
    pub silicate: f32,
    pub hydrogen: f32,
    pub oxygen: f32,
    pub power: f32,
}

#[derive(Component, Debug)]
pub struct PlanetResources(pub Vec<RawResource>);

#[derive(Debug)]
pub struct RawResource {
    resource_type: RawResourceType,
    levels: Vec<(f32, &'static str)>,
    consumed: f32,
}

impl RawResource {
    pub const fn new(res_type: RawResourceType, levels: Vec<(f32, &'static str)>) -> Self {
        Self {
            resource_type: res_type,
            levels,
            consumed: 0.,
        }
    }

    pub const fn name(&self) -> RawResourceType {
        self.resource_type
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

    fn get_last(&self) -> f32 {
        self.levels.last().expect("Minimum 1 level").0
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

    /// Gets the percentage of each (consumed, avalible, unlockable) as a f32 between 0.0 and 1.0
    pub fn get_ratios(&self, techs: &TechUnlocks) -> (f32, f32, f32) {
        let Some(current_unlocked) = self.get_current(techs) else {
            return (0., 0., 1.);
        };
        let max_unlockable = self.get_last();
        (
            self.consumed / max_unlockable,
            (current_unlocked - self.consumed) / max_unlockable,
            (max_unlockable - current_unlocked) / max_unlockable,
        )
    }
}

fn update_resource_text(
    resources: Res<HarvestedResources>,
    mut text_query: Query<(&mut Text, &ResourceLabel), With<ResourceLabel>>,
) {
    for (mut text, res_type) in &mut text_query {
        match res_type.0 {
            RawResourceType::Metals => text.sections[0].value = resources.metals.to_string(),
            RawResourceType::Silicate => text.sections[0].value = resources.silicate.to_string(),
            RawResourceType::Hydrogen => text.sections[0].value = resources.hydrogen.to_string(),
            RawResourceType::Oxygen => text.sections[0].value = resources.oxygen.to_string(),
            RawResourceType::Power => text.sections[0].value = resources.power.to_string(),
        }
    }
}
