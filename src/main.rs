use bevy::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .init_resource::<Player>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(SystemSet::new().with_system(move_player))
        .run();
}

fn setup(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut player: ResMut<Player>,
) {
    
    create_player(&mut commands, &mut meshes, &mut materials, &mut player);
}

fn create_player(
    commands: &mut Commands, 
    meshes: &mut ResMut<Assets<Mesh>>, 
    materials: &mut ResMut<Assets<StandardMaterial>>,
    player: &mut ResMut<Player>,
) {

    const HEIGHT: f32 = 5.0;
    const SMALL_SCALE: f32 = 0.5;
    const ARM_POS: f32 = 2.0;

    let sphere_handle = meshes.add(Mesh::from(shape::UVSphere::default()));

    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.0, 0.0, 0.0),
        ..StandardMaterial::default()
    });

    let line_handle = meshes.add(Mesh::from(shape::Box::new(0.1, HEIGHT, 0.1)));


    let mut left_arm_transform = Transform::from_translation(Vec3::new(0.0, ARM_POS, 0.0));
    left_arm_transform.scale = Vec3::new(1.0, SMALL_SCALE, 1.0);

    rotate_around(&mut left_arm_transform, Vec3::new(0.0, ARM_POS - SMALL_SCALE * HEIGHT / 2.0, 0.0), Quat::from_rotation_z(PI * 0.5));

    let mut right_arm_transform = Transform::from_translation(Vec3::new(0.0, ARM_POS, 0.0));
    right_arm_transform.scale = Vec3::new(1.0, SMALL_SCALE, 1.0);

    rotate_around(&mut right_arm_transform, Vec3::new(0.0, ARM_POS - SMALL_SCALE * HEIGHT / 2.0, 0.0), Quat::from_rotation_z(PI * 1.2));

    let mut left_leg_transform = Transform::from_translation(Vec3::new(0.0, - HEIGHT/2.0 - SMALL_SCALE * HEIGHT/2.0, 0.0));
    left_leg_transform.scale = Vec3::new(1.0, SMALL_SCALE, 1.0);

    rotate_around(&mut left_leg_transform, Vec3::new(0.0, -HEIGHT/2.0, 0.0), Quat::from_rotation_z(-PI * 0.2));

    let mut right_leg_transform = Transform::from_translation(Vec3::new(0.0, - HEIGHT/2.0 - SMALL_SCALE * HEIGHT/2.0, 0.0));
    right_leg_transform.scale = Vec3::new(1.0, SMALL_SCALE, 1.0);

    rotate_around(&mut right_leg_transform, Vec3::new(0.0, -HEIGHT/2.0, 0.0), Quat::from_rotation_z(PI * 0.2));

    let mut camera_bundle = PerspectiveCameraBundle::new_3d();
    camera_bundle.transform = Transform::from_xyz(0.0, 0.0, 10.0);
   
    //first, sphere head
    player.head = Some(commands.spawn_bundle(PbrBundle {
        mesh: sphere_handle,
        material: material_handle.clone(),
        transform: Transform::from_xyz(0.0, HEIGHT/2.0, 0.0),
        ..PbrBundle::default()
    })
    .with_children(|parent| {

        parent.spawn_bundle(camera_bundle);

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
                transform: left_arm_transform,
                ..PbrBundle::default()
            });
            parent.spawn_bundle(PbrBundle {
                mesh: line_handle.clone(),
                material: material_handle.clone(),
                transform: right_arm_transform,
                ..PbrBundle::default()
            });
            parent.spawn_bundle(PbrBundle {
                mesh: line_handle.clone(),
                material: material_handle.clone(),
                transform: left_leg_transform,
                ..PbrBundle::default()
            });
            parent.spawn_bundle(PbrBundle {
                mesh: line_handle.clone(),
                material: material_handle.clone(),
                transform: right_leg_transform,
                ..PbrBundle::default()
            });
        });
    }).id());
}

#[derive(Default)]
struct Player {
    head: Option<Entity>,
}

fn rotate_around(transform: &mut Transform, point: Vec3, rotation: Quat) {
    transform.translation = point + rotation * (transform.translation - point);
    transform.rotation *= rotation;
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut transforms: Query<&mut Transform>,
    player: ResMut<Player>,
) {
    if keyboard_input.pressed(KeyCode::W) {
        transforms.get_mut(player.head.unwrap()).unwrap().translation.z -= 0.5;
    }
    if keyboard_input.pressed(KeyCode::A) {
        transforms.get_mut(player.head.unwrap()).unwrap().translation.x -= 0.5;
    }
    if keyboard_input.pressed(KeyCode::D) {
        transforms.get_mut(player.head.unwrap()).unwrap().translation.x += 0.5;
    }
    if keyboard_input.pressed(KeyCode::S) {
        transforms.get_mut(player.head.unwrap()).unwrap().translation.z += 0.5;
    }
}