use crate::anim::get_trans_from_pos;
use crate::anim::{AnimPos, PlayerState, STICK_SIZE, MAJOR_HEIGHT, MINOR_HEIGHT, 
    LEFT_ARM, RIGHT_ARM, LEFT_LEG, RIGHT_LEG, Limb};
use bevy::prelude::*;

mod anim;

fn main() {
    App::new()
        .init_resource::<Time>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(move_player)
        .add_system(update_anims)
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
    //camera_bundle.transform = Transform::from_xyz(0.0, 10.0, -9.0)
    //    .with_rotation(Quat::from_euler(EulerRot::XYZ, 3.9, 0.0, 3.14));
    camera_bundle.transform = Transform::from_xyz(0.0, 0.0, 10.0);

    commands.spawn_bundle(camera_bundle);

    //first, sphere head
    commands.spawn_bundle(PbrBundle {
        mesh: sphere_handle,
        material: material_handle.clone(),
        transform: Transform::from_xyz(0.0, MAJOR_HEIGHT/2.0 + 1.0, 0.0),
        ..PbrBundle::default()
    })
    .insert(Player)
    .insert(Sticky::Player)
    .insert(Head)
    .with_children(|parent| {

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

pub fn rotate_around(transform: &mut Transform, point: Vec3, rotation: Quat) {
    transform.translation = point + rotation * (transform.translation - point);
    transform.rotation *= rotation;
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, (With<Player>, With<Head>)>
) {
    let mut transform = query.get_single_mut().unwrap();
    if keyboard_input.pressed(KeyCode::W) {
        transform.translation.z += 0.5;
    }
    if keyboard_input.pressed(KeyCode::A) {
        transform.translation.x += 0.5;
    }
    if keyboard_input.pressed(KeyCode::D) {
        transform.translation.x -= 0.5;
    }
    if keyboard_input.pressed(KeyCode::S) {
        transform.translation.z -= 0.5;
    }
    // if keyboard_input.pressed(KeyCode::G) {
    //     anim::set_idle(transforms, &mut player);
    // }
    // if keyboard_input.pressed(KeyCode::T) {
    //     anim::start_anim(PlayerState::Walking, &mut player);
    // }
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