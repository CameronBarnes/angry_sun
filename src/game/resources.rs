use std::vec::Vec;

use bevy::{prelude::*, utils::HashMap};
use derive_more::derive::Display;

use crate::{format_number, screen::Screen, ui::multi_progress_bar::MultiProgressBar};

use super::{
    spawn::planets::ONE_AU,
    sun::Sun,
    unlocks::{TechUnlocks, Technology},
};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(HarvestedResources::default());
    app.add_systems(
        PreUpdate,
        (
            update_resource_text,
            update_planet_ui_resource_bar,
            update_planet_resource_buy_cost_labels,
            update_resource_bar_text_label,
        ),
    );
    app.add_systems(
        Update,
        (consuming_structures, producing_structures)
            .chain()
            .run_if(in_state(Screen::Playing)),
    );
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Display)]
pub enum RawResourceType {
    Metals,
    Silicate,
    Hydrogen,
    Oxygen,
    Power,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Display)]
pub enum StationType {
    Surface,
    Ocean,
    Orbit,
}

impl StationType {
    pub const fn cost(self) -> (f32, f32) {
        match self {
            Self::Surface => (100., 150.),
            Self::Ocean => (200., 200.),
            Self::Orbit => (400., 50.),
        }
    }
}

#[derive(Debug, Component, Clone, Copy, Eq, PartialEq)]
pub struct ResourceLabel(pub RawResourceType);

#[derive(Debug, Component, Clone, PartialEq, Eq)]
pub struct PlanetResourceLabel(pub Entity, pub RawResourceType);

#[derive(Debug, Component, Clone, Copy, Eq, PartialEq)]
pub struct ResourceBarTextLabel;

#[derive(Debug, Component, Clone, PartialEq, Eq)]
pub struct ResourceCostLabel(pub RawResourceType);

#[derive(Resource, Debug, Default)]
pub struct HarvestedResources {
    pub metals: f32,
    pub silicate: f32,
    pub hydrogen: f32,
    pub oxygen: f32,
    pub power: f32,
}

impl HarvestedResources {
    pub const fn get(&self, res_type: RawResourceType) -> f32 {
        match res_type {
            RawResourceType::Metals => self.metals,
            RawResourceType::Silicate => self.silicate,
            RawResourceType::Hydrogen => self.hydrogen,
            RawResourceType::Oxygen => self.oxygen,
            RawResourceType::Power => self.power,
        }
    }

    pub fn get_mut(&mut self, res_type: RawResourceType) -> &mut f32 {
        match res_type {
            RawResourceType::Metals => &mut self.metals,
            RawResourceType::Silicate => &mut self.silicate,
            RawResourceType::Hydrogen => &mut self.hydrogen,
            RawResourceType::Oxygen => &mut self.oxygen,
            RawResourceType::Power => &mut self.power,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct PlanetResources {
    resources: Vec<RawResource>,
}

impl PlanetResources {
    pub const fn new(resources: Vec<RawResource>) -> Self {
        Self { resources }
    }

    pub fn get(&self, resource_type: RawResourceType) -> Option<&RawResource> {
        self.resources
            .iter()
            .find(|res| res.name() == resource_type)
    }

    pub fn get_mut(&mut self, resource_type: RawResourceType) -> Option<&mut RawResource> {
        self.resources
            .iter_mut()
            .find(|res| res.name() == resource_type)
    }

    pub fn slice(&self) -> &[RawResource] {
        &self.resources
    }

    pub fn apply_scale(&mut self, size: f32) {
        self.resources
            .iter_mut()
            .for_each(|res| res.apply_scale(size));
    }
}

#[derive(Debug, Clone)]
pub struct RawResource {
    resource_type: RawResourceType,
    station_type: StationType,
    levels: Vec<(f32, Technology)>,
    consumed: f32,
}

impl RawResource {
    pub const fn new(
        resource_type: RawResourceType,
        station_type: StationType,
        levels: Vec<(f32, Technology)>,
    ) -> Self {
        Self {
            resource_type,
            station_type,
            levels,
            consumed: 0.,
        }
    }

    /// Return the `RawResourceType`
    pub const fn name(&self) -> RawResourceType {
        self.resource_type
    }

    /// Scale the provided percentage values from creation (0.0-1.0) with the planet size
    pub fn apply_scale(&mut self, size: f32) {
        self.levels.iter_mut().for_each(|pair| pair.0 *= size);
    }

    fn get_current(&self, techs: &TechUnlocks) -> Option<f32> {
        if !techs.check(self.levels[0].1) {
            return None;
        }
        self.levels
            .iter()
            .filter(|(_value, tech)| techs.check(*tech))
            .last()
            .map(|(value, _tech)| *value)
    }

    /// Returns the maximum unlockable resource amount from the last resource level
    fn get_last(&self) -> f32 {
        self.levels.last().expect("Minimum 1 level").0
    }

    /// Returns the amount of this resource that has already been consumed
    pub const fn get_consumed(&self) -> f32 {
        self.consumed
    }

    /// Returns the currently available resource to harvest
    pub fn get_available(&self, techs: &TechUnlocks) -> f32 {
        self.get_current(techs)
            .map_or(0., |available| available - self.get_consumed())
    }

    /// Returns the next technology to unlock for this resource
    pub fn get_next(&self, techs: &TechUnlocks) -> Option<Technology> {
        for (_, tech) in &self.levels {
            if !techs.check(*tech) {
                return Some(*tech);
            }
        }
        None
    }

    /// Increase the amount of this resource that has been consumed
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

    /// Returns formatted text displaying the percentages of resources consumed, available, and
    /// unlockable
    pub fn get_ratios_text(&self, techs: &TechUnlocks) -> String {
        let (consumed, available, unlockable) = self.get_ratios(techs);
        let mut out = if consumed > 0. {
            format!(
                "{}% Consumed\n{}% Available",
                format_number(consumed * 100.),
                format_number(available * 100.)
            )
        } else {
            format!("{}% Available", format_number(available * 100.))
        };
        if unlockable > 0. {
            out.push_str(&format!(
                "\n{}% Unlockable",
                format_number(unlockable * 100.)
            ));
        }
        out
    }

    /// Returns the cost of building another harvester for this resource
    pub fn cost(&self, techs: &TechUnlocks) -> (f32, f32) {
        let mut cost = self.station_type.cost();
        let mut first = true; // We need this because if the first tech is not unlocked then it'll
                              // just show the base cost for that station type, which is wrong
        for (_, tech) in &self.levels {
            if first || techs.check(*tech) {
                let modifier = tech.cost_modifier();
                cost.0 *= modifier;
                cost.1 *= modifier;
                first = false;
            } else {
                break;
            }
        }
        cost
    }
}

#[derive(Debug, Component)]
pub struct ProducingStructure {
    pub planet: Option<Entity>,
    pub res_type: RawResourceType,
    pub produced: f32,
    pub sun_buff: f32,
}

#[allow(clippy::cast_precision_loss)]
pub fn cost_calculator(cost: f32, number: usize, mult: f32) -> f32 {
    cost * mult.powf(number as f32)
}

#[derive(Debug, Component, Default)]
pub struct BuiltHarvesters(pub HashMap<RawResourceType, Vec<Entity>>);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EnabledStructure(pub bool);

trait Requirement {
    fn check(&self, resources: &HarvestedResources) -> bool;
    fn consume(&self, resources: &mut HarvestedResources) -> bool;
}

#[derive(Component, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PoweredStructure(pub f32);

impl Requirement for PoweredStructure {
    fn check(&self, resources: &HarvestedResources) -> bool {
        resources.power >= self.0
    }

    fn consume(&self, resources: &mut HarvestedResources) -> bool {
        if self.check(resources) {
            resources.power -= self.0;
            true
        } else {
            false
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct ConsumingStructure(pub Vec<(f32, RawResourceType)>);

impl Requirement for ConsumingStructure {
    fn check(&self, resources: &HarvestedResources) -> bool {
        for (amount, resource) in &self.0 {
            if resources.get(*resource) < *amount {
                return false;
            }
        }
        true
    }

    fn consume(&self, resources: &mut HarvestedResources) -> bool {
        if self.check(resources) {
            for (amount, resource) in &self.0 {
                *resources.get_mut(*resource) -= *amount;
            }
            true
        } else {
            false
        }
    }
}

fn consuming_structures(
    tech: Res<TechUnlocks>,
    sun: Query<&Sun>,
    planet_resources_query: Query<&PlanetResources>,
    mut resources: ResMut<HarvestedResources>,
    mut structure_query: Query<(
        Option<&PoweredStructure>,
        Option<&ConsumingStructure>,
        Option<&ProducingStructure>,
        &mut EnabledStructure,
        &GlobalTransform,
    )>,
) {
    let Ok(sun) = sun.get_single() else {
        return; // TODO: Probably return an error here
    };

    for (power, consumed_res, producing, mut enabled, transform) in &mut structure_query {
        let has_resource = producing.map_or(true, |producing| {
            producing
                .planet
                .and_then(|entity| planet_resources_query.get(entity).ok())
                .map_or_else(
                    || true,
                    |resources| {
                        let produced = if producing.sun_buff == 0. {
                            producing.produced
                        } else {
                            producing.produced
                                * ((producing.sun_buff * sun.power_scale())
                                    / (transform.translation().distance(Vec3::ZERO) / *ONE_AU))
                        };
                        resources
                            .get(producing.res_type)
                            .map_or(true, |res| res.get_available(&tech) >= produced)
                    },
                )
        });
        if power.is_some() || consumed_res.is_some() {
            if has_resource
                && power.map_or(true, |power| power.check(&resources))
                && consumed_res.map_or(true, |consumed| consumed.check(&resources))
            {
                if let Some(power) = power {
                    power.consume(&mut resources);
                }
                if let Some(consumed_res) = consumed_res {
                    consumed_res.consume(&mut resources);
                }
                enabled.0 = true;
            } else {
                enabled.0 = false;
            }
        } else {
            panic!("EnabledStructure does not have PoweredStructure or ConsumingStructure. One of the two is required.");
        }
    }
}

fn producing_structures(
    tech: Res<TechUnlocks>,
    sun: Query<&Sun>,
    mut resources: ResMut<HarvestedResources>,
    structure_query: Query<(
        Option<&EnabledStructure>,
        &ProducingStructure,
        &GlobalTransform,
    )>,
    mut planet_resources_query: Query<&mut PlanetResources>,
) {
    let Ok(sun) = sun.get_single() else {
        return; // TODO: Probably throw an error here
    };

    for (_, producing, transform) in structure_query
        .iter()
        .filter(|(enabled, _, _)| enabled.map_or(true, |enabled| enabled.0))
    {
        let produced = if producing.sun_buff == 0. {
            producing.produced
        } else {
            producing.produced
                * ((producing.sun_buff * sun.power_scale())
                    / (transform.translation().distance(Vec3::ZERO) / *ONE_AU))
        };
        if let Some(mut planet_res) = producing
            .planet
            .and_then(|entity| planet_resources_query.get_mut(entity).ok())
        {
            if let Some(res) = planet_res.get_mut(producing.res_type) {
                if res.get_available(&tech) >= produced {
                    res.increment_consumed(produced);
                }
            }
        }
        *resources.get_mut(producing.res_type) += produced;
    }
}

// UI stuff bellow here

/// Update the Resource bar at the top of the screen
fn update_resource_text(
    resources: Res<HarvestedResources>,
    mut text_query: Query<(&mut Text, &ResourceLabel), With<ResourceLabel>>,
) {
    for (mut text, res_type) in &mut text_query {
        text.sections[0].value = format_number(resources.get(res_type.0));
    }
}

/// Update the ``MultiProgressBar``'s for planet resources
pub fn update_planet_ui_resource_bar(
    tech: Res<TechUnlocks>,
    planet_query: Query<&PlanetResources>,
    mut bar_query: Query<(&mut MultiProgressBar, &PlanetResourceLabel)>,
) {
    for (mut bar, label) in &mut bar_query {
        if let Ok(resources) = planet_query.get(label.0) {
            if let Some(resource) = resources.get(label.1) {
                let (consumed, available, unlockable) = resource.get_ratios(&tech);
                let vals = bar.get_values_mut();
                vals[0] = consumed * 100.;
                vals[1] = available * 100.;
                vals[2] = unlockable * 100.;
            }
        }
    }
}

fn update_resource_bar_text_label(
    tech: Res<TechUnlocks>,
    planet_query: Query<&PlanetResources>,
    mut label_query: Query<(&mut Text, &PlanetResourceLabel), With<ResourceBarTextLabel>>,
) {
    for (mut text, planet_res) in &mut label_query {
        if let Ok(resources) = planet_query.get(planet_res.0) {
            if let Some(resource) = resources.get(planet_res.1) {
                text.sections[0].value = resource.get_ratios_text(&tech);
            }
        }
    }
}

fn update_planet_resource_buy_cost_labels(
    tech: Res<TechUnlocks>,
    planet_query: Query<(&PlanetResources, &BuiltHarvesters)>,
    mut label_query: Query<(&mut Text, &PlanetResourceLabel, &ResourceCostLabel)>,
) {
    for (mut text, planet_res, cost_type) in &mut label_query {
        if let Ok((resources, harvesters)) = planet_query.get(planet_res.0) {
            if let Some(resource) = resources.get(planet_res.1) {
                let mut cost = resource.cost(&tech);
                let modifier = harvesters.0.get(&resource.name()).map_or(0, Vec::len);
                cost.0 = cost_calculator(cost.0, modifier, 0.001);
                cost.1 = cost_calculator(cost.1, modifier, 0.001);
                match cost_type.0 {
                    RawResourceType::Metals => text.sections[0].value = format_number(cost.0),
                    RawResourceType::Silicate => text.sections[0].value = format_number(cost.1),
                    _ => unimplemented!(),
                }
            }
        }
    }
}
