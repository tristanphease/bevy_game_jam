use bevy::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut camera_bundle = PerspectiveCameraBundle::new_3d();
    camera_bundle.transform = Transform::from_xyz(0.0, 0.0, 10.0);
    

    commands.spawn_bundle(camera_bundle);
    create_player(&mut commands, &mut meshes, &mut materials);
}

fn create_player(
    commands: &mut Commands, 
    meshes: &mut ResMut<Assets<Mesh>>, 
    materials: &mut ResMut<Assets<StandardMaterial>>
) {

    const HEIGHT: f32 = 6.0;

    let sphere_handle = meshes.add(Mesh::from(shape::UVSphere::default()));

    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.0, 0.0, 0.0),
        ..StandardMaterial::default()
    });

    let line_handle = meshes.add(Mesh::from(shape::Box::new(0.1, HEIGHT, 0.1)));

    //first, sphere head
    commands.spawn_bundle(PbrBundle {
        mesh: sphere_handle,
        material: material_handle.clone(),
        transform: Transform::from_xyz(0.0, 3.0, 0.0),
        ..PbrBundle::default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(PbrBundle {
            mesh: line_handle.clone(),
            material: material_handle.clone(),
            transform: Transform::from_xyz(0.0, -HEIGHT/2.0, 0.0),
            ..PbrBundle::default()
        })
        .with_children(|parent| { 
            parent.spawn_bundle(PbrBundle {
                mesh: line_handle.clone(),
                material: material_handle.clone(),
                transform: Transform::from_rotation(Quat::from_rotation_z(PI/2.0)),
                ..PbrBundle::default()
            });
        });
    });



}