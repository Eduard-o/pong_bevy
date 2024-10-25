use crate::systems::{WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::color::palettes::basic::BLACK;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

#[derive(Component)]
pub struct PrimaryPaddle;

#[derive(Component)]
pub struct SecondaryPaddle;

#[derive(Component)]
pub struct Paddles;

enum Direction {
    Up,
    Down,
}

pub struct PaddlesPlugin;

impl Plugin for PaddlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, set_up_paddles);
        app.add_systems(Update, input);
    }
}

fn set_up_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(16.0, 64.0)).into(),
            material: materials.add(Color::from(BLACK)),
            transform: Transform::from_xyz(-(WINDOW_WIDTH / 2.0) + 8.0, 0.0, 0.0),
            ..default()
        },
        PrimaryPaddle,
        Paddles,
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(16.0, 64.0)).into(),
            material: materials.add(Color::from(BLACK)),
            transform: Transform::from_xyz((WINDOW_WIDTH / 2.0) - 8.0, 0.0, 0.0),
            ..default()
        },
        SecondaryPaddle,
        Paddles,
    ));
}

fn input(
    time: Res<Time>,
    kb_in: Res<ButtonInput<KeyCode>>,
    mut paddle_1q: Query<&mut Transform, (With<PrimaryPaddle>, Without<SecondaryPaddle>)>,
    mut paddle_2q: Query<&mut Transform, (With<SecondaryPaddle>, Without<PrimaryPaddle>)>,
) {
    let speed = 5.0;
    let delta = time.delta_seconds() * 100.0;

    let mut paddle_1 = paddle_1q.single_mut();
    let mut paddle_2 = paddle_2q.single_mut();

    // Handle first paddle input.
    if kb_in.pressed(KeyCode::KeyW) && in_bounds(paddle_1.translation, Direction::Up) {
        paddle_1.translation.y += speed * delta;
    } else if kb_in.pressed(KeyCode::KeyS) && in_bounds(paddle_1.translation, Direction::Down) {
        paddle_1.translation.y -= speed * delta;
    }

    // Handle second paddle input.
    if kb_in.pressed(KeyCode::ArrowUp) && in_bounds(paddle_2.translation, Direction::Up) {
        paddle_2.translation.y += speed * delta;
    } else if kb_in.pressed(KeyCode::ArrowDown) && in_bounds(paddle_2.translation, Direction::Down)
    {
        paddle_2.translation.y -= speed * delta;
    }
}

fn in_bounds(paddle_t: Vec3, direction: Direction) -> bool {
    match direction {
        Direction::Up => paddle_t.y + 32.0 < WINDOW_HEIGHT / 2.0,
        Direction::Down => paddle_t.y - 32.0 > -WINDOW_HEIGHT / 2.0,
    }
}
