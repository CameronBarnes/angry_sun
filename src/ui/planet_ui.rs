use bevy::prelude::*;
use bevy_mod_picking::prelude::PickSelection;

use sickle_ui::{prelude::*, widgets::layout::column};

use crate::{
    game::{planets::PlanetNameLabel, resources::PlanetResources, unlocks::TechUnlocks},
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (spawn_ui, update_ui).chain());
}

#[derive(Component, Debug)]
pub struct PlanetUI(Entity);

#[derive(Component, Debug)]
pub struct ResourceHolderLabel;

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
        commands
            .ui_builder(UiRoot)
            .column(|column| {
                column
                    .row(|row| {
                        row.spawn((
                            TextBundle::from_section(
                                "Planet Name",
                                TextStyle {
                                    font_size: 20.,
                                    ..Default::default()
                                },
                            ),
                            PlanetNameLabel,
                        ));
                    })
                    .style()
                    .top(Val::Percent(5.))
                    .justify_content(JustifyContent::Center);
                // This is the one that's going to hold all the resources
                column
                    .row(|row| {
                        row.insert(ResourceHolderLabel);
                    })
                    .style()
                    .justify_content(JustifyContent::Center);
            })
            .insert((PlanetUI(entity), StateScoped(Screen::Playing)))
            .style()
            .height(Val::Percent(90.))
            .width(Val::Percent(20.))
            .top(Val::Percent(10.))
            .left(Val::Percent(80.))
            .background_color(Color::WHITE.with_alpha(0.2));
    } else if let Some(commands) = existing_ui_query
        .get_single()
        .ok()
        .and_then(|(entity, _)| commands.get_entity(entity))
    {
        commands.despawn_recursive();
    }
}

fn update_ui(
    mut commands: Commands,
    techs: Res<TechUnlocks>,
    mut name_text_query: Query<&mut Text, With<PlanetNameLabel>>,
    selected_planet_query: Query<(&PickSelection, &Name, &PlanetResources), With<PickSelection>>,
    resource_holder_query: Query<Entity, With<ResourceHolderLabel>>,
) {
    if let Some((_, name, planet_resources)) = selected_planet_query
        .iter()
        .find(|(selection, _, _)| selection.is_selected)
    {
        if let Ok(mut text) = name_text_query.get_single_mut() {
            name.as_str().clone_into(&mut text.sections[0].value);
        }

        if let Ok(holder) = resource_holder_query.get_single() {
            if let Some(mut holder_commands) = commands.get_entity(holder) {
                holder_commands.despawn_descendants(); // TODO: Consider using a more efficient aproach in
                                                       // future instead of recreating it every frame
                let mut holder = commands.ui_builder(holder);
                for resource in &planet_resources.0 {
                    let (consumed, available, unlockable) = resource.get_ratios(&techs);
                    let name = resource.name().to_string();
                    holder.row(|type_row| {
                        type_row.label(LabelConfig::from(name));
                    });
                    holder.row(|bar_row| {
                        bar_row
                            .column(|_| {})
                            .style()
                            .height(Val::Percent(1.))
                            .width(Val::Percent(consumed * 100.))
                            .background_color(Color::srgb(1., 0., 0.));
                        bar_row
                            .column(|_| {})
                            .style()
                            .height(Val::Percent(1.))
                            .width(Val::Percent(available * 100.))
                            .background_color(Color::srgb(0., 1., 0.));
                        bar_row
                            .column(|_| {})
                            .style()
                            .height(Val::Percent(1.))
                            .width(Val::Percent(unlockable * 100.))
                            .background_color(Color::srgb(0., 0., 1.));
                    });
                }
            }
        }
    }
}
