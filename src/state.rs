use bevy::prelude::*;

use crate::{
    ball::Ball,
    bat::{Bat, Direction, Variant},
    consts, keymap,
};

#[derive(Component)]
pub enum State {
    Paused,
    NewGame,
    None,
    Winner(Variant),
}

pub fn spawn(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let state = State::NewGame;
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("pause.png"),
            transform: Transform::from_scale(Vec3::splat(1.0 * consts::SCALE))
                .with_translation(Vec3::new(0.0, 0.0, 100.0)),
            ..default()
        },
        state,
    ));
}

pub fn update(
    keys: Res<Input<KeyCode>>,
    mut state: Query<(&mut State, &mut Transform)>,
    mut ball: Query<&mut Ball>,
    mut bats: Query<&mut Bat>,
) {
    for (mut state, mut transform) in &mut state {
        if keys.just_pressed(keymap::pause()) && matches!(state.as_ref(), State::None) {
            *state = State::Paused;
            transform.scale = Vec3::splat(1.0 * consts::SCALE);
        } else if keys.just_pressed(keymap::pause()) {
            *state = State::None;
            transform.scale = Vec3::splat(0.0);
        }
        if keys.just_pressed(keymap::restart()) {
            for mut ball in &mut ball {
                let Ball {
                    position,
                    velocity,
                    last_hit,
                } = Ball::default();
                ball.position = position;
                ball.velocity = velocity;
                ball.last_hit = last_hit;
            }
            for mut bat in &mut bats {
                bat.swinging = Direction::None;
                bat.position_x = 0.0;
            }
            *state = State::NewGame;
            transform.scale = Vec3::splat(1.0 * consts::SCALE);
        }
    }
}
