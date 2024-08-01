use bevy::prelude::*;
use bevy_mod_picking::prelude::PickSelection;

use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, spawn_ui);
}

#[derive(Component, Debug)]
pub struct PlanetUI(Entity);

fn spawn_ui(
    mut commands: Commands,
    existing_ui_query: Query<(Entity, &PlanetUI)>,
    selected_query: Query<(&PickSelection, Entity), With<PickSelection>>,
) {
    if let Some((_, entity)) = selected_query
        .iter()
        .find(|(selection, _)| selection.is_selected)
    {
        // If there is already UI built for that planet, then exit, otherwise remove the prev UI
        if let Ok((prev_entity, prev_ui)) = existing_ui_query.get_single() {
            if prev_ui.0 == entity {
                return;
            } else if let Some(commands) = commands.get_entity(prev_entity) {
                commands.despawn_recursive();
            }
        }

        info! {"Spawing UI!"};
        commands.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(10.),
                    height: Val::Percent(90.),
                    align_self: AlignSelf::End,
                    display: Display::Grid,
                    grid_template_columns: vec![
                        GridTrack::max_content(),
                        GridTrack::auto(),
                        GridTrack::auto(),
                    ],
                    grid_template_rows: vec![GridTrack::flex(1.)],
                    ..Default::default()
                },
                //background_color: Color::srgb(0.45, 0.45, 0.45).with_alpha(0.55).into(),
                ..Default::default()
            },
            PlanetUI(entity),
            StateScoped(Screen::Playing),
        ));
    } else if let Some(commands) = existing_ui_query
        .get_single()
        .ok()
        .and_then(|(entity, _)| commands.get_entity(entity))
    {
        commands.despawn_recursive();
    }
}
