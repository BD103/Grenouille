use bevy::prelude::*;

/// The required animation plugin.
pub struct AnimatePlugin;

impl Plugin for AnimatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprite);
    }
}

/// Represents a [`TextureAtlasSprite`]'s animation indices.
///
/// This should be used alongside [`AnimationTimer`] and [`SpriteSheetBundle`].
#[derive(Component, Debug)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

impl AnimationIndices {
    /// Creates a new [`AnimationIndices`] where `first` and `last` are the same.
    pub const fn splat(indice: usize) -> Self {
        AnimationIndices {
            first: indice,
            last: indice,
        }
    }
}

/// A [`Timer`] wrapped used to trigger an animation.
///
/// This should be used alongside [`AnimationIndices`] and [`SpriteSheetBundle`]. The indice steps
/// forward every time the timer finishes.
#[derive(Component, Deref, DerefMut, Debug)]
pub struct AnimationTimer(pub Timer);

/// Animates any entity with a texture atlas, [`AnimationIndices`], and [`AnimationTimer`].
pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}
