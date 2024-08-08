use bevy::{prelude::*, utils::HashSet};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(TechUnlocks::new());
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Technology {
    None,
    DeepSeaMining,
    DeepCrustMining,
    ExtraTerrestrialMining,
    HotSurfaceMining,
    SeaWaterElectrolysis,
    SurfaceMineralDecomposition,
    GasGiantMining,
    StellarLifting,
}

#[derive(Resource, Debug)]
pub struct TechUnlocks {
    techs: HashSet<Technology>,
}

impl TechUnlocks {
    pub fn new() -> Self {
        let mut tmp = Self {
            techs: HashSet::default(),
        };
        tmp.put(Technology::None);
        tmp
    }
}

impl TechUnlocks {
    pub fn check(&self, tech: Technology) -> bool {
        self.techs.contains(&tech)
    }

    pub fn put(&mut self, tech: Technology) {
        self.techs.insert(tech);
    }
}
