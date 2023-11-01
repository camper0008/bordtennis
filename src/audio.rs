use bevy::{prelude::*, audio::PlaybackMode};

#[derive(Component)]
pub struct Hit;

#[derive(Component)]
pub struct Music;


pub fn spawn_hit_sound(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((AudioBundle {
        source: asset_server.load("hit.ogg"),
        ..default()
    }, Hit));
}

pub fn spawn_music(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((AudioBundle {
        source: asset_server.load("music.ogg"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            ..default()
        }
    }, Music));
}

pub fn update_music(
    time: Res<Time>,
) {
    if let Ok(sink) = music_controller.get_single() {
        sink.set_speed(1.0);
    }
}