use bevy::{prelude::*, sprite::Anchor, window::WindowResized};

use crate::{
    consts,
    state::{GameState, State},
};

#[derive(Component)]
pub struct ControlsUI;

#[derive(Component)]
pub struct ScoreUI(usize);

#[derive(Component)]
pub struct TimerUI(usize);

fn timer_text_x_position(digit: usize, max_digits: usize, window_width: f32) -> f32 {
    let half = (max_digits / 2) as isize;
    let text_width = 4.0 * 0.5 * consts::SCALE;
    let text_padding = 0.5 * consts::SCALE;

    let difference: isize = half - digit as isize;

    text_width * (difference as f32) + text_padding * (difference as f32) + window_width * 0.5
        - text_width * max_digits as f32
}

fn score_text_x_position(digit: usize, max_digits: usize) -> f32 {
    let half = (max_digits / 2) as isize;
    let text_width = 4.0 * 0.5 * consts::SCALE;
    let text_padding = 0.5 * consts::SCALE;

    let difference: isize = half - digit as isize;

    text_width * (difference as f32) + text_padding * (difference as f32)
}

fn top_text_y_position(window_height: f32) -> f32 {
    let height = 6.0 * 0.5 * consts::SCALE;
    let text_padding = 0.5 * consts::SCALE;

    window_height * 0.5 - height - text_padding
}

const SCORE_DIGITS: usize = 7;
const TIMER_DIGITS: usize = 3;
const TIMER_SUB_DIGITS: usize = 1;

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    window: Query<&Window>,
) {
    let window = window.single();

    {
        let texture_handle = asset_server.load("text.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 1, 5, None, None);
        let texture_atlas = texture_atlases.add(texture_atlas);

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas,
                transform: Transform::from_scale(Vec3::splat(0.5 * consts::SCALE))
                    .with_translation(Vec3::new(window.resolution.width() * -0.5, 0.0, -100.0)),
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
    {
        let texture_handle = asset_server.load("score.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(6.0, 8.0), 8, 3, None, None);
        let texture_atlas = texture_atlases.add(texture_atlas);

        for digit in 0..SCORE_DIGITS {
            let texture_atlas = texture_atlas.clone();
            commands.spawn((
                SpriteSheetBundle {
                    texture_atlas,
                    transform: Transform::from_scale(Vec3::splat(0.5 * consts::SCALE))
                        .with_translation(Vec3::new(
                            score_text_x_position(digit, SCORE_DIGITS),
                            top_text_y_position(window.height()),
                            100.0,
                        )),
                    sprite: TextureAtlasSprite {
                        index: 0,
                        ..default()
                    },
                    ..default()
                },
                ScoreUI(digit),
            ));
        }
    }

    {
        let texture_handle = asset_server.load("score.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(6.0, 8.0), 8, 3, None, None);
        let texture_atlas = texture_atlases.add(texture_atlas);

        for digit in 0..TIMER_DIGITS + TIMER_SUB_DIGITS + 1 {
            let texture_atlas = texture_atlas.clone();
            commands.spawn((
                SpriteSheetBundle {
                    texture_atlas,
                    transform: Transform::from_scale(Vec3::splat(0.5 * consts::SCALE))
                        .with_translation(Vec3::new(
                            timer_text_x_position(digit, TIMER_DIGITS, window.width()),
                            top_text_y_position(window.height()),
                            100.0,
                        )),
                    sprite: TextureAtlasSprite {
                        index: 0,
                        ..default()
                    },
                    ..default()
                },
                TimerUI(digit),
            ));
        }
    }
}

pub fn update(
    mut score_ui: Query<(&ScoreUI, &mut TextureAtlasSprite, &mut Transform)>,
    mut timer_ui: Query<(&TimerUI, &mut TextureAtlasSprite), Without<ScoreUI>>,
    state: Query<&State>,
    window: Query<&Window>,
    time: Res<Time>,
) {
    let window = window.single();
    let state = state.single();
    let state_points = format!(
        "{:0width$}",
        state.hits_with_velocity as usize,
        width = SCORE_DIGITS
    );

    let state_timer = format!(
        "{:0width$.1}",
        state.game_time.elapsed_secs(),
        width = TIMER_DIGITS + TIMER_SUB_DIGITS + 1
    );

    for (ui, mut sprite, mut transform) in &mut score_ui {
        let position = state_points.len() - ui.0 - 1;
        let value = &state_points[position..=position];
        let value: usize = value.parse().expect("should be valid score");
        if !matches!(state.game_state, GameState::Playing) {
            transform.translation.y = top_text_y_position(window.height())
                + (time.elapsed_seconds() * consts::SCALE * 0.5 + ui.0 as f32).sin()
                    * consts::SCORE_ANIMATION_OFFSET;
        } else {
            transform.translation.y = top_text_y_position(window.height());
        }
        sprite.index = value;
    }

    const COMMA_SPRITE_INDEX: usize = 10;

    for (ui, mut sprite) in &mut timer_ui {
        let position = state_timer.len() - ui.0 - 1;
        let value = &state_timer[position..=position];
        let value = value.parse().unwrap_or(COMMA_SPRITE_INDEX);
        let value = if ui.0 < TIMER_SUB_DIGITS {
            value + COMMA_SPRITE_INDEX + 1
        } else {
            value
        };
        sprite.index = value;
    }
}

pub fn window_resized(
    resize_event: Res<Events<WindowResized>>,
    mut controls_ui: Query<&mut Transform, With<ControlsUI>>,
    mut score_ui: Query<(&mut Transform, &ScoreUI), (Without<ControlsUI>, Without<TimerUI>)>,
    mut timer_ui: Query<(&mut Transform, &TimerUI), (Without<ControlsUI>, Without<ScoreUI>)>,
) {
    let mut reader = resize_event.get_reader();
    for event in reader.iter(&resize_event) {
        let mut transform = controls_ui.single_mut();
        transform.translation.x = event.width * -0.5;

        for (mut transform, ui) in &mut score_ui {
            transform.translation.x = score_text_x_position(ui.0, SCORE_DIGITS);
            transform.translation.y = top_text_y_position(event.height);
        }

        for (mut transform, ui) in &mut timer_ui {
            transform.translation.x = timer_text_x_position(ui.0, TIMER_DIGITS, event.width);
            transform.translation.y = top_text_y_position(event.height);
        }
    }
}
