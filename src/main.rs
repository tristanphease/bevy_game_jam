use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    let camera_bundle = PerspectiveCameraBundle::new_3d();
    commands.spawn_bundle(camera_bundle);
}