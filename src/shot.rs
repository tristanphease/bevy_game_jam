use crate::{Sticky, Physics, Head, Player, Enemy, AppState, detect_collision};
use crate::enemy::{EnemyMaterials, Health, EnemyColour, EnemyNum};
use bevy::prelude::*;

const DELETE_SHOT: f32 = -20.0;
const SHOT_SIZE: f32 = 0.3;

//permissive to make it a little easier
const STICKY_SIZE: f32 = 0.8;

pub fn create_shot(
	commands: &mut Commands,
	mesh: Handle<Mesh>,
	material: Handle<StandardMaterial>,
	position: Vec3,
	velocity: Vec3,
	sticky: Sticky,
) {

	commands.spawn_bundle(PbrBundle {
		mesh,
		material,
		transform: Transform::from_translation(position),
		..PbrBundle::default()
	})
	.insert(sticky)
	.insert(ShotPhysics {
		velocity,
	});
}

#[derive(Component)]
pub struct ShotPhysics {
	velocity: Vec3,
}

pub struct ShotMesh {
	pub shot_handle: Handle<Mesh>,
}

pub struct PlayerMaterial {
	pub player_mat: Handle<StandardMaterial>,
}

pub fn create_shot_mesh_system(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
) {
	let mesh_handle = meshes.add(Mesh::from(shape::UVSphere {
		radius: SHOT_SIZE,
		..shape::UVSphere::default()
	}));

	commands.insert_resource(ShotMesh {
		shot_handle: mesh_handle,
	});
}

pub fn enemy_shot_system(
	mut commands: Commands,
	enemy_mats: Res<EnemyMaterials>,
	shot_mesh: Res<ShotMesh>,
	query: Query<(&GlobalTransform, &EnemyColour), (With<Enemy>, With<Head>)>,
	player_query: Query<&GlobalTransform, (With<Player>, With<Head>)>
) {
	let player_trans = player_query.get_single().unwrap();

	for (trans, enemy_colour) in query.iter() {
		//very scuffed
		if fastrand::f32() < 0.002 {
			let material = match enemy_colour {
				EnemyColour::Red => enemy_mats.red.clone(),
				EnemyColour::Green => enemy_mats.green.clone(),
				EnemyColour::Blue => enemy_mats.blue.clone(),
			};
			create_shot(
				&mut commands,
				shot_mesh.shot_handle.clone(),
				material,
				trans.translation,
				player_trans.translation - trans.translation,
				Sticky::Enemy,
			);
		}
	}
}

pub fn shot_physics_system(
	mut query: Query<(&mut Transform, &mut ShotPhysics)>,
	time: Res<Time>,
) {
	let delta = time.delta_seconds();
	for (mut trans, mut shot_physics) in query.iter_mut() {
		trans.translation += shot_physics.velocity * delta;
		shot_physics.velocity.y -= 0.1;
	}
}

pub fn remove_shot_system(
	mut commands: Commands,
	query: Query<(Entity, &Transform), With<ShotPhysics>>,
) {
	for (entity, transform) in query.iter() {
		if transform.translation.y <= DELETE_SHOT {
			commands.entity(entity).despawn();
		}
	}
}

pub fn shot_sticky_collision_check_system(
	mut commands: Commands,
	shot_query: Query<(Entity, &Transform), With<ShotPhysics>>,
	mut sticky_query: Query<(&mut Transform, Entity, Option<&mut Health>, Option<&mut Physics>), (With<Head>, With<Sticky>, Without<ShotPhysics>)>,
	mut enemy_num: ResMut<EnemyNum>,
	mut app_state: ResMut<State<AppState>>,
) {
	for (mut sticky_trans, sticky_entity, enemy_health, player_physics) in sticky_query.iter_mut() {
		for (shot_entity, shot_trans) in shot_query.iter() {
			let sticky_hitbox_trans = sticky_trans.translation + Vec3::new(0.0, -3.5, 0.0);
			let sticky_hitbox_scale = Vec3::new(STICKY_SIZE, 6.0, STICKY_SIZE);

			if detect_collision((sticky_hitbox_trans, sticky_hitbox_scale), (shot_trans.translation, Vec3::new(SHOT_SIZE, SHOT_SIZE, SHOT_SIZE))) {
				commands.entity(shot_entity).despawn();
				if player_physics.is_some() {
					let mut physics = player_physics.unwrap();
					physics.velocity.x += (fastrand::f32() - 0.5) * 10.0;
					physics.velocity.y += 30.0;
					physics.velocity.z += (fastrand::f32() - 0.5) * 10.0;
				} else {
					let mut health = enemy_health.unwrap();
					health.amount -= 1;
					if health.amount == 0 {
						commands.entity(sticky_entity).despawn_recursive();
						enemy_num.number -= 1;
						if enemy_num.number == 0 {
							app_state.set(AppState::Win).unwrap();
						}
					} else {
						sticky_trans.translation.x = (fastrand::f32() - 0.5) * 100.0;
						sticky_trans.translation.z = (fastrand::f32() - 0.5) * 100.0;
					}
				}
				break;
			}
		}
	}
}

