mod ball;
mod events;
mod paddles;
mod systems;

use crate::ball::BallPlugin;
use crate::events::{on_ball_collision, BallCollisionEvent};
use crate::paddles::PaddlesPlugin;
use crate::systems::{custom_window, exit_on_esc, set_up_camera};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_event::<BallCollisionEvent>()
        .add_plugins(DefaultPlugins.set(custom_window()))
        .add_plugins(PaddlesPlugin)
        .add_plugins(BallPlugin)
        .add_systems(Startup, set_up_camera)
        .add_systems(Update, (on_ball_collision, exit_on_esc))
        .run();
}
