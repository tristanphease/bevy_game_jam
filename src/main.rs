use crate::anim::{AnimInfo, STICK_SIZE, MAJOR_HEIGHT, MINOR_HEIGHT};
use bevy::prelude::*;
use std::f32::consts::PI;

mod anim;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(SystemSet::new().with_system(move_player))
        .run();
}

fn setup(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    
    create_player(&mut commands, &mut meshes, &mut materials);
}

fn create_player(
    commands: &mut Commands, 
    meshes: &mut ResMut<Assets<Mesh>>, 
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {

    let sphere_handle = meshes.add(Mesh::from(shape::UVSphere::default()));

    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.0, 0.0, 0.0),
        ..StandardMaterial::default()
    });

    let main_line_handle = meshes.add(Mesh::from(shape::Box::new(STICK_SIZE, MAJOR_HEIGHT, STICK_SIZE)));
    let minor_line_handle = meshes.add(Mesh::from(shape::Box::new(STICK_SIZE, MINOR_HEIGHT, STICK_SIZE)));

    let mut camera_bundle = PerspectiveCameraBundle::new_3d();
    camera_bundle.transform = Transform::from_xyz(0.0, -2.0, 10.0);

    commands.spawn_bundle(camera_bundle);

    let mut left_arm = None;
    let mut right_arm = None;
    let mut left_leg = None;
    let mut right_leg = None;
   
    //first, sphere head
    let head = commands.spawn_bundle(PbrBundle {
        mesh: sphere_handle,
        material: material_handle.clone(),
        transform: Transform::from_xyz(0.0, MAJOR_HEIGHT/2.0 + 1.0, 0.0),
        ..PbrBundle::default()
    }).with_children(|parent| {

        parent.spawn_bundle(PbrBundle {
            mesh: main_line_handle,
            material: material_handle.clone(),
            transform: Transform::from_xyz(0.0, - MAJOR_HEIGHT/2.0 - 1.0, 0.0),
            ..PbrBundle::default()
        })
        .with_children(|parent| { 
            left_arm = Some(parent.spawn_bundle(PbrBundle {
                mesh: minor_line_handle.clone(),
                material: material_handle.clone(),
                ..PbrBundle::default()
            }).id());
            right_arm = Some(parent.spawn_bundle(PbrBundle {
                mesh: minor_line_handle.clone(),
                material: material_handle.clone(),
                ..PbrBundle::default()
            }).id());
            left_leg = Some(parent.spawn_bundle(PbrBundle {
                mesh: minor_line_handle.clone(),
                material: material_handle.clone(),
                ..PbrBundle::default()
            }).id());
            right_leg = Some(parent.spawn_bundle(PbrBundle {
                mesh: minor_line_handle,
                material: material_handle,
                ..PbrBundle::default()
            }).id());
        });
    }).id();

    let player = Player {
        head,
        left_arm: left_arm.unwrap(),
        left_arm_anim: AnimInfo::default(),
        right_arm: right_arm.unwrap(),
        right_arm_anim: AnimInfo::default(),
        left_leg: left_leg.unwrap(),
        left_leg_anim: AnimInfo::default(),
        right_leg: right_leg.unwrap(),
        right_leg_anim: AnimInfo::default(),
    };

    commands.spawn().insert(player);
}

#[derive(Component)]
pub struct Player {
    head: Entity,
    left_arm: Entity,
    left_arm_anim: AnimInfo,
    right_arm: Entity,
    right_arm_anim: AnimInfo,
    left_leg: Entity,
    left_leg_anim: AnimInfo,
    right_leg: Entity,
    right_leg_anim: AnimInfo,
}

pub fn rotate_around(transform: &mut Transform, point: Vec3, rotation: Quat) {
    transform.translation = point + rotation * (transform.translation - point);
    transform.rotation *= rotation;
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut transforms: Query<&mut Transform>,
    mut player: Query<&mut Player>,
) {
    let mut player = player.get_single_mut().unwrap();
    if keyboard_input.pressed(KeyCode::W) {
        transforms.get_mut(player.head).unwrap().translation.z -= 0.5;
    }
    if keyboard_input.pressed(KeyCode::A) {
        transforms.get_mut(player.head).unwrap().translation.x -= 0.5;
    }
    if keyboard_input.pressed(KeyCode::D) {
        transforms.get_mut(player.head).unwrap().translation.x += 0.5;
    }
    if keyboard_input.pressed(KeyCode::S) {
        transforms.get_mut(player.head).unwrap().translation.z += 0.5;
    }
    if keyboard_input.pressed(KeyCode::G) {
        anim::set_idle(transforms, &mut player);
    }
}