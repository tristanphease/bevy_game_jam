use crate::{Sticky, Physics, Head, detect_collision};
use crate::enemy::{EnemyMaterials, Health};
use bevy::prelude::*;

const DELETE_SHOT: f32 = -20.0;
const SHOT_SIZE: f32 = 0.3;

pub const STICKY_SIZE: f32 = 0.4;

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
) {

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
	mut sticky_query: Query<(&GlobalTransform, Entity, Option<&mut Health>, Option<&mut Physics>), (With<Head>, With<Sticky>)>,
) {
	for (sticky_trans, sticky_entity, enemy_health, player_physics) in sticky_query.iter_mut() {
		for (shot_entity, shot_trans) in shot_query.iter() {
			let sticky_hitbox_trans = sticky_trans.translation + Vec3::new(0.0, -3.5, 0.0);
			let sticky_hitbox_scale = Vec3::new(STICKY_SIZE, 6.0, STICKY_SIZE);

			if detect_collision((sticky_hitbox_trans, sticky_hitbox_scale), (shot_trans.translation, Vec3::new(SHOT_SIZE, SHOT_SIZE, SHOT_SIZE))) {
				commands.entity(shot_entity).despawn();
				if player_physics.is_some() {
					player_physics.unwrap().velocity.y += 20.0;
				} else {
					let mut health = enemy_health.unwrap();
					health.amount -= 1;
					if health.amount == 0 {
						commands.entity(sticky_entity).despawn_recursive();
					}
				}
				break;
			}
		}
	}
}
