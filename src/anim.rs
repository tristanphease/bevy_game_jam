use crate::rotate_around;
use crate::Player;
use bevy::prelude::*;

//animation of player

const NUM_ARMS: usize = 4;

//order is left arm, right arm, left leg, right leg
const IDLE_ANIM: [f32; NUM_ARMS * 3] = [
	-1.4, 0.0, 1.0,
	0.5, -0.6, 0.7,
	-0.5, -3.3, -0.5,
	0.6, -3.3, 0.5,
];
//const WALKING_ANIM: [] = [];

pub enum PlayerState {
	Idle,
	Walking,
}

//
//amount is between 0 and 1
pub fn switch_anim(transform: &mut Transform, amount: f32) {

}

pub fn set_idle(mut transforms: Query<&mut Transform>, player: &mut Player) {


	let mut left_arm_trans = transforms.get_mut(player.left_arm).unwrap();
	let arm_pos = Vec3::new(0.0, 1.1, 0.0);
	let idle_left_pos = Vec3::new(IDLE_ANIM[0], IDLE_ANIM[1], IDLE_ANIM[2]);

	let angle = Quat::from_rotation_arc(Vec3::Y, idle_left_pos);

	*left_arm_trans = Transform::from_xyz(0.0, 1.1 + 2.0/2.0, 0.0);
	rotate_around(&mut left_arm_trans, arm_pos, angle);


	// left_arm_trans.look_at(Vec3::new(IDLE_ANIM[0], IDLE_ANIM[1], IDLE_ANIM[2]), Vec3::X);

	// let mut right_arm_trans = transforms.get_mut(player.right_arm.unwrap()).unwrap();
	// right_arm_trans.look_at(Vec3::new(IDLE_ANIM[3], IDLE_ANIM[4], IDLE_ANIM[5]), Vec3::X);

	// let mut left_leg_trans = transforms.get_mut(player.left_leg.unwrap()).unwrap();
	// left_leg_trans.look_at(Vec3::new(IDLE_ANIM[6], IDLE_ANIM[7], IDLE_ANIM[8]), Vec3::X);

	// let mut right_leg_trans = transforms.get_mut(player.right_leg.unwrap()).unwrap();
	// right_leg_trans.look_at(Vec3::new(IDLE_ANIM[9], IDLE_ANIM[10], IDLE_ANIM[11]), Vec3::X);
}

pub fn start_anim(new_state: PlayerState, player: &mut Player) {
	
}

#[derive(Default)]
pub struct AnimInfo {
	start_angle: Quat,
	end_angle: Quat,
	time_takes: f32,
	amount_through: f32,
}

impl AnimInfo {
	pub fn add_time(&mut self, delta_time: f32, obj_trans: &mut Transform) {
		self.amount_through = f32::clamp(self.amount_through + delta_time / self.time_takes, 0.0, 1.0);

		obj_trans.rotation = Quat::lerp(self.start_angle, self.end_angle, self.amount_through);
	}
}