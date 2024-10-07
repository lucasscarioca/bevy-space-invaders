use bevy::prelude::*;

use crate::resolution;
use crate::projectile;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, setup_player)
      .add_systems(Update, update_player);
  }
}

#[derive(Component)]
struct Player {
  // Provides cooldown for shooting so we don't just shoot a bullet every frame
  pub shoot_timer: f32,
}

fn setup_player(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  resolution: Res<resolution::Resolution>,
) {
  let player_image = asset_server.load("player.png");
  commands.spawn((
    SpriteBundle {
      texture: player_image,
      transform: Transform::from_xyz(0., -(resolution.screen_dimensions.y * 0.5) + (resolution.pixel_ratio * 5.0), 0.).with_scale(Vec3::splat(resolution.pixel_ratio)),
      ..default()
    },
    Player {
      shoot_timer: 0.
    }
  ));
}

const SPEED: f32 = 200.;
const BULLET_SPEED: f32 = 400.;
const SHOOT_COOLDOWN: f32 = 0.5;


fn update_player(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut player_query: Query<(&mut Player, &mut Transform)>,
  time: Res<Time>,
  keys: Res<ButtonInput<KeyCode>>,
  resolution: Res<resolution::Resolution>,
) {
  // query for the only instace of the player
  let (mut player, mut transform) = player_query.single_mut();

  // The input which the player is pressing for the horizontal axis
  let mut horizontal = 0.;
  if keys.pressed(KeyCode::KeyA) {
    horizontal += -1.;
  }
  if keys.pressed(KeyCode::KeyD) {
    horizontal += 1.;
  }

  // Move player
  transform.translation.x += horizontal * time.delta_seconds() * SPEED;
  // Confine player
  let left_bound = -resolution.screen_dimensions.x * 0.5;
  let right_bound = resolution.screen_dimensions.x * 0.5;
  transform.translation.x = transform.translation.x.clamp(left_bound, right_bound);

  player.shoot_timer -= time.delta_seconds();

  if keys.pressed(KeyCode::Space) && player.shoot_timer <= 0. {
    // Shoot
    player.shoot_timer = SHOOT_COOLDOWN;
    let bullet_texture = asset_server.load("bullet.png");
    commands.spawn((
      SpriteBundle {
        texture: bullet_texture,
        transform: Transform::from_translation(transform.translation).with_scale(Vec3::splat(resolution.pixel_ratio)),
        ..default()
      },
      projectile::Projectile {
        speed: BULLET_SPEED,
      },
    ));
  }
}
