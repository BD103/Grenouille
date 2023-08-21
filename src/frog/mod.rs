pub mod eyes;

use crate::animate::{AnimationIndices, AnimationTimer};
use bevy::prelude::*;

/// The required frog plugin.
///
/// This spawns the frog and animates it.
pub struct FrogPlugin;

impl Plugin for FrogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_frog)
            .add_systems(Update, eyes::blink_frog);
    }
}

/// The frog component tag.
#[derive(Component, Debug)]
pub struct Frog;

/// The frog body component tag.
///
/// This is a child of [`Frog`].
#[derive(Component, Debug)]
pub struct FrogBody;

/// The frog mouth component tag.
///
/// This is a child of [`Frog`].
#[derive(Component, Debug)]
pub struct FrogMouth;

/// Spawns in a [`Frog`] and children at [`Startup`].
pub fn spawn_frog(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("frog.png");
    let atlas = TextureAtlas::from_grid(texture_handle, Vec2::splat(128.0), 4, 4, None, None);
    let atlas_handle = texture_atlases.add(atlas);

    fn create_child(
        tag: impl Component,
        indices: AnimationIndices,
        atlas_handle: Handle<TextureAtlas>,
        extras: impl Bundle,
    ) -> impl FnOnce(&mut ChildBuilder) {
        |parent| {
            parent.spawn((
                tag,
                SpriteSheetBundle {
                    texture_atlas: atlas_handle,
                    sprite: TextureAtlasSprite::new(indices.first),
                    ..default()
                },
                indices,
                AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
                extras,
            ));
        }
    }

    commands
        .spawn((
            Frog,
            TransformBundle {
                local: Transform {
                    translation: Vec3::new(0.0, -32.0, 0.0),
                    scale: Vec3::new(2.0, 2.0, 1.0),
                    ..default()
                },
                ..default()
            },
            VisibilityBundle::default(),
        ))
        .with_children(create_child(
            FrogBody,
            AnimationIndices { first: 0, last: 2 },
            atlas_handle.clone(),
            (),
        ))
        .with_children(create_child(
            eyes::FrogEyes::default(),
            // Skip 6th frame
            eyes::OPEN_INDICES,
            atlas_handle.clone(),
            (),
        ))
        .with_children(create_child(
            FrogMouth,
            AnimationIndices {
                first: 12,
                last: 14,
            },
            atlas_handle,
            (),
        ));
}
