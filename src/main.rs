use crate::spherical::Spherical;
use bevy::input::mouse::MouseMotion;
use crate::map::Collision;
use crate::map::{on_load_map, add_map, add_light};
use crate::anim::{AnimPos, AnimInfo, PlayerState, 
    STICK_SIZE, MAJOR_HEIGHT, MINOR_HEIGHT, 
    LEFT_ARM, RIGHT_ARM, LEFT_LEG, RIGHT_LEG,
    update_anims, anim_choose_system, spin_sticky_system,
};
use crate::state::{AppState, check_lose_system, game_over_system};
use crate::enemy::{create_enemies, place_enemies_system};
use crate::shot::{PlayerMaterial, ShotMesh, create_shot, create_shot_mesh_system, shot_physics_system,
    remove_shot_system, shot_sticky_collision_check_system,
};
use std::f32::consts::PI;
use bevy::prelude::*;

mod anim;
mod map;
mod state;
mod spherical;
mod enemy;
mod shot;

const VELOCITY: f32 = 0.8;
const JUMP_HEIGHT: f32 = 4.0;

const PLAYER_SHOT_DELAY: f32 = 0.5;

fn main() {
    App::new()
        .init_resource::<Time>()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Loading)
        .add_system_set(
            SystemSet::on_update(AppState::Loading)
                .with_system(on_load_map)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::Playing)
                .with_system(create_player)
                .with_system(create_enemies)
                .with_system(create_shot_mesh_system)
                .with_system(add_light)
                .with_system(add_map)
        )
        .add_system_set(
            SystemSet::on_update(AppState::Playing)
                .with_system(move_player)
                .with_system(update_anims)
                .with_system(spin_sticky_system)
                .with_system(physics_system)
                .with_system(gravity_system)
                .with_system(anim_choose_system)
                .with_system(cursor_grab_system)
                .with_system(player_shoot_system)
                .with_system(move_camera)
                .with_system(check_lose_system)
                .with_system(shot_physics_system)
                .with_system(remove_shot_system)
                .with_system(shot_sticky_collision_check_system)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::GameOver)
                .with_system(game_over_system)
        )
        .run();
}

fn create_player(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let sphere_handle = meshes.add(Mesh::from(shape::UVSphere::default()));

    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.0, 0.0, 0.0),
        unlit: true,
        ..StandardMaterial::default()
    });

    let main_line_handle = meshes.add(Mesh::from(shape::Box::new(STICK_SIZE, MAJOR_HEIGHT, STICK_SIZE)));
    let minor_line_handle = meshes.add(Mesh::from(shape::Box::new(STICK_SIZE, MINOR_HEIGHT, STICK_SIZE)));

    let mut camera_bundle = PerspectiveCameraBundle::new_3d();
    camera_bundle.transform = Transform::from_xyz(0.0, 8.0, -9.0)
        .with_rotation(Quat::from_euler(EulerRot::XYZ, 3.9, 0.0, 3.14));

    //first, sphere head
    commands.spawn_bundle(PbrBundle {
        mesh: sphere_handle,
        material: material_handle.clone(),
        transform: Transform::from_xyz(0.0, MAJOR_HEIGHT + MINOR_HEIGHT, 0.0),
        ..PbrBundle::default()
    })
    .insert(Player)
    .insert(Sticky::Player)
    .insert(Head)
    .insert(Physics {
        hitboxes: vec![
            (Vec3::new(0.0, -3.5, 0.0), Vec3::new(0.4, 6.0, 0.4)),
        ],
        velocity: Vec3::new(0.0, 0.0, 0.0),
        grounded: false,
    })
    .insert(AnimInfo {
        time_takes: 1.0,
        amount_through: 1.0,
        index: 0,
        anim: PlayerState::Idle,
    })
    .with_children(|parent| {

        parent.spawn_bundle(camera_bundle);

        parent.spawn_bundle(PbrBundle {
            mesh: main_line_handle,
            material: material_handle.clone(),
            transform: Transform::from_xyz(0.0, - MAJOR_HEIGHT/2.0 - 1.0, 0.0),
            ..PbrBundle::default()
        })
        .insert(Player)
        .insert(Sticky::Player)
        .with_children(|parent| { 
            parent.spawn_bundle(PbrBundle {
                mesh: minor_line_handle.clone(),
                material: material_handle.clone(),
                ..PbrBundle::default()
            })
            .insert(Player)
            .insert(Sticky::Player)
            .insert(AnimPos::default())
            .insert(LEFT_ARM);

            parent.spawn_bundle(PbrBundle {
                mesh: minor_line_handle.clone(),
                material: material_handle.clone(),
                ..PbrBundle::default()
            })
            .insert(Player)
            .insert(Sticky::Player)
            .insert(AnimPos::default())
            .insert(RIGHT_ARM);
            parent.spawn_bundle(PbrBundle {
                mesh: minor_line_handle.clone(),
                material: material_handle.clone(),
                ..PbrBundle::default()
            })
            .insert(Player)
            .insert(Sticky::Player)
            .insert(AnimPos::default())
            .insert(LEFT_LEG);
            
            parent.spawn_bundle(PbrBundle {
                mesh: minor_line_handle,
                material: material_handle.clone(),
                ..PbrBundle::default()
            })
            .insert(Player)
            .insert(Sticky::Player)
            .insert(AnimPos::default())
            .insert(RIGHT_LEG);
        });
    });

    commands.insert_resource(PlayerMaterial {
        player_mat: material_handle,
    });
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component, PartialEq, Eq)]
pub enum Sticky {
    Player,
    Enemy,
}

#[derive(Component)]
pub struct Head;

#[derive(Component)]
pub struct Physics {
    pub hitboxes: Vec<(Vec3, Vec3)>,
    pub velocity: Vec3,
    pub grounded: bool,
}

pub fn rotate_around(transform: &mut Transform, point: Vec3, rotation: Quat) {
    transform.translation = point + rotation * (transform.translation - point);
    transform.rotation *= rotation;
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Physics, &GlobalTransform), (With<Player>, With<Head>)>,
) {
    let (mut physics, trans) = query.get_single_mut().unwrap();

    let velocity = if physics.grounded {
        VELOCITY
    } else {
        VELOCITY * 0.2
    };

    if keyboard_input.pressed(KeyCode::W) {
        physics.velocity += trans.rotation * Vec3::new(0.0, 0.0, velocity);
    }
    if keyboard_input.pressed(KeyCode::A) {
        physics.velocity += trans.rotation * Vec3::new(velocity, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::D) {
        physics.velocity += trans.rotation * Vec3::new(-velocity, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::S) {
        physics.velocity += trans.rotation * Vec3::new(0.0, 0.0, -velocity);
    }
    if keyboard_input.pressed(KeyCode::Space) && physics.grounded {
        physics.velocity += trans.rotation * Vec3::new(0.0, JUMP_HEIGHT, 0.0);
    }
}

fn player_shoot_system(
    mut commands: Commands,
    shot_mesh: Res<ShotMesh>,
    player_mat: Res<PlayerMaterial>,
    mouse_input: Res<Input<MouseButton>>,
    player_query: Query<&Transform, (With<Player>, With<Head>)>,
    camera_query: Query<&GlobalTransform, (With<Camera>, Without<Head>)>,
    mut shot_cooldown: Local<ShotCooldown>,
    time: Res<Time>
) {
    shot_cooldown.cooldown -= time.delta_seconds();
    if mouse_input.just_pressed(MouseButton::Left) && shot_cooldown.cooldown <= 0.0 {
        let player_trans = player_query.get_single().unwrap();
        let camera_trans = camera_query.get_single().unwrap();
        create_shot(
            &mut commands,
            shot_mesh.shot_handle.clone(),
            player_mat.player_mat.clone(),
            player_trans.translation,
            camera_trans.rotation * Vec3::new(0.0, 0.0, -20.0),
            Sticky::Player,
        );
        shot_cooldown.cooldown = PLAYER_SHOT_DELAY;
    }
}

#[derive(Default)]
struct ShotCooldown {
    cooldown: f32,
}

fn move_camera(
    windows: Res<Windows>,
    mut ev_motion: EventReader<MouseMotion>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut player_query: Query<&mut Transform, (With<Player>, With<Head>, Without<Camera>)>,
) {
    let mut camera_transform = camera_query.get_single_mut().unwrap();

    let mut player_trans = player_query.get_single_mut().unwrap();

    let mut orbit = Vec2::ZERO;
    for motion in ev_motion.iter() {
        orbit += motion.delta;
    }

    let window = get_primary_window_size(&windows);
    let delta_x = orbit.x / window.x * PI * 2.0;
    let delta_y = -orbit.y / window.y * PI;
    let yaw = Quat::from_rotation_y(-delta_x);

    let (mut x, y, z) = camera_transform.rotation.to_euler(EulerRot::XYZ);
    x = add_clamp_around(x, -delta_y, PI/2.0, 0.1);
    camera_transform.rotation = Quat::from_euler(EulerRot::XYZ, x, y, z);

    player_trans.rotation = yaw * player_trans.rotation; // rotate around global y axis


    let mut cam_sphere = Spherical::from_vec3(camera_transform.translation);
    cam_sphere.phi = f32::clamp(cam_sphere.phi + delta_y, 0.1, PI - 0.1);
    camera_transform.translation = cam_sphere.to_vec3();

}

//awful
fn add_clamp_around(val: f32, change: f32, pos: f32, clamp_val: f32) -> f32 {
    let new_val = val + change;
    if val < -pos {
        if new_val > -pos - clamp_val {
            return -pos - clamp_val;
        }
    } else if val > pos{
        if new_val < pos + clamp_val {
            return pos + clamp_val;
        }
    } else {
        if new_val > pos - clamp_val {
            return pos - clamp_val;
        }
        if new_val < -pos + clamp_val {
            return -pos + clamp_val;
        }
    }
    return new_val;
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}

fn cursor_grab_system(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();

    if btn.just_pressed(MouseButton::Left) {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }

    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }

    if window.cursor_locked() {
        window.set_cursor_position(Vec2::new(window.physical_width() as f32, window.physical_height() as f32));
    }
}

fn gravity_system(
    mut query: Query<&mut Physics>,
) {
    for mut physics in query.iter_mut() {
        physics.velocity.y -= 0.2;
    }
}

fn physics_system(
    mut query: Query<(&mut Transform, &mut Physics, &GlobalTransform), Without<Collision>>,
    collision_query: Query<&Transform, With<Collision>>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();
    for (mut transform, mut physics, global_trans) in query.iter_mut() {

        transform.translation += physics.velocity * delta;

        physics.grounded = false;

        for collision_trans in collision_query.iter() {
            //handle collision rudimentary
            for &(trans_hitbox, scale_hitbox) in physics.hitboxes.iter() {
                let hitbox_pos = trans_hitbox + global_trans.translation;
                if detect_collision((hitbox_pos, scale_hitbox), (collision_trans.translation, collision_trans.scale)) {
                    //move player to nearest edge
                    
                    let player_top = hitbox_pos.y + scale_hitbox.y/2.0;
                    let player_bottom = hitbox_pos.y - scale_hitbox.y/2.0;

                    let collision_top = collision_trans.translation.y + collision_trans.scale.y/2.0;
                    let collision_bottom = collision_trans.translation.y - collision_trans.scale.y/2.0;

                    let up_diff = collision_top - player_bottom;
                    let down_diff = player_top - collision_bottom;

                    let player_left = hitbox_pos.x + scale_hitbox.x/2.0;
                    let player_right = hitbox_pos.x - scale_hitbox.x/2.0;

                    let collision_left = collision_trans.translation.x + collision_trans.scale.x/2.0;
                    let collision_right = collision_trans.translation.x - collision_trans.scale.x/2.0;

                    let left_diff = collision_left - player_right;
                    let right_diff = player_left - collision_right;

                    let player_forward = hitbox_pos.z + scale_hitbox.z/2.0;
                    let player_back = hitbox_pos.z - scale_hitbox.z/2.0;

                    let collision_forward = collision_trans.translation.z + collision_trans.scale.z/2.0;
                    let collision_back = collision_trans.translation.z - collision_trans.scale.z/2.0;

                    let forward_diff = collision_forward - player_back;
                    let back_diff = player_forward - collision_back;

                    let array = [up_diff, down_diff, left_diff, right_diff, forward_diff, back_diff];
                    let (min_index, min) = array.as_slice().iter().enumerate().reduce(|accum, item| {
                        if accum.1 <= item.1 {
                            accum
                        } else {
                            item
                        }
                    }).unwrap();

                    match min_index {
                        0 => {
                            transform.translation.y += min;
                            physics.grounded = true;
                            physics.velocity.y = f32::max(0.0, physics.velocity.y);
                            physics.velocity *= 0.9;
                        },
                        1 => {
                            transform.translation.y -= min;
                            physics.velocity.y = f32::min(0.0, physics.velocity.y);
                        },
                        2 => {
                            transform.translation.x += min;
                            physics.velocity.x = f32::max(0.0, physics.velocity.x);
                        },
                        3 => {
                            transform.translation.x -= min;
                            physics.velocity.x = f32::min(0.0, physics.velocity.x);
                        },
                        4 => {
                            transform.translation.z += min;
                            physics.velocity.z = f32::max(0.0, physics.velocity.z);
                        },
                        5 => {
                            transform.translation.z -= min;
                            physics.velocity.z = f32::min(0.0, physics.velocity.z);
                        },
                        _ => unreachable!(),
                    }

                    break;
                }
            }
        }
    }
}

pub fn detect_collision(hitbox: (Vec3, Vec3), wall: (Vec3, Vec3)) -> bool {
    
    if f32::abs(hitbox.0.x - wall.0.x) <= hitbox.1.x/2.0 + wall.1.x/2.0 &&
       f32::abs(hitbox.0.y - wall.0.y) <= hitbox.1.y/2.0 + wall.1.y/2.0 &&
       f32::abs(hitbox.0.z - wall.0.z) <= hitbox.1.z/2.0 + wall.1.z/2.0 {

        return true;
   }
   return false;
}