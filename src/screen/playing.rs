//! The screen state for the main game loop.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use super::Screen;
use crate::game::{
    assets::SoundtrackKey,
    audio::soundtrack::PlaySoundtrack,
    resources::{RawResourceType, ResourceLabel},
    spawn::level::SpawnLevel,
    sun::{SunCycleLabel, SunPowerLabel},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), enter_playing);
    app.add_systems(OnExit(Screen::Playing), exit_playing);

    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

// FIXME: Fix the too many lines issue by breaking this up
#[allow(clippy::too_many_lines)]
fn enter_playing(mut commands: Commands) {
    commands.trigger(SpawnLevel);
    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Gameplay));

    // Spawn UI
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(10.),
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
        })
        .insert(StateScoped(Screen::Playing))
        .with_children(|root| {
            // Box for sun
            root.spawn(NodeBundle {
                style: Style {
                    display: Display::Grid,
                    width: Val::Percent(10.),
                    height: Val::Percent(100.),
                    grid_template_columns: vec![GridTrack::max_content(), GridTrack::auto()],
                    grid_template_rows: vec![
                        GridTrack::auto(),
                        GridTrack::auto(),
                        GridTrack::auto(),
                    ],
                    justify_items: JustifyItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|parent| {
                // We want the first item to take up the entire top row
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            grid_column: GridPlacement::span(2),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "SUN",
                            TextStyle {
                                font_size: 24.,
                                ..Default::default()
                            },
                        ));
                    });

                parent.spawn(TextBundle::from_section(
                    "Solar Output:",
                    TextStyle {
                        font_size: 20.,
                        ..Default::default()
                    },
                ));
                parent
                    .spawn(TextBundle::from_section(
                        "0",
                        TextStyle {
                            font_size: 18.,
                            ..Default::default()
                        },
                    ))
                    .insert(SunPowerLabel);
                parent.spawn(TextBundle::from_section(
                    "Solar Cycle:",
                    TextStyle {
                        font_size: 20.,
                        ..Default::default()
                    },
                ));
                parent
                    .spawn(TextBundle::from_section(
                        "1",
                        TextStyle {
                            font_size: 18.,
                            ..Default::default()
                        },
                    ))
                    .insert(SunCycleLabel);
            });

            // Box for spacing
            root.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(5.),
                    ..Default::default()
                },
                ..Default::default()
            });

            // Box for resources
            root.spawn(NodeBundle {
                style: Style {
                    display: Display::Grid,
                    width: Val::Percent(90.),
                    height: Val::Percent(100.),
                    grid_template_columns: vec![
                        GridTrack::auto(),
                        GridTrack::auto(),
                        GridTrack::auto(),
                        GridTrack::auto(),
                        GridTrack::auto(),
                    ],
                    grid_template_rows: vec![GridTrack::auto(), GridTrack::auto()],
                    justify_items: JustifyItems::Center,
                    ..Default::default()
                },
                //background_color: Color::WHITE.darker(0.5).into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Power",
                    TextStyle {
                        font_size: 24.,
                        ..Default::default()
                    },
                ));
                parent.spawn(TextBundle::from_section(
                    "Metals",
                    TextStyle {
                        font_size: 24.,
                        ..Default::default()
                    },
                ));
                parent.spawn(TextBundle::from_section(
                    "Silicate",
                    TextStyle {
                        font_size: 24.,
                        ..Default::default()
                    },
                ));
                parent.spawn(TextBundle::from_section(
                    "Hydrogen",
                    TextStyle {
                        font_size: 24.,
                        ..Default::default()
                    },
                ));
                parent.spawn(TextBundle::from_section(
                    "Oxygen",
                    TextStyle {
                        font_size: 24.,
                        ..Default::default()
                    },
                ));

                parent.spawn((
                    TextBundle::from_section(
                        "0",
                        TextStyle {
                            font_size: 18.,
                            ..Default::default()
                        },
                    ),
                    ResourceLabel(RawResourceType::Power),
                ));
                parent.spawn((
                    TextBundle::from_section(
                        "1",
                        TextStyle {
                            font_size: 18.,
                            ..Default::default()
                        },
                    ),
                    ResourceLabel(RawResourceType::Metals),
                ));
                parent.spawn((
                    TextBundle::from_section(
                        "2",
                        TextStyle {
                            font_size: 18.,
                            ..Default::default()
                        },
                    ),
                    ResourceLabel(RawResourceType::Silicate),
                ));
                parent.spawn((
                    TextBundle::from_section(
                        "3",
                        TextStyle {
                            font_size: 18.,
                            ..Default::default()
                        },
                    ),
                    ResourceLabel(RawResourceType::Hydrogen),
                ));
                parent.spawn((
                    TextBundle::from_section(
                        "4",
                        TextStyle {
                            font_size: 18.,
                            ..Default::default()
                        },
                    ),
                    ResourceLabel(RawResourceType::Oxygen),
                ));
            });
        });
}

fn exit_playing(mut commands: Commands) {
    // We could use [`StateScoped`] on the sound playing entities instead.
    commands.trigger(PlaySoundtrack::Disable);
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
