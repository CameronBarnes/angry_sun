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
            Self::StellarLifting => 2_000.,
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

    pub const fn cost(self) -> (f32, f32) {
        match self {
            Self::None => (0., 0.),
            Self::Orbitals => (5_000., 300.),
            Self::DeepSeaMining => (1_000., 500.),
            Self::DeepCrustMining => (20_000., 20_000.),
            Self::ExtraTerrestrialMining => (5_000., 2_000.),
            Self::HotSurfaceMining => (5_000., 10_000.),
            Self::SeaWaterElectrolysis => (2_000., 300.),
            Self::SurfaceMineralDecomposition => (5_000., 500.),
            Self::GasGiantMining => (30_000., 3_000.),
            Self::StellarLifting => (200_000., 50_000.),
        }
    }
}

#[derive(Resource, Debug, Default)]
pub struct TechUnlocks {
    techs: HashSet<Technology>,
}

impl TechUnlocks {
    /// Returns true if the specified technology is currently unlocked
    pub fn check(&self, tech: Technology) -> bool {
        tech.is_none() || self.techs.contains(&tech)
    }

    /// Returns true if the pre-requisite technologies for the provided tech are unlocked
    pub fn can_unlock(&self, tech: Technology) -> bool {
        tech.prerequisites()
            .into_iter()
            .all(|perquisite| self.check(perquisite))
    }

    /// Unlocks the provided technology if the prerequisites are met, returns a bool indicating
    /// success
    pub fn unlock(&mut self, tech: Technology) -> bool {
        if self.can_unlock(tech) {
            self.techs.insert(tech);
            true
        } else {
            false
        }
    }
}
