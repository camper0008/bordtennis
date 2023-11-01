use bevy::prelude::*;

use crate::consts;

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("table.png"),
        transform: Transform::from_scale(Vec3::splat(1.0 * consts::SCALE))
            .with_translation(Vec3::new(0.0, -2.0 * consts::SCALE, 0.0)),
        ..default()
    });
}
