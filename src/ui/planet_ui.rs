use bevy::prelude::*;
use bevy_mod_picking::prelude::PickSelection;

use sickle_ui::{prelude::*, widgets::layout::column};

use crate::{
    game::{planets::PlanetNameLabel, resources::PlanetResources, unlocks::{TechUnlocks, Technology}},
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
    selected_query: Query<(&PickSelection, Entity), With<PlanetResources>>,
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

        info! {"Spawing Planet UI!"};
        commands
            .ui_builder(UiRoot)
            .column(|column| {
                column
                    .row(|planet_name_row| {
                        planet_name_row.spawn((
                            TextBundle::from_section(
                                "Planet Name",
                                TextStyle {
                                    font_size: 30.,
                                    ..Default::default()
                                },
                            ),
                            PlanetNameLabel,
                        ));
                    })
                    .style()
                    .justify_content(JustifyContent::Center);
                // Just to provide some vertical space between the name and the resources
                column
                    .row(|spacer_row| {
                        spacer_row.spawn(NodeBundle::default());
                    })
                    .style()
                    .min_height(Val::Vh(2.5));
                // This is the one that's going to hold all the resources
                column
                    .row(|content_row| {
                        content_row.spawn((
                            NodeBundle {
                                style: Style {
                                    display: Display::Grid,
                                    height: Val::Percent(100.),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            ResourceHolderLabel,
                        ));
                    })
                    .style()
                    .height(Val::Percent(100.))
                    .width(Val::Percent(100.))
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

// FIXME: Resolve the too many lines issue by factoring this out
// TODO: Probably consider creating a custom UI widget to handle this
#[allow(clippy::too_many_lines)]
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
        // Update the planet name text
        if let Ok(mut text) = name_text_query.get_single_mut() {
            name.as_str().clone_into(&mut text.sections[0].value);
        }

        let Ok(holder) = resource_holder_query.get_single() else {
            return; // Exit if the UI is not open
        };
        let Some(mut holder_commands) = commands.get_entity(holder) else {
            return; // FIXME: Throw some kind of error or warning here. This is a problem
        };
        // Remove the previous UI elements. Not very efficient
        holder_commands.despawn_descendants(); // TODO: Consider using a more efficient aproach in
                                               // future instead of recreating it every frame
                                               // Build new UI items for the Planet UI
        commands.ui_builder(holder).column(|column| {
            for resource in &planet_resources.0 {
                let (consumed, available, unlockable) = resource.get_ratios(&techs);
                let name = resource.name().to_string();
                // Display the resource name
                column.row(|type_row| {
                    type_row.spawn(TextBundle::from_section(
                        name,
                        TextStyle {
                            font_size: 20.,
                            ..Default::default()
                        },
                    ));
                });

                // Display text to acompany the progress bar
                column
                    .row(|bar_text_row| {
                        bar_text_row.spawn(TextBundle::from_section(
                            resource.get_ratios_text(&techs),
                            TextStyle {
                                font_size: 12.,
                                ..Default::default()
                            },
                        ));
                    })
                    .style()
                    .max_width(Val::Vw(12.));

                // Display the progress-type bar showing avalibility
                column
                    .row(|bar_row| {
                        // Column for consumed resources portion of the multi-progress bar
                        let mut tmp = bar_row.column(|col| {
                            col.spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.),
                                    height: Val::Percent(100.),
                                    min_height: Val::Px(10.),
                                    ..Default::default()
                                },
                                background_color: Color::srgb(1., 0., 0.).into(),
                                ..Default::default()
                            });
                        });
                        tmp.style()
                            .height(Val::Percent(100.))
                            .width(Val::Percent(consumed * 100.));
                        if consumed > 0. {
                            tmp.style().min_width(Val::Px(1.));
                        }
                        // Column for available resources portion of the multi-progress bar
                        tmp = bar_row.column(|col| {
                            col.spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.),
                                    height: Val::Percent(100.),
                                    min_height: Val::Px(10.),
                                    ..Default::default()
                                },
                                background_color: Color::srgb(0., 1., 0.).into(),
                                ..Default::default()
                            });
                        });
                        tmp.style()
                            .height(Val::Percent(100.))
                            .width(Val::Percent(available * 100.));
                        if available > 0. {
                            tmp.style().min_width(Val::Px(1.));
                        }
                        // Column for the unlockable resources portion of the multi-progress bar
                        tmp = bar_row.column(|col| {
                            col.spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.),
                                    height: Val::Percent(100.),
                                    min_height: Val::Px(10.),
                                    ..Default::default()
                                },
                                background_color: Color::srgb(0., 0., 1.).into(),
                                ..Default::default()
                            });
                        });
                        tmp.style()
                            .height(Val::Percent(100.))
                            .width(Val::Percent(unlockable * 100.));
                        if unlockable > 0. {
                            tmp.style().min_width(Val::Px(1.));
                        }
                    })
                    .style()
                    .width(Val::Vw(12.))
                    .height(Val::Percent(100.))
                    .max_height(Val::Vh(5.));
                column
                    .row(|spacer_row| {
                        spacer_row.spawn(NodeBundle::default());
                    })
                    .style()
                    .min_height(Val::Vh(1.));
            }
        });
    }
}
