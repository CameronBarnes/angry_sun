use bevy::{prelude::*, reflect::List};
use bevy_mod_picking::prelude::PickSelection;

use sickle_ui::{prelude::*, widgets::layout::column};

use crate::{
    format_number,
    game::{
        planets::PlanetNameLabel,
        resources::{BuiltHarvesters, PlanetResourceLabel, PlanetResources},
        unlocks::{TechUnlocks, Technology},
    },
    screen::Screen,
};

use super::{
    multi_progress_bar::MultiProgressBar,
    palette::{BUTTON_HOVERED_BACKGROUND, BUTTON_PRESSED_BACKGROUND, NODE_BACKGROUND},
    prelude::InteractionPalette,
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
    selected_planet_query: Query<
        (
            &PickSelection,
            &Name,
            &PlanetResources,
            &BuiltHarvesters,
            Entity,
        ),
        With<PickSelection>,
    >,
    resource_holder_query: Query<Entity, With<ResourceHolderLabel>>,
) {
    if let Some((_, name, planet_resources, planet_harvesters, planet_entity)) =
        selected_planet_query
            .iter()
            .find(|(selection, _, _, _, _)| selection.is_selected)
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

        // Create button color component
        let button_palette = InteractionPalette {
            none: NODE_BACKGROUND,
            hovered: BUTTON_HOVERED_BACKGROUND,
            pressed: BUTTON_PRESSED_BACKGROUND,
        };

        let button_palette_disabled = InteractionPalette {
            none: NODE_BACKGROUND.with_luminance(0.2),
            hovered: BUTTON_HOVERED_BACKGROUND.with_luminance(0.15),
            pressed: BUTTON_PRESSED_BACKGROUND.with_luminance(0.15),
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
                        let tmp = MultiProgressBar::spawn(
                            bar_row.entity_commands(),
                            vec![
                                (consumed * 100., Color::srgb(1., 0., 0.)),
                                (available * 100., Color::srgb(0., 1., 0.)),
                                (unlockable * 100., Color::srgb(0., 0., 1.)),
                            ],
                        );
                        bar_row
                            .commands()
                            .get_entity(tmp)
                            .expect("Just created, should be valid")
                            .insert(PlanetResourceLabel(planet_entity, resource.name()));
                    })
                    .style()
                    .width(Val::Vw(15.))
                    .height(Val::Percent(100.))
                    .max_height(Val::Vh(5.));
                // Cost and buy button
                // TODO: Handle buying more than one
                column.row(|cost_buy_row| {
                    // Cost text
                    cost_buy_row.column(|cost_col| {
                        let mut cost = resource.cost(&techs);
                        #[allow(clippy::cast_precision_loss)]
                        let modifier = planet_harvesters
                            .0
                            .get(&resource.name())
                            .map_or(1., |vec| 1. + (vec.len() as f32 / 1000.));
                        cost.0 *= modifier;
                        cost.1 *= modifier;
                        cost_col.row(|metal_row| {
                            metal_row.spawn(TextBundle::from_section(
                                format!("{} Metal", format_number(cost.0)),
                                TextStyle {
                                    font_size: 10.,
                                    ..Default::default()
                                },
                            ));
                        });
                        cost_col.row(|silicate_row| {
                            silicate_row.spawn(TextBundle::from_section(
                                format!("{} Silicate", format_number(cost.1)),
                                TextStyle {
                                    font_size: 10.,
                                    ..Default::default()
                                },
                            ));
                        });
                    });
                    // Place buy button
                    cost_buy_row
                        .column(|button_col| {
                            button_col
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            justify_content: JustifyContent::Center,
                                            ..Default::default()
                                        },
                                        background_color: button_palette.none.into(),
                                        ..Default::default()
                                    },
                                    button_palette.clone(),
                                ))
                                .entity_commands()
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Buy",
                                        TextStyle {
                                            font_size: 18.,
                                            ..Default::default()
                                        },
                                    ));
                                });
                        })
                        .style()
                        .width(Val::Percent(100.))
                        .justify_content(JustifyContent::Center);
                });
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
