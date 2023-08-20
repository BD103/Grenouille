use bevy::prelude::*;

pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_title);
    }
}

#[derive(Component, Debug)]
pub struct Title;

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
