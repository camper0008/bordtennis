use bevy::prelude::KeyCode;

use crate::bat::Variant;

pub fn swing(variant: &Variant) -> KeyCode {
    match variant {
        Variant::Light => KeyCode::W,
        Variant::Dark => KeyCode::Up,
    }
}

pub fn left(variant: &Variant) -> KeyCode {
    match variant {
        Variant::Light => KeyCode::A,
        Variant::Dark => KeyCode::Left,
    }
}

pub fn right(variant: &Variant) -> KeyCode {
    match variant {
        Variant::Light => KeyCode::D,
        Variant::Dark => KeyCode::Right,
    }
}
