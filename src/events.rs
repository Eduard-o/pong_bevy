use crate::ball::Ball;
use crate::paddles::{PrimaryPaddle, SecondaryPaddle};
use crate::systems::{WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;

#[derive(Event)]
pub struct BallCollisionEvent(pub CollisionType);

pub enum CollisionType {
    WindowHeight,
    WindowWidth,
    Paddle,
}

pub fn on_ball_collision(
    mut event_ball_collision: EventWriter<BallCollisionEvent>,
    ball_q: Query<&Transform, With<Ball>>,
    paddle_1q: Query<&Transform, With<PrimaryPaddle>>,
    paddle_2q: Query<&Transform, With<SecondaryPaddle>>,
) {
    let Ok(ball_t) = ball_q.get_single() else {
        return;
    };

    let ball = ball_t.translation;
    let ball_radius = 5.0;

    let paddle_1 = paddle_1q.single().translation;
    let paddle_2 = paddle_2q.single().translation;

    // TODO: Fix ball collision. This is temporary garbage collision detection.
    if (ball.x + ball_radius < paddle_1.x + 16.0
        && (ball.y + ball_radius > paddle_1.y - 32.0 && ball.y + ball_radius < paddle_1.y + 32.0))
        || (ball.x + ball_radius > paddle_2.x - 8.0
            && (ball.y + ball_radius > paddle_2.y - 32.0
                && ball.y + ball_radius < paddle_2.y + 32.0))
    {
        event_ball_collision.send(BallCollisionEvent(CollisionType::Paddle));
        println!("Paddle Collision");
    }

    if ball_t.translation.x + ball_radius > WINDOW_WIDTH / 2.0
        || ball_t.translation.x - ball_radius < -WINDOW_WIDTH / 2.0
    {
        event_ball_collision.send(BallCollisionEvent(CollisionType::WindowWidth));
        println!("Window Width Collision");
    }

    if ball_t.translation.y + ball_radius > WINDOW_HEIGHT / 2.0
        || ball_t.translation.y - ball_radius < -WINDOW_HEIGHT / 2.0
    {
        event_ball_collision.send(BallCollisionEvent(CollisionType::WindowHeight));
        println!("Window Height Collision");
    }
}
