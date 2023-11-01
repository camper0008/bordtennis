use bevy::prelude::*;

mod audio;
mod ball;
mod bat;
mod consts;
mod keymap;
mod state;
mod table;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("bdadf7").unwrap()))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, bat::update)
        .add_systems(Update, ball::update)
        .add_systems(Update, state::update)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    bat::spawn(&mut commands, &asset_server, bat::Variant::Dark);
    bat::spawn(&mut commands, &asset_server, bat::Variant::Light);
    table::spawn(&mut commands, &asset_server);
    ball::spawn(&mut commands, &asset_server);
    state::spawn(&mut commands, &asset_server);
    audio::spawn_music(&mut commands, &asset_server);
}
