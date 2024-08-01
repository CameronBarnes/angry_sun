use bevy::prelude::*;
use bevy_mod_picking::prelude::PickSelection;

use super::planets::Planet;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        PreUpdate,
        (handle_linked_selection_objects, handle_highlighting).chain(),
    );
}

#[derive(Component, Debug)]
pub struct HighlightObject;

#[derive(Component, Debug)]
pub struct LinkSelectionObject(pub Entity);

fn handle_linked_selection_objects(
    mut ring_query: Query<(&LinkSelectionObject, &mut PickSelection), With<LinkSelectionObject>>,
    mut planet_query: Query<&mut PickSelection, (With<Planet>, Without<LinkSelectionObject>)>,
) {
    for (link, mut selected) in &mut ring_query {
        if selected.is_selected {
            if let Ok(mut planet) = planet_query.get_mut(link.0) {
                planet.is_selected = true;
                planet.set_changed();
                selected.is_selected = false;
                selected.set_changed();
            }
        }
    }
}

fn handle_highlighting(
    mut highlight_query: Query<(&mut Visibility, &Parent), With<HighlightObject>>,
    source_query: Query<&PickSelection>,
) {
    for (mut highlight, parent) in &mut highlight_query {
        if let Ok(selection) = source_query.get(parent.get()) {
            if selection.is_selected {
                *highlight = Visibility::Visible;
            } else {
                *highlight = Visibility::Hidden;
            }
        } else {
            *highlight = Visibility::Hidden;
        }
    }
}
