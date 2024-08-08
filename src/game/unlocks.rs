use bevy::{prelude::*, utils::HashSet};
use convert_case::{Case, Casing};
use derive_more::derive::Display;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(TechUnlocks::default());
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, Display)]
pub enum Technology {
    None,
    Orbitals,
    DeepSeaMining,
    DeepCrustMining,
    ExtraTerrestrialMining,
    HotSurfaceMining,
    SeaWaterElectrolysis,
    SurfaceMineralDecomposition,
    GasGiantMining,
    StellarLifting,
}

impl Technology {
    pub const fn cost_modifier(self) -> f32 {
        match self {
            Self::None | Self::GasGiantMining | Self::Orbitals => 1.,
            Self::DeepSeaMining | Self::SurfaceMineralDecomposition => 2.,
            Self::DeepCrustMining => 5.,
            Self::ExtraTerrestrialMining | Self::SeaWaterElectrolysis => 1.5,
            Self::HotSurfaceMining => 2.5,
            Self::StellarLifting => 10_000.,
        }
    }

    pub const fn is_none(self) -> bool {
        matches!(self, Self::None)
    }

    pub fn to_formatted_string(self) -> String {
        self.to_string().to_case(Case::Title)
    }

    pub fn prerequisites(self) -> Vec<Self> {
        match self {
            Self::None
            | Self::DeepSeaMining
            | Self::DeepCrustMining
            | Self::ExtraTerrestrialMining
            | Self::SeaWaterElectrolysis
            | Self::Orbitals => vec![],
            Self::HotSurfaceMining | Self::SurfaceMineralDecomposition => {
                vec![Self::ExtraTerrestrialMining]
            }
            Self::GasGiantMining => vec![Self::Orbitals],
            Self::StellarLifting => vec![Self::GasGiantMining],
        }
    }
}

#[derive(Resource, Debug, Default)]
pub struct TechUnlocks {
    techs: HashSet<Technology>,
}

impl TechUnlocks {
    pub fn check(&self, tech: Technology) -> bool {
        tech.is_none() || self.techs.contains(&tech)
    }

    pub fn put(&mut self, tech: Technology) {
        self.techs.insert(tech);
    }
}
