use bevy::{prelude::*, reflect::List};
use bevy_mod_picking::{prelude::PickSelection, selection::NoDeselect, PickableBundle};

use sickle_ui::{prelude::*, widgets::layout::column};

use crate::{
    format_number,
    game::{
        highlight::LinkSelectionObject,
        planets::PlanetNameLabel,
        resources::{
            self, update_planet_ui_resource_bar, BuiltHarvesters, HarvestedResources,
            PlanetResourceLabel, PlanetResources, RawResourceType, ResourceBarTextLabel,
            ResourceCostLabel,
        },
        sun::Sun,
        unlocks::{TechUnlocks, Technology},
    },
    screen::Screen,
    ui::palette::BUTTON_PALETTE,
};

use super::{
    multi_progress_bar::MultiProgressBar,
    palette::{BUTTON_HOVERED_BACKGROUND, BUTTON_PRESSED_BACKGROUND, NODE_BACKGROUND},
    prelude::InteractionPalette,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        PreUpdate,
        (spawn_ui, update_ui_name, update_buy_button)
            .chain()
            .before(update_planet_ui_resource_bar)
            .run_if(in_state(Screen::Playing)),
    );
}

#[derive(Component, Debug)]
pub struct PlanetUI(Entity);

#[derive(Component, Debug)]
pub struct ResourceHolderLabel;

// FIXME: Resolve the too many lines issue by factoring this out
// TODO: Probably consider creating a custom UI widget to handle this
#[allow(clippy::too_many_lines)]
fn spawn_ui(
    mut commands: Commands,
    existing_ui_query: Query<(Entity, &PlanetUI)>,
    selected_planet_query: Query<(&PickSelection, &PlanetResources, Entity), With<PickSelection>>,
) {
    if let Some((_, planet_resources, planet_entity)) = selected_planet_query
        .iter()
        .find(|(selection, _, _)| selection.is_selected)
    {
        // If there is already UI built for that planet, then exit, otherwise remove the prev UI
        if let Ok((prev_entity, prev_ui)) = existing_ui_query.get_single() {
            if prev_ui.0 == planet_entity {
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
                            NoDeselect,
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
                        content_row.column(|column| {
                            for resource in planet_resources.slice() {
                                // Display the resource name
                                column.row(|type_row| {
                                    type_row.spawn((
                                        TextBundle::from_section(
                                            resource.name().to_string(),
                                            TextStyle {
                                                font_size: 20.,
                                                ..Default::default()
                                            },
                                        ),
                                        NoDeselect,
                                    ));
                                });

                                // Display text to acompany the progress bar
                                column
                                    .row(|bar_text_row| {
                                        bar_text_row.spawn((
                                            TextBundle::from_section(
                                                "Bar Text",
                                                TextStyle {
                                                    font_size: 12.,
                                                    ..Default::default()
                                                },
                                            ),
                                            PlanetResourceLabel(planet_entity, resource.name()),
                                            ResourceBarTextLabel,
                                        ));
                                    })
                                    .style()
                                    .max_width(Val::Vw(12.));

                                // Display the progress-type bar showing avalibility
                                column
                                    .row(|bar_row| {
                                        let tmp = MultiProgressBar::spawn_with_colors(
                                            bar_row.entity_commands(),
                                            vec![
                                                Color::srgb(1., 0., 0.),
                                                Color::srgb(0., 1., 0.),
                                                Color::srgb(0., 0., 1.),
                                            ],
                                        );
                                        bar_row
                                            .commands()
                                            .get_entity(tmp)
                                            .expect("Just created, should be valid")
                                            .insert((
                                                PlanetResourceLabel(planet_entity, resource.name()),
                                                NoDeselect,
                                            ));
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
                                        cost_col.row(|metal_row| {
                                            metal_row.column(|dynamic_label| {
                                                dynamic_label.spawn((
                                                    TextBundle::from_section(
                                                        "Cost",
                                                        TextStyle {
                                                            font_size: 10.,
                                                            ..Default::default()
                                                        },
                                                    ),
                                                    PlanetResourceLabel(
                                                        planet_entity,
                                                        resource.name(),
                                                    ),
                                                    ResourceCostLabel(RawResourceType::Metals),
                                                    NoDeselect,
                                                ));
                                            });
                                            metal_row.column(|static_label| {
                                                static_label.spawn((
                                                    TextBundle::from_section(
                                                        "  Metal",
                                                        TextStyle {
                                                            font_size: 10.,
                                                            ..Default::default()
                                                        },
                                                    ),
                                                    NoDeselect,
                                                ));
                                            });
                                        });
                                        cost_col.row(|silicate_row| {
                                            silicate_row.column(|dynamic_label| {
                                                dynamic_label.spawn((
                                                    TextBundle::from_section(
                                                        "Cost",
                                                        TextStyle {
                                                            font_size: 10.,
                                                            ..Default::default()
                                                        },
                                                    ),
                                                    PlanetResourceLabel(
                                                        planet_entity,
                                                        resource.name(),
                                                    ),
                                                    ResourceCostLabel(RawResourceType::Silicate),
                                                    NoDeselect,
                                                ));
                                            });
                                            silicate_row.column(|static_label| {
                                                static_label.spawn((
                                                    TextBundle::from_section(
                                                        "  Silicate",
                                                        TextStyle {
                                                            font_size: 10.,
                                                            ..Default::default()
                                                        },
                                                    ),
                                                    NoDeselect,
                                                ));
                                            });
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
                                                        background_color: BUTTON_PALETTE
                                                            .clone()
                                                            .none
                                                            .into(),
                                                        ..Default::default()
                                                    },
                                                    BUTTON_PALETTE.clone(),
                                                    PlanetResourceLabel(
                                                        planet_entity,
                                                        resource.name(),
                                                    ),
                                                    NoDeselect,
                                                ))
                                                .entity_commands()
                                                .with_children(|parent| {
                                                    parent.spawn((
                                                        TextBundle::from_section(
                                                            "Buy",
                                                            TextStyle {
                                                                font_size: 18.,
                                                                ..Default::default()
                                                            },
                                                        ),
                                                        NoDeselect,
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
                    })
                    .style()
                    .height(Val::Percent(100.))
                    .width(Val::Percent(100.))
                    .justify_content(JustifyContent::Center);
            })
            .insert((PlanetUI(planet_entity), StateScoped(Screen::Playing)))
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

fn update_ui_name(
    mut name_text_query: Query<&mut Text, With<PlanetNameLabel>>,
    selected_planet_query: Query<(&PickSelection, &Name), With<PlanetResources>>,
) {
    if let Some((_, name)) = selected_planet_query
        .iter()
        .find(|(selection, _)| selection.is_selected)
    {
        // Update the planet name text
        if let Ok(mut text) = name_text_query.get_single_mut() {
            name.as_str().clone_into(&mut text.sections[0].value);
        }
    }
}

fn update_buy_button(
    tech: Res<TechUnlocks>,
    resources: Res<HarvestedResources>,
    sun: Query<&Sun>,
    planet_query: Query<(&PlanetResources, &BuiltHarvesters, &GlobalTransform)>,
    mut button_query: Query<(&mut InteractionPalette, &PlanetResourceLabel)>,
) {
    for (mut pallete, label) in &mut button_query {
        if let Ok((resources, harvesters, transform)) = planet_query.get(label.0) {
            if let Some(resource) = resources.get(label.1) {}
        }
    }
}
