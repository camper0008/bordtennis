use bevy::{audio::PlaybackMode, prelude::*};

#[derive(Component)]
pub struct Hit;

#[derive(Component)]
pub struct Music;

pub fn spawn_hit_sound(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("hit.ogg"),
            ..default()
        },
        Hit,
    ));
}

pub fn spawn_music(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("music-step-0.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                paused: true,
                ..default()
            },
        },
        Music,
    ));
}
