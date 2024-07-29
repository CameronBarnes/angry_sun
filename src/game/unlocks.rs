use bevy::{prelude::Resource, utils::HashSet};

#[derive(Resource, Debug, Default)]
pub struct TechUnlocks {
    techs: HashSet<&'static str>,
}

impl TechUnlocks {
    pub fn check(&self, tech: &'static str) -> bool {
        self.techs.contains(tech)
    }

    pub fn put(&mut self, tech: &'static str) {
        self.techs.insert(tech);
    }
}
