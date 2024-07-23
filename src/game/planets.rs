use bevy::{
    app::{App, Update},
    prelude::{in_state, IntoSystemConfigs, Query, Res, Transform, With},
    time::Time,
};

use crate::screen::Screen;

use super::spawn::planets::Orbit;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (move_planets).run_if(in_state(Screen::Playing)));
}

fn move_planets(
    time: Res<Time>,
    mut planet_query: Query<(&mut Transform, &mut Orbit), With<Orbit>>,
) {
    if planet_query.is_empty() {
        return;
    }

    for (mut transform, mut orbit) in &mut planet_query {
        orbit.increment_orbit(time.delta_seconds());
        let (x, y) = orbit.to_x_y();
        transform.translation.x = x;
        transform.translation.y = y;
    }
}
