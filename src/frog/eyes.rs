//! Specific functionality related to the frog's eyes.

use super::AnimationIndices;
use bevy::prelude::*;
use std::time::Duration;

/// [`AnimationIndices`] for when the frog's eyes are open.
pub const OPEN_INDICES: AnimationIndices = AnimationIndices { first: 3, last: 4 };
/// [`AnimationIndices`] for when the frog's eyes are blinking.
pub const BLINK_INDICES: AnimationIndices = AnimationIndices::splat(5);

/// Seconds that the frog's eyes stay open.
pub const OPEN_DURATION: f32 = 5.0;
/// Seconds that the frog's eyes stay blinking.
pub const BLINK_DURATION: f32 = 0.2;

/// The frog eyes component.
#[derive(Component, Default, Debug)]
pub enum FrogEyes {
    #[default]
    Open,
    // Closed,
    Blinking,
}

/// A timer that tracks the [`FrogEyes`]'s blink duration.
#[derive(Component, Deref, DerefMut, Debug)]
pub struct BlinkTimer(Timer);

impl BlinkTimer {
    pub fn set_open(&mut self) {
        self.0.set_duration(Duration::from_secs_f32(OPEN_DURATION));
        self.0.reset();
    }

    pub fn set_blinking(&mut self) {
        self.0.set_duration(Duration::from_secs_f32(BLINK_DURATION));
        self.0.reset();
    }
}

impl Default for BlinkTimer {
    fn default() -> Self {
        BlinkTimer(Timer::from_seconds(OPEN_DURATION, TimerMode::Once))
    }
}

/// Updates the [`FrogEyes`]'s blink status on [`Update`].
pub fn blink_frog(
    mut query: Query<(
        &mut FrogEyes,
        &mut AnimationIndices,
        &mut TextureAtlasSprite,
    )>,
    mut blink_timer: Local<BlinkTimer>,
    time: Res<Time>,
) {
    blink_timer.tick(time.delta());

    if blink_timer.finished() {
        let (mut frog_eyes, mut indices, mut sprite) = query.single_mut();

        match *frog_eyes {
            FrogEyes::Open => {
                *frog_eyes = FrogEyes::Blinking;
                *indices = BLINK_INDICES;
                sprite.index = 5;
                blink_timer.set_blinking();
            }
            FrogEyes::Blinking => {
                *frog_eyes = FrogEyes::Open;
                *indices = OPEN_INDICES;
                sprite.index = 3;
                blink_timer.set_open();
            }
        }
    }
}
