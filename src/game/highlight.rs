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
pub struct HasHighlightObject(pub Entity);

#[derive(Component, Debug)]
pub struct LinkSelectObject(pub Entity);

fn handle_linked_selection_objects(
    mut ring_query: Query<(&LinkSelectObject, &mut PickSelection), With<LinkSelectObject>>,
    mut planet_query: Query<&mut PickSelection, (With<Planet>, Without<LinkSelectObject>)>,
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
    mut highlight_query: Query<&mut Visibility, With<HighlightObject>>,
    source_query: Query<(&HasHighlightObject, &PickSelection), With<HasHighlightObject>>,
) {
    // Clear existing
    for mut highlight in &mut highlight_query {
        *highlight = Visibility::Hidden;
    }

    // Enable for anything currently selected
    for (highlight_object, selection) in &source_query {
        if selection.is_selected {
            if let Ok(mut highlight) = highlight_query.get_mut(highlight_object.0) {
                *highlight = Visibility::Visible;
            }
        }
    }
}
