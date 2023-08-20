use bevy::prelude::*;

pub struct FrogPlugin;

impl Plugin for FrogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_frog)
            .add_systems(Update, animate_sprite);
    }
}

#[derive(Component, Debug)]
pub struct Frog;

#[derive(Component, Debug)]
pub struct FrogBody;

#[derive(Component, Debug)]
pub struct FrogEyes;

#[derive(Component, Debug)]
pub struct FrogMouth;

pub fn spawn_frog(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("frog.png");
    let atlas = TextureAtlas::from_grid(texture_handle, Vec2::splat(128.0), 4, 3, None, None);
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
            FrogEyes,
            // Skip 6th frame
            AnimationIndices { first: 4, last: 5 },
            atlas_handle.clone(),
        ))
        .with_children(create_child(
            FrogMouth,
            AnimationIndices { first: 8, last: 10 },
            atlas_handle,
        ));
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
