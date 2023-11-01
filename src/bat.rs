use std::f32::consts::PI;

use bevy::{prelude::*, time::Stopwatch};

use crate::{
    consts, keymap,
    state::{GameState, State},
};

#[derive(Component)]
pub struct BatDebounce {
    pub time: Stopwatch,
}

#[derive(Clone)]
pub enum Variant {
    Light,
    Dark,
}

impl Variant {
    fn default_rotation(&self) -> Quat {
        match self {
            Variant::Light => Quat::from_rotation_z(0.0),
            Variant::Dark => Quat::from_rotation_z(PI),
        }
    }
    pub fn default_y_position(&self) -> f32 {
        match self {
            Variant::Light => -20.0,
            Variant::Dark => 20.0,
        }
    }
}

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    None,
}

#[derive(Component)]
pub struct Bat {
    pub variant: Variant,
    animation_timer: Timer,
    pub swinging: Direction,
    pub position_x: f32,
}

pub fn spawn(commands: &mut Commands, asset_server: &Res<AssetServer>, variant: Variant) {
    let texture = match variant {
        Variant::Light => "bat_light.png",
        Variant::Dark => "bat_dark.png",
    };
    let position = Vec3::new(0.0, variant.default_y_position() * consts::SCALE, 10.0);
    let bat = Bat {
        variant,
        swinging: Direction::None,
        animation_timer: Timer::from_seconds(consts::SWING_COOLDOWN, TimerMode::Repeating),
        position_x: 0.0,
    };
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(texture),
            transform: Transform::from_scale(Vec3::splat(1.0 * consts::SCALE))
                .with_translation(position)
                .with_rotation(bat.variant.default_rotation()),
            ..default()
        },
        bat,
    ));
}

pub fn update(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut bat: Query<(&mut Transform, &mut Bat)>,
    state: Query<&State>,
) {
    let state = state.get_single().unwrap();
    if !matches!(state.game_state, GameState::Playing) {
        return;
    }
    for (mut transform, mut bat) in &mut bat {
        match bat.swinging {
            Direction::None => {
                if keys.just_pressed(keymap::swing(&bat.variant)) {
                    bat.swinging = Direction::Down;
                    continue;
                }
                let move_speed = if keys.pressed(keymap::run(&bat.variant)) {
                    consts::RUN_SPEED
                } else {
                    consts::MOVE_SPEED
                };
                if keys.pressed(keymap::left(&bat.variant)) {
                    bat.position_x -= time.delta_seconds() * move_speed;
                }
                if keys.pressed(keymap::right(&bat.variant)) {
                    bat.position_x += time.delta_seconds() * move_speed;
                }
                bat.position_x = bat
                    .position_x
                    .clamp(-consts::SCALE * 12.0, consts::SCALE * 12.0);
                let offset = Vec3::new(
                    bat.position_x * consts::SCALE,
                    transform.translation.y,
                    transform.translation.z,
                );
                transform.translation = offset;
            }
            Direction::Up => {
                transform.rotate_x(PI * 4.0 * time.delta().as_secs_f32());
                bat.animation_timer.tick(time.delta());
                if bat.animation_timer.just_finished() {
                    transform.rotation = bat.variant.default_rotation();
                    bat.swinging = Direction::None;
                }
            }
            Direction::Down => {
                transform.rotate_x(-PI * 4.0 * time.delta().as_secs_f32());
                bat.animation_timer.tick(time.delta());
                if bat.animation_timer.just_finished() {
                    bat.swinging = Direction::Up;
                }
            }
        };
    }
}
