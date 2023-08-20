use bevy::prelude::*;
use std::time::Duration;

pub struct FrogPlugin;

impl Plugin for FrogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_frog)
            .add_systems(Update, (animate_sprite, blink_frog));
    }
}

#[derive(Component, Debug)]
pub struct Frog;

#[derive(Component, Debug)]
pub struct FrogBody;

#[derive(Component, Default, Debug)]
pub enum FrogEyes {
    #[default]
    Open,
    // Closed,
    Blinking,
}

#[derive(Component, Debug)]
pub struct FrogMouth;

pub fn spawn_frog(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("frog.png");
    let atlas = TextureAtlas::from_grid(texture_handle, Vec2::splat(128.0), 3, 3, None, None);
    let atlas_handle = texture_atlases.add(atlas);

    fn create_child(
        tag: impl Component,
        indices: AnimationIndices,
        atlas_handle: Handle<TextureAtlas>,
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
        ))
        .with_children(create_child(
            FrogEyes::default(),
            // Skip 6th frame
            AnimationIndices { first: 3, last: 4 },
            atlas_handle.clone(),
        ))
        .with_children(create_child(
            FrogMouth,
            AnimationIndices { first: 6, last: 8 },
            atlas_handle,
        ));
}

#[derive(Component, Deref, DerefMut, Debug)]
pub struct BlinkTimer(Timer);

impl Default for BlinkTimer {
    fn default() -> Self {
        BlinkTimer(Timer::from_seconds(5.0, TimerMode::Once))
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
                *indices = AnimationIndices { first: 5, last: 5 };
                sprite.index = 5;

                blink_timer.set_duration(Duration::from_secs_f32(0.2));
                blink_timer.reset();
            }
            FrogEyes::Blinking => {
                *frog_eyes = FrogEyes::Open;
                *indices = AnimationIndices { first: 3, last: 4 };
                sprite.index = 3;

                blink_timer.set_duration(Duration::from_secs(5));
                blink_timer.reset();
            }
        }
    }
}

#[derive(Component, Debug)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut, Debug)]
pub struct AnimationTimer(Timer);

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
