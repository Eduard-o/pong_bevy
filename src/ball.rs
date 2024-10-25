use crate::events::{BallCollisionEvent, CollisionType};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use rand;
use rand::{thread_rng, Rng};

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Velocity(pub Vec2, pub f32);

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, set_up_ball);
        app.add_systems(Update, move_ball);
    }
}

fn set_up_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let initial_direction = Vec2::new(
        thread_rng().gen_range(-0.5..0.6),
        thread_rng().gen_range(-0.5..0.6),
    );

    let initial_speed: f32 = 5.0;

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(5.0)).into(),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Ball,
        Velocity(initial_direction.normalize(), initial_speed),
    ));
}

fn move_ball(
    time: Res<Time>,
    mut ball_q: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    mut ball_collision_event: EventReader<BallCollisionEvent>,
) {
    let Ok((mut ball, mut ball_velocity)) = ball_q.get_single_mut() else {
        return;
    };

    for event in ball_collision_event.read() {
        match event.0 {
            CollisionType::WindowHeight => {
                ball_velocity.0.y *= -1.0;
            }
            CollisionType::WindowWidth => {
                ball_velocity.0.x *= -1.0;
            }
            CollisionType::Paddle => {
                ball_velocity.0.x *= -1.0;
            }
        }
    }

    let velocity = ball_velocity.0 * ball_velocity.1;

    apply_velocity(&time, &mut ball, velocity);

    ball_velocity.1 += 0.005 * time.delta_seconds();
}

fn apply_velocity(time: &Res<Time>, ball: &mut Mut<Transform>, velocity: Vec2) {
    let delta = time.delta_seconds() * 100.0;

    ball.translation.x += velocity.x * delta;
    ball.translation.y += velocity.y * delta;
}
