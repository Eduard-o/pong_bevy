use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode, WindowResolution, WindowTheme};

pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;

pub fn custom_window() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            present_mode: PresentMode::AutoNoVsync,
            mode: WindowMode::Windowed,
            position: WindowPosition::Centered(MonitorSelection::Primary),
            resolution: WindowResolution::from([WINDOW_WIDTH, WINDOW_HEIGHT]),
            title: "Pong with Bevy".to_string(),
            name: Some("pong_bevy.app".to_string()),
            resizable: false,
            window_theme: Some(WindowTheme::Dark),
            ..default()
        }),
        ..default()
    }
}

pub fn set_up_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn exit_on_esc(kb_input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if kb_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
