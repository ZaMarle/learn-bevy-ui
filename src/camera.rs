use bevy::prelude::*;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_camera);
    }
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0)
                .looking_at(
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::Y,
                ),
            ..default()
        },
        MainCamera,
    ));
}
