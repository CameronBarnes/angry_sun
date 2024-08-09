use bevy::{
    ecs::system::{EntityCommand, EntityCommands},
    prelude::*,
    reflect::List,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_bars);
}

#[derive(Component, Debug)]
pub struct MultiProgressBar {
    section_colors: Vec<Color>,
    section_values: Vec<f32>,
    separated: bool,
}

#[derive(Debug, Component, Clone, Copy)]
pub struct MultiProgressBarSegment(pub usize);

impl MultiProgressBar {
    fn new(sections: Vec<(f32, Color)>, separated: bool) -> Self {
        let (section_values, section_colors) = sections.into_iter().unzip();
        Self {
            section_colors,
            section_values,
            separated,
        }
    }

    #[allow(clippy::cast_precision_loss)]
    fn new_from_colors(sections: Vec<Color>, separated: bool) -> Self {
        let num = sections.len();
        Self {
            section_colors: sections,
            section_values: (0..num).map(|_| 100. / (num as f32)).collect(),
            separated,
        }
    }
}

impl MultiProgressBar {
    #[allow(clippy::cast_precision_loss)]
    pub fn spawn_with_colors(commands: EntityCommands, sections: Vec<Color>) {
        let len = sections.len();
        let val = 100. / (len as f32);
        Self::spawn(commands, (0..len).map(|_| val).zip(sections).collect());
    }

    pub fn spawn(mut commands: EntityCommands, sections: Vec<(f32, Color)>) {
        commands.with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            height: Val::Percent(100.),
                            width: Val::Percent(100.),
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Self::new(sections.clone(), true),
                ))
                .with_children(|parent| {
                    for (id, (width, color)) in sections.into_iter().enumerate() {
                        parent.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(width),
                                    height: Val::Percent(100.),
                                    ..Default::default()
                                },
                                background_color: color.into(),
                                ..Default::default()
                            },
                            MultiProgressBarSegment(id),
                        ));
                    }
                });
        });
    }
}

fn update_bars(
    bar_query: Query<&MultiProgressBar>,
    mut segment_query: Query<(
        &Parent,
        &mut Style,
        &mut BackgroundColor,
        &MultiProgressBarSegment,
    )>,
) {
    for (parent, mut style, mut color, segment) in &mut segment_query {
        let Ok(parent) = bar_query.get(parent.get()) else {
            continue; // TODO: Probably throw some kind of error here
        };
        if parent.separated {
            let new_color = parent.section_colors[segment.0];
            let width = parent.section_values[segment.0];
            style.width = Val::Percent(width);
            *color = new_color.into();
            if width > 0. {
                style.min_width = Val::Px(1.);
            } else {
                style.min_width = Val::Px(0.);
            }
        } else {
            unimplemented!()
        }
    }
}
