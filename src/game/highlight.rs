use bevy::prelude::*;
use bevy_mod_picking::prelude::PickSelection;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, handle_highlighting);
}

#[derive(Component, Debug)]
pub struct HighlightObject;

#[derive(Component, Debug)]
pub struct HasHighlightObject(pub Entity);

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
