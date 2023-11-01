use bevy::{audio::PlaybackMode, prelude::*};

#[derive(Component)]
pub struct Hit;

#[derive(Component)]
pub enum Music {
    Zero,
    One,
    Two,
}

pub fn spawn_hit_sound(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("hit.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                ..default()
            },
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
        Music::Zero,
    ));
    commands.spawn((
        AudioBundle {
            source: asset_server.load("music-step-1.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                paused: true,
                ..default()
            },
        },
        Music::One,
    ));
    commands.spawn((
        AudioBundle {
            source: asset_server.load("music-step-2.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                paused: true,
                ..default()
            },
        },
        Music::Two,
    ));
}
