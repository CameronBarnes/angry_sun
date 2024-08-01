use bevy::prelude::*;
use bevy_mod_picking::prelude::PickSelection;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_ui);
}

#[derive(Event, Debug)]
pub struct SpawnPlanetUI;

fn spawn_ui(_trigger: Trigger<SpawnPlanetUI>, mut commands: Commands, selected_query: Query<&PickSelection, With<PickSelection>>) {

}
