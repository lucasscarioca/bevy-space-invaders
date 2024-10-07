use bevy::prelude::*;

pub mod game;
pub mod alien;
pub mod resolution;
pub mod player;
pub mod projectile;

fn main() {
    App::new()
        .add_plugins(
            // List of plugins
            (DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Space Invaders"),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                resolution: Vec2::new(512., 512.).into(),
                ..default()
            }),
            ..default()
        })
            .set(ImagePlugin::default_nearest()),
            game::GamePlugin)
        )
        .run();
}
