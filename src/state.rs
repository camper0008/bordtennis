use bevy::{prelude::*, time::Stopwatch};

use crate::{
    audio::Music,
    ball::Ball,
    bat::{Bat, Direction, Variant},
    consts, keymap,
};

pub enum GameState {
    Paused,
    NewGame,
    Playing,
    Winner(Variant),
}

impl GameState {
    fn sprite_index(&self) -> usize {
        match self {
            GameState::Paused => 0,
            GameState::NewGame => 1,
            GameState::Winner(Variant::Light) => 2,
            GameState::Winner(Variant::Dark) => 3,
            GameState::Playing => unreachable!(),
        }
    }
}

#[derive(Component)]
pub struct State {
    pub game_state: GameState,
    pub game_time: Stopwatch,
    music_state: Music,
    pub hits_with_velocity: f32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            game_state: GameState::NewGame,
            game_time: Stopwatch::new(),
            music_state: Music::Zero,
            hits_with_velocity: 0.0,
        }
    }
}

impl State {
    pub fn game_over(&mut self, pause_state: GameState) {
        self.game_time.reset();
        self.music_state = Music::Zero;
        self.game_state = pause_state;
    }
}

#[derive(Component)]
pub struct ControlsUI;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let state = State::default();

    let texture_handle = asset_server.load("text.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 1, 5, None, None);
    let texture_atlas = texture_atlases.add(texture_atlas);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas,
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
    mut state: Query<(&mut State, &mut Transform, &mut TextureAtlasSprite)>,
    mut ball: Query<&mut Ball>,
    mut bats: Query<&mut Bat>,
    music_controller: Query<(&AudioSink, &Music)>,
) {
    let (mut state, mut transform, mut menu_sprite) = state.single_mut();
    if keys.just_pressed(keymap::pause()) {
        state.game_state = match state.game_state {
            GameState::Paused => GameState::Playing,
            GameState::Playing => GameState::Paused,
            GameState::NewGame | GameState::Winner(_) => {
                state.hits_with_velocity = 0.0;
                GameState::Playing
            }
        }
    }

    if keys.just_pressed(keymap::restart()) {
        for mut ball in &mut ball {
            let Ball {
                position,
                velocity,
                last_hit,
                hit_edge: _,
            } = Ball::default();
            ball.position = position;
            ball.velocity = velocity;
            ball.last_hit = last_hit;
        }
        for mut bat in &mut bats {
            bat.swinging = Direction::None;
            bat.position_x = 0.0;
        }
        state.game_over(GameState::NewGame);
    };
    if matches!(state.game_state, GameState::Playing) {
        state.game_time.tick(time.delta());
    }
    let elapsed = state.game_time.elapsed_secs();
    state.music_state = if elapsed > 96.0 {
        Music::Two
    } else if elapsed > 32.0 {
        Music::One
    } else {
        Music::Zero
    };
    for (&ref sink, &ref music_state) in &music_controller {
        match (&music_state, &state.music_state) {
            (Music::Two, Music::Two) if matches!(state.game_state, GameState::Playing) => {
                sink.play()
            }
            (Music::One, Music::One) if matches!(state.game_state, GameState::Playing) => {
                sink.play()
            }
            (Music::Zero, Music::Zero) if matches!(state.game_state, GameState::Playing) => {
                sink.play()
            }
            _ => sink.pause(),
        }
    }
    match &state.game_state {
        game_state @ (GameState::Paused | GameState::NewGame | GameState::Winner(_)) => {
            let offset = (time.elapsed_seconds() * 2.0).sin() * consts::SCALE;

            transform.scale = Vec3::splat(1.0 * consts::SCALE);
            transform.translation =
                Vec3::new(transform.translation.x, offset, transform.translation.z);
            menu_sprite.index = game_state.sprite_index();
        }
        GameState::Playing => transform.scale = Vec3::ZERO,
    }
}
