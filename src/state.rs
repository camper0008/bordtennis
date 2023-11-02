use bevy::{prelude::*, sprite::Anchor, time::Stopwatch, window::WindowResized};

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
        self.hits_with_velocity = 0.0;
    }
}

#[derive(Component)]
pub struct ControlsUI;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    window: Query<&Window>,
) {
    let state = State::default();

    let texture_handle = asset_server.load("text.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 1, 5, None, None);
    let menu_atlas = texture_atlases.add(texture_atlas);
    let controls_atlas = menu_atlas.clone();

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: menu_atlas,
            transform: Transform::from_scale(Vec3::splat(1.0 * consts::SCALE))
                .with_translation(Vec3::new(0.0, 0.0, 100.0)),
            ..default()
        },
        state,
    ));

    let window = window.single();

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: controls_atlas,
            transform: Transform::from_scale(Vec3::splat(0.5 * consts::SCALE))
                .with_translation(Vec3::new(window.resolution.width() * -0.5, 0.0, 100.0)),
            sprite: TextureAtlasSprite {
                index: 4,
                anchor: Anchor::CenterLeft,
                ..default()
            },
            ..default()
        },
        ControlsUI,
    ));
}

pub fn window_resized(
    resize_event: Res<Events<WindowResized>>,
    mut controls_ui: Query<&mut Transform, With<ControlsUI>>,
) {
    let mut reader = resize_event.get_reader();
    for event in reader.iter(&resize_event) {
        let mut transform = controls_ui.single_mut();
        transform.translation.x = event.width * -0.5;
    }
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
    if keys.just_pressed(keymap::pause()) && matches!(state.game_state, GameState::Playing) {
        state.game_state = GameState::Paused;
    } else if keys.just_pressed(keymap::pause()) {
        state.game_state = GameState::Playing;
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
    state.music_state = if elapsed > 90.0 {
        Music::Two
    } else if elapsed > 45.0 {
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
            transform.scale = Vec3::splat(1.0 * consts::SCALE);
            transform.translation = Vec3::new(
                transform.translation.x,
                time.elapsed_seconds().sin() * consts::SCALE * 4.0,
                transform.translation.z,
            );
            menu_sprite.index = game_state.sprite_index();
        }
        GameState::Playing => transform.scale = Vec3::ZERO,
    }
}
