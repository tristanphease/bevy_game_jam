use crate::spherical::Spherical;
use bevy::input::mouse::MouseMotion;
use crate::map::Collision;
use crate::map::{on_load_map, add_map, add_light};
use crate::anim::{AnimPos, PlayerState, 
    STICK_SIZE, MAJOR_HEIGHT, MINOR_HEIGHT, 
    LEFT_ARM, RIGHT_ARM, LEFT_LEG, RIGHT_LEG, Limb,
    get_trans_from_pos
};
use crate::state::AppState;
use std::f32::consts::PI;
use bevy::prelude::*;

mod anim;
mod map;
mod state;
mod spherical;

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
                .with_system(add_light)
                .with_system(add_map)
                     
        )
        .add_system_set(
            SystemSet::on_update(AppState::Playing)
                .with_system(move_player)
                .with_system(update_anims)
                .with_system(physics_system)
                .with_system(gravity_system)
                .with_system(cursor_grab_system)
                .with_system(move_camera)
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
    camera_bundle.transform = Transform::from_xyz(0.0, 10.0, -9.0)
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
            (Vec3::new(0.0, -3.5, 0.0), Vec3::new(0.25, 5.0, 0.25)),
            (Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0)),
        ],
        velocity: Vec3::new(0.0, 0.0, 0.0),
        grounded: false,
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
                material: material_handle,
                ..PbrBundle::default()
            })
            .insert(Player)
            .insert(Sticky::Player)
            .insert(AnimPos::default())
            .insert(RIGHT_LEG);
        });
    });

    commands.spawn().insert(AnimInfo {
        time_takes: 1.0,
        amount_through: 1.0,
        index: 0,
        anim: PlayerState::Idle,
    })
    .insert(Sticky::Player);
}

#[derive(Component)]
struct AnimInfo {
    time_takes: f32,
    amount_through: f32,
    index: usize,
    anim: PlayerState,
}

impl AnimInfo {
    pub fn add_time(&mut self, delta_time: f32) -> bool {
        self.amount_through = self.amount_through + delta_time / self.time_takes;

        if self.amount_through > 1.0 {
            //go to next anim
            self.amount_through = self.amount_through - 1.0;
            self.index = self.index + 1;
            if self.index == self.anim.get_anim_num_frames() {
                self.index = 0;
            }
            return true;
        }
        return false;
    }
}

#[derive(Component)]
struct Player;

#[derive(Component, PartialEq, Eq)]
enum Sticky {
    Player,
    Enemy,
}

#[derive(Component)]
struct Head;

#[derive(Component)]
struct Physics {
    hitboxes: Vec<(Vec3, Vec3)>,
    velocity: Vec3,
    grounded: bool,
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
    if keyboard_input.pressed(KeyCode::W) {
        physics.velocity += trans.rotation * Vec3::new(0.0, 0.0, 0.5);
    }
    if keyboard_input.pressed(KeyCode::A) {
        physics.velocity += trans.rotation * Vec3::new(0.5, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::D) {
        physics.velocity += trans.rotation * Vec3::new(-0.5, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::S) {
        physics.velocity += trans.rotation * Vec3::new(0.0, 0.0, -0.5);
    }
    if keyboard_input.pressed(KeyCode::Space) && physics.grounded {
        physics.velocity += trans.rotation * Vec3::new(0.0, 4.0, 0.0);
    }
}

fn update_anims(
    mut player_query: Query<(&mut AnimInfo, &Sticky)>,
    mut query: Query<(&mut Transform, &mut AnimPos, &Limb, &Sticky)>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();

    for (mut anim_info, player_sticky) in player_query.iter_mut() {
        let change = anim_info.add_time(delta);
        for (mut transform, mut anim_pos, limb, sticky) in query.iter_mut() {
            if sticky == player_sticky {
                if change {
                    anim_pos.change_pos(anim_info.anim, *limb, anim_info.index);
                }
                *transform = get_trans_from_pos(*limb, anim_pos.calc_curr_pos(anim_info.amount_through));
            }
        }
    }
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

fn detect_collision(hitbox: (Vec3, Vec3), wall: (Vec3, Vec3)) -> bool {
    
    if f32::abs(hitbox.0.x - wall.0.x) <= hitbox.1.x/2.0 + wall.1.x/2.0 &&
       f32::abs(hitbox.0.y - wall.0.y) <= hitbox.1.y/2.0 + wall.1.y/2.0 &&
       f32::abs(hitbox.0.z - wall.0.z) <= hitbox.1.z/2.0 + wall.1.z/2.0 {

        return true;
   }
   return false;
}