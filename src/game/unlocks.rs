use bevy::{prelude::*, utils::HashSet};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(TechUnlocks::new());
}

#[derive(Resource, Debug)]
pub struct TechUnlocks {
    techs: HashSet<&'static str>,
}

impl TechUnlocks {
    pub fn new() -> Self {
        let mut tmp = Self {
            techs: HashSet::default(),
        };
        tmp.put("None");
        tmp
    }
}

impl TechUnlocks {
    pub fn check(&self, tech: &'static str) -> bool {
        self.techs.contains(tech)
    }

    pub fn put(&mut self, tech: &'static str) {
        self.techs.insert(tech);
    }
}
