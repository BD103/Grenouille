use super::AnimationIndices;
use bevy::prelude::*;
use std::time::Duration;

pub const OPEN_INDICES: AnimationIndices = AnimationIndices { first: 3, last: 4 };
pub const BLINK_INDICES: AnimationIndices = AnimationIndices::splat(5);

pub const OPEN_DURATION: f32 = 5.0;
pub const BLINK_DURATION: f32 = 0.2;

#[derive(Component, Default, Debug)]
pub enum FrogEyes {
    #[default]
    Open,
    // Closed,
    Blinking,
}

#[derive(Component, Deref, DerefMut, Debug)]
pub struct BlinkTimer(Timer);

impl Default for BlinkTimer {
    fn default() -> Self {
        BlinkTimer(Timer::from_seconds(OPEN_DURATION, TimerMode::Once))
    }
}

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

                blink_timer.set_duration(Duration::from_secs_f32(BLINK_DURATION));
                blink_timer.reset();
            }
            FrogEyes::Blinking => {
                *frog_eyes = FrogEyes::Open;
                *indices = OPEN_INDICES;
                sprite.index = 3;

                blink_timer.set_duration(Duration::from_secs_f32(OPEN_DURATION));
                blink_timer.reset();
            }
        }
    }
}
