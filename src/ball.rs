use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    audio::{self},
    bat::{Bat, Direction, Variant},
    consts,
    state::{GameState, State},
};

#[derive(Component)]
pub struct Ball {
    pub position: Vec2,
    pub velocity: Vec2,
    pub last_hit: Variant,
}

impl Default for Ball {
    fn default() -> Self {
        let server = Variant::Dark;
        Self {
            position: Vec2::new(0.0, server.default_y_position()),
            velocity: Vec2::new(0.0, server.default_y_position() * -0.5),
            last_hit: server,
        }
    }
}

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ball = Ball::default();
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("ball.png"),
            transform: Transform::from_scale(Vec3::splat(1.0 * consts::SCALE))
                .with_translation(Vec3::new(0.0, 0.0, 15.0)),
            ..default()
        },
        ball,
    ));
}

pub fn update(
    time: Res<Time>,
    mut state: Query<&mut State>,
    mut ball: Query<(&mut Transform, &mut Ball)>,
    mut bats: Query<&mut Bat>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut state = state.single_mut();
    if !matches!(state.game_state, GameState::Playing) {
        return;
    }
    let (mut transform, mut ball) = ball.single_mut();
    for bat in &mut bats {
        let initial_position = bat.variant.default_y_position();
        if initial_position > 0.0 && ball.position.y - 1.0 > initial_position
            || initial_position < 0.0 && ball.position.y + 1.0 < initial_position
        {
            continue;
        }

        match (&bat.variant, &ball.last_hit) {
            (Variant::Dark, Variant::Dark) => continue,
            (Variant::Light, Variant::Light) => continue,
            _ => {}
        };
        if bat.swinging != Direction::Down {
            continue;
        }
        let diff_x = bat.position_x - ball.position.x;
        let diff_y = bat.variant.default_y_position() - ball.position.y;
        if diff_y.abs() > 3.95 || diff_x.abs() > 3.95 {
            continue;
        }
        ball.velocity.x = -diff_x * 4.0;
        ball.velocity.y *= -(diff_y.abs() * 0.4).clamp(0.9, 1.25);
        ball.last_hit = bat.variant.clone();
        ball.velocity.y = ball.velocity.y.clamp(-64.0, 64.0);

        audio::spawn_hit_sound(&mut commands, &asset_server);
    }
    let offset = ball.velocity * Vec2::splat(time.delta_seconds());
    ball.position += offset;
    transform.translation.x = ball.position.x * consts::SCALE;
    transform.translation.y = ball.position.y * consts::SCALE;
    let angle = (ball.velocity.x / ball.velocity.y).atan();
    transform.rotation = Quat::from_rotation_z(angle + PI * 0.5);

    if !(Variant::Light.default_y_position()..Variant::Dark.default_y_position())
        .contains(&ball.position.y)
    {
        state.game_over(GameState::Winner(ball.last_hit.clone()));
        let Ball {
            position,
            velocity,
            last_hit,
        } = Ball::default();
        ball.position = position;
        ball.velocity = velocity;
        ball.last_hit = last_hit;
        for mut bat in &mut bats {
            bat.swinging = Direction::None;
            bat.position_x = 0.0;
        }
    }
}
