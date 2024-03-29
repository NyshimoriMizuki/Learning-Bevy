use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const PLAYER_SPEED: f32 = 200.0;
const PLAYER_RADIUS: f32 = 32.0;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(move_player)
            .add_system(confine_player_movement);
    }

    fn setup(&self, _app: &mut App) {
        // do nothing
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    fn is_unique(&self) -> bool {
        true
    }
}

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut command: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assets: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    command.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: assets.load("sprites/ball_blue_large.png"),
            ..Default::default()
        },
        Player,
    ));
}

fn move_player(
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard.pressed(KeyCode::Left) || keyboard.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard.pressed(KeyCode::Right) || keyboard.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard.pressed(KeyCode::Up) || keyboard.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard.pressed(KeyCode::Down) || keyboard.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let win = window_query.get_single().unwrap();
    let min_xy = 0.0 + PLAYER_RADIUS;
    let max_xy = (win.width() - PLAYER_RADIUS, win.height() - PLAYER_RADIUS);

    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut translation = transform.translation;

        if translation.x < min_xy {
            translation.x = min_xy;
        } else if translation.x > max_xy.0 {
            translation.x = max_xy.0;
        }
        if translation.y < min_xy {
            translation.y = min_xy;
        } else if translation.y > max_xy.1 {
            translation.y = max_xy.1;
        }

        transform.translation = translation;
    }
}
