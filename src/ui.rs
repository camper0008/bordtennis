use bevy::{prelude::*, sprite::Anchor, window::WindowResized};

use crate::{consts, state::State};

#[derive(Component)]
pub struct ControlsUI;

#[derive(Component)]
pub struct ScoreUI(usize);

#[derive(Component)]
pub struct TimeUI;

fn score_text_x_position(digit: usize, max_digits: usize) -> f32 {
    let half = (max_digits / 2) as isize;
    let text_width = 4.0 * 0.5 * consts::SCALE;
    let text_padding = 0.5 * consts::SCALE;

    let difference: isize = half - digit as isize;

    text_width * (difference as f32) + text_padding * (difference as f32)
}

fn score_text_y_position(window_height: f32) -> f32 {
    let height = 6.0 * 0.5 * consts::SCALE;
    let text_padding = 0.5 * consts::SCALE;

    window_height * 0.5 - height - text_padding
}

const SCORE_DIGITS: usize = 7;

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
    {
        let texture_handle = asset_server.load("score.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(6.0, 8.0), 8, 2, None, None);
        let texture_atlas = texture_atlases.add(texture_atlas);

        for digit in 0..SCORE_DIGITS {
            let texture_atlas = texture_atlas.clone();
            commands.spawn((
                SpriteSheetBundle {
                    texture_atlas,
                    transform: Transform::from_scale(Vec3::splat(0.5 * consts::SCALE))
                        .with_translation(Vec3::new(
                            score_text_x_position(digit, SCORE_DIGITS),
                            score_text_y_position(window.height()),
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
}

pub fn update(mut score_ui: Query<(&ScoreUI, &mut TextureAtlasSprite)>, state: Query<&State>) {
    let state = state.single();
    let state_points = format!(
        "{:0width$}",
        state.hits_with_velocity as usize,
        width = SCORE_DIGITS
    );

    for (ui, mut sprite) in &mut score_ui {
        let position = state_points.len() - ui.0 - 1;
        let value = &state_points[position..=position];
        let value: usize = value.parse().expect("should be valid score");
        sprite.index = value;
    }
}

pub fn window_resized(
    resize_event: Res<Events<WindowResized>>,
    mut controls_ui: Query<&mut Transform, With<ControlsUI>>,
    mut score_ui: Query<(&mut Transform, &ScoreUI), Without<ControlsUI>>,
) {
    let mut reader = resize_event.get_reader();
    for event in reader.iter(&resize_event) {
        let mut transform = controls_ui.single_mut();
        transform.translation.x = event.width * -0.5;

        for (mut transform, ui) in &mut score_ui {
            transform.translation.x = score_text_x_position(ui.0, SCORE_DIGITS);
            transform.translation.y = score_text_y_position(event.height);
        }
    }
}
