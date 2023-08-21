use bevy::prelude::*;

/// The required title plugin.
pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_title);
    }
}

/// The title component tag.
#[derive(Component, Debug)]
pub struct Title;

/// Spawns in the title on [`Startup`].
pub fn spawn_title(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Title,
        SpriteBundle {
            texture: asset_server.load("title.png"),
            transform: Transform {
                translation: Vec3::new(8.0, 128.0, 0.0),
                scale: Vec3::new(2.0, 2.0, 1.0),
                ..default()
            },
            ..default()
        },
    ));
}
