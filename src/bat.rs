use std::f32::consts::PI;

use bevy::{prelude::*, time::Stopwatch};

use crate::{consts, keymap};

#[derive(Component)]
pub struct BatDebounce {
    pub time: Stopwatch,
}

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
}

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    None,
}

#[derive(Component)]
pub struct Bat {
    variant: Variant,
    animation_timer: Timer,
    pub swinging: Direction,
    offset_x: f32,
}

pub fn spawn(commands: &mut Commands, asset_server: &Res<AssetServer>, variant: Variant) {
    let (item_offset, texture) = match variant {
        Variant::Light => (Vec3::new(0.0, -20.0 * consts::SCALE, 10.0), "bat_light.png"),
        Variant::Dark => (Vec3::new(0.0, 20.0 * consts::SCALE, 10.0), "bat_dark.png"),
    };
    let bat = Bat {
        variant,
        swinging: Direction::None,
        animation_timer: Timer::from_seconds(consts::SWING_COOLDOWN, TimerMode::Repeating),
        offset_x: 0.0,
    };
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(texture),
            transform: Transform::from_scale(Vec3::splat(1.0 * consts::SCALE))
                .with_translation(item_offset)
                .with_rotation(bat.variant.default_rotation()),
            ..default()
        },
        bat,
    ));
}

pub fn update(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Bat)>,
) {
    for (mut transform, mut bat) in &mut query {
        match bat.swinging {
            Direction::None => {
                if keys.just_pressed(keymap::swing(&bat.variant)) {
                    bat.swinging = Direction::Down;
                    continue;
                }
                if keys.pressed(keymap::left(&bat.variant)) {
                    bat.offset_x -= time.delta_seconds() * consts::SCALE * consts::MOVE_SPEED;
                }
                if keys.pressed(keymap::right(&bat.variant)) {
                    bat.offset_x += time.delta_seconds() * consts::SCALE * consts::MOVE_SPEED;
                }
                bat.offset_x = bat
                    .offset_x
                    .clamp(-consts::SCALE * 12.0, consts::SCALE * 12.0);
                let offset = Vec3::new(
                    bat.offset_x,
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
