use bevy::{prelude::*, time::Stopwatch};

use crate::{
    audio::Music,
    ball::Ball,
    bat::{Bat, Direction, Variant},
    consts, keymap,
};

pub enum PauseState {
    Paused,
    NewGame,
    None,
    Winner(Variant),
}
#[derive(Component)]
pub struct State {
    pub pause_state: PauseState,
    game_time: Stopwatch,
    music_state: Music,
}

pub fn spawn(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let state = State {
        pause_state: PauseState::NewGame,
        game_time: Stopwatch::new(),
        music_state: Music::One,
    };
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
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut state: Query<(&mut State, &mut Transform)>,
    mut ball: Query<&mut Ball>,
    mut bats: Query<&mut Bat>,
    music_controller: Query<(&AudioSink, &Music)>,
) {
    let (mut state, mut transform) = state.get_single_mut().unwrap();
    if keys.just_pressed(keymap::pause()) && matches!(state.pause_state, PauseState::None) {
        state.pause_state = PauseState::Paused;
        transform.scale = Vec3::splat(1.0 * consts::SCALE);
    } else if keys.just_pressed(keymap::pause()) {
        state.pause_state = PauseState::None;
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
        state.pause_state = PauseState::NewGame;
        state.game_time.reset();
        transform.scale = Vec3::splat(1.0 * consts::SCALE);
    };
    if matches!(state.pause_state, PauseState::None) {
        state.game_time.tick(time.delta());
    }
    let elapsed = state.game_time.elapsed_secs();
    state.music_state = if elapsed > 90.0 {
        Music::Two
    } else if elapsed > 45.0 {
        Music::One
    } else {
        Music::Zero
    };
    for (&ref sink, &ref music_state) in &music_controller {
        match (&music_state, &state.music_state) {
            (Music::Two, Music::Two) => sink.play(),
            (Music::One, Music::One) => sink.play(),
            (Music::Zero, Music::Zero) => sink.play(),
            _ => sink.pause(),
        }
    }
}
