use bevy::prelude::*;

use crate::{
    bat::{Bat, Direction},
    consts,
};

#[derive(Component)]
pub struct Ball {}

pub fn spawn(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let ball = Ball {};
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("ball.png"),
            transform: Transform::from_scale(Vec3::splat(1.0 * consts::SCALE))
                .with_translation(Vec3::new(0.0, 0.0, 5.0)),
            ..default()
        },
        ball,
    ));
}

pub fn update(
    time: Res<Time>,
    mut ball: Query<(&mut Transform, &mut Ball)>,
    mut bats: Query<&mut Bat>,
) {
    for (mut transform, mut _ball) in &mut ball {
        transform.translation.y = time.elapsed_seconds().sin() * consts::SCALE * 16.0;
        for bat in &mut bats {
            match bat.swinging {
                Direction::Up => todo!(),
                Direction::Down => todo!(),
                Direction::None => todo!(),
            }
        }
    }
}
