use bevy::prelude::*;
use bevy_flycam::{NoCameraPlayerPlugin, FlyCam};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(NoCameraPlayerPlugin)
    .add_startup_system(setup_cam)
    .add_startup_system(spawn_cubes)
    .run()
}

fn setup_cam(
    mut commands: Commands,
) {
    commands.spawn((Camera3dBundle::default(), FlyCam));
}

fn spawn_cubes(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
) {
    let mesh = mesh_assets.add(Cuboid::new(1., 1., 1.));
    for x in -10..10 {
        for z in -10..10 {
            commands.spawn(PbrBundle {
                mesh: mesh.clone(),
                transform: Transform::from_translation(Vec3::new(x as f32 * 2., 0., z as f32 * 2.)),
                ..Default::default()
            });
        }
    }
}
