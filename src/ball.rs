use bevy::prelude::*;

use crate::{
    bat::{Bat, Direction, Variant},
    consts,
};

#[derive(Component)]
pub struct Ball {
    position: Vec2,
    velocity: Vec2,
    last_hit: Option<Variant>,
}

pub fn spawn(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let ball = Ball {
        position: Vec2::new(0.0, 0.0),
        velocity: Vec2::new(0.0, -10.0),
        last_hit: None,
    };
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
    mut ball: Query<(&mut Transform, &mut Ball)>,
    mut bats: Query<&mut Bat>,
) {
    for (mut transform, mut ball) in &mut ball {
        for bat in &mut bats {
            let initial_position = bat.variant.default_y_position();
            if initial_position > 0.0 && ball.position.y - 1.0 > initial_position
                || initial_position < 0.0 && ball.position.y + 1.0 < initial_position
            {
                continue;
            }

            match (&bat.variant, &ball.last_hit) {
                (Variant::Dark, Some(Variant::Dark)) => continue,
                (Variant::Light, Some(Variant::Light)) => continue,
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
            ball.last_hit = Some(bat.variant.clone());
            ball.velocity.y = ball.velocity.y.clamp(-64.0, 64.0);
        }
        let offset = ball.velocity * Vec2::splat(time.delta_seconds());
        ball.position += offset;
        transform.translation.x = ball.position.x * consts::SCALE;
        transform.translation.y = ball.position.y * consts::SCALE;
    }
}
