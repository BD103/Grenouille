#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod frog;

use bevy::prelude::*;
use frog::FrogPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Frog :3".into(),
                    resolution: (128.0 * 6.0, 128.0 * 4.0).into(),
                    ..default()
                }),
                ..default()
            }),
        FrogPlugin,
    ))
    .insert_resource(ClearColor(Color::rgb_u8(0xff, 0xb0, 0xbf)))
    .add_systems(Startup, setup);

    #[cfg(debug_assertions)]
    {
        use bevy::window::close_on_esc;
        app.add_systems(Update, close_on_esc);
    }

    app.run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
