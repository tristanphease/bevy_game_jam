use crate::rotate_around;
use crate::Player;
use bevy::prelude::*;

//animation of player

const NUM_ARMS: usize = 4;

const LEFT_ARM_INDEX: usize = 0;
const RIGHT_ARM_INDEX: usize = 1;
const LEFT_LEG_INDEX: usize = 2;
const RIGHT_LEG_INDEX: usize = 3;

pub const MAJOR_HEIGHT: f32 = 3.0;
pub const MINOR_HEIGHT: f32 = 2.0;
pub const ARM_POS: f32 = 1.1;
pub const STICK_SIZE: f32 = 0.2;

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
	let limb = Limb::Arm(Pos::Left);
	*left_arm_trans = get_trans_from_pos(limb, get_target_pos(PlayerState::Idle, limb));

	let mut right_arm_trans = transforms.get_mut(player.right_arm).unwrap();
	let limb = Limb::Arm(Pos::Right);
	*right_arm_trans = get_trans_from_pos(limb, get_target_pos(PlayerState::Idle, limb));

	let mut left_leg_trans = transforms.get_mut(player.left_leg).unwrap();
	let limb = Limb::Leg(Pos::Left);
	*left_leg_trans = get_trans_from_pos(limb, get_target_pos(PlayerState::Idle, limb));

	let mut right_leg_trans = transforms.get_mut(player.right_leg).unwrap();
	let limb = Limb::Leg(Pos::Right);
	*right_leg_trans = get_trans_from_pos(limb, get_target_pos(PlayerState::Idle, limb));


	/*let mut right_arm_trans = transforms.get_mut(player.right_arm).unwrap();
	let right_arm_info = AnimInfo::finished(PlayerState::Idle, Limb::Arm(Pos::Right));
	right_arm_trans.rotation = right_arm_info.end_angle;
	player.right_arm_anim = right_arm_info;*/

	/*let mut left_leg_trans = transforms.get_mut(player.left_leg).unwrap();
	let left_leg_info = AnimInfo::finished(PlayerState::Idle, Limb::Leg(Pos::Left));
	left_leg_trans.rotation = left_leg_info.end_angle;
	player.left_leg_anim = left_leg_info;

	let mut right_leg_trans = transforms.get_mut(player.right_leg).unwrap();
	let right_leg_info = AnimInfo::finished(PlayerState::Idle, Limb::Leg(Pos::Right));
	right_leg_trans.rotation = right_leg_info.end_angle;
	player.right_leg_anim = right_leg_info;*/
}

pub fn start_anim(new_state: PlayerState, player: &mut Player) {
	
}

#[derive(Debug, Default)]
pub struct AnimInfo {
	start_pos: Vec3,
	end_pos: Vec3,
	time_takes: f32,
	amount_through: f32,
	index: usize,
}

impl AnimInfo {
	pub fn add_time(&mut self, delta_time: f32, obj_trans: &mut Transform, pivot: Vec3) {
		self.amount_through = f32::clamp(self.amount_through + delta_time / self.time_takes, 0.0, 1.0);

		*obj_trans = Transform::from_translation(get_default_arm_pos());
		let pos = Vec3::lerp(self.start_pos, self.end_pos, self.amount_through);
		let quat = Quat::from_rotation_arc(get_default_arm_pos(), pos);
		rotate_around(obj_trans, pivot, quat);
	}

	pub fn finished(anim: PlayerState, limb: Limb) -> AnimInfo {
		AnimInfo {
			start_pos: Vec3::default(),
			end_pos: get_target_pos(anim, limb),
			time_takes: 0.0,
			amount_through: 1.0,
			index: 0,
		}
	}
}

fn get_trans_from_pos(limb: Limb, pos: Vec3) -> Transform {
	let (default_pos, pivot) = match limb {
		Limb::Arm(_) => (get_default_arm_pos(), get_arm_pivot()),
		Limb::Leg(_) => (get_default_leg_pos(), get_leg_pivot()),
	};

	let mut trans = Transform::from_translation(default_pos);
	let quat = Quat::from_rotation_arc(default_pos.normalize(), pos.normalize());
	rotate_around(&mut trans, pivot, quat);

	trans
}

fn get_target_pos(anim: PlayerState, limb: Limb) -> Vec3 {
	let anim_slice = match anim {
		PlayerState::Idle => IDLE_ANIM.as_slice(),
		PlayerState::Walking => todo!(),
	};

	let limb_index = match limb {
		Limb::Arm(arm) => match arm {
			Pos::Left => LEFT_ARM_INDEX,
			Pos::Right => RIGHT_ARM_INDEX,
		},
		Limb::Leg(leg) => match leg {
			Pos::Left => LEFT_LEG_INDEX,
			Pos::Right => RIGHT_LEG_INDEX,
		},
	};

	let mut anim_iter = anim_slice.chunks(3).skip(limb_index).next().unwrap().iter();

	Vec3::new(*anim_iter.next().unwrap(), *anim_iter.next().unwrap(), *anim_iter.next().unwrap())
}

fn get_angle(limb: Limb, target_pos: Vec3) -> Quat {
	match limb {
		Limb::Arm(_) => Quat::from_rotation_arc(get_default_arm_pos(), target_pos),
		Limb::Leg(_) => Quat::from_rotation_arc(get_default_leg_pos(), target_pos),
	}
}

fn get_default_arm_pos() -> Vec3 {
	Vec3::new(0.0, ARM_POS + MINOR_HEIGHT/2.0, 0.0)
}

fn get_arm_pivot() -> Vec3 {
	Vec3::new(0.0, ARM_POS, 0.0)
}

fn get_default_leg_pos() -> Vec3 {
	Vec3::new(0.0, - MAJOR_HEIGHT/2.0 - MINOR_HEIGHT/2.0, 0.0)
}

fn get_leg_pivot() -> Vec3 {
	Vec3::new(0.0, - MAJOR_HEIGHT/2.0, 0.0)
}

#[derive(Copy, Clone, Debug)]
pub enum Limb {
	Arm(Pos),
	Leg(Pos),
}

#[derive(Copy, Clone, Debug)]
pub enum Pos {
	Left,
	Right,
}