use crate::rotate_around;

use bevy::prelude::*;

//animation of player

const NUM_LIMBS: usize = 4;

const LEFT_ARM_INDEX: usize = 0;
const RIGHT_ARM_INDEX: usize = 1;
const LEFT_LEG_INDEX: usize = 2;
const RIGHT_LEG_INDEX: usize = 3;

pub const MAJOR_HEIGHT: f32 = 3.0;
pub const MINOR_HEIGHT: f32 = 2.0;
pub const ARM_POS: f32 = 1.1;
pub const STICK_SIZE: f32 = 0.2;

pub const LEFT_ARM: Limb = Limb::Arm(Pos::Left);
pub const RIGHT_ARM: Limb = Limb::Arm(Pos::Right);
pub const LEFT_LEG: Limb = Limb::Leg(Pos::Left);
pub const RIGHT_LEG: Limb = Limb::Leg(Pos::Right);

//order is left arm, right arm, left leg, right leg
const IDLE_ANIM_FRAMES: usize = 1;
const IDLE_ANIM: [f32; NUM_LIMBS * 3 * IDLE_ANIM_FRAMES] = [
	-1.0, -0.5, -0.3,
	1.0, -0.4, 0.3,
	-0.5, -3.4, -0.2,
	0.7, -3.3, 0.2,
];

const WALKING_ANIM_FRAMES: usize = 2;
const WALKING_ANIM: [f32; NUM_LIMBS * 3 * WALKING_ANIM_FRAMES] = [

	-1.4, 0.0, 1.0,
	0.5, -0.4, -1.0,
	-0.6, -3.3, 0.6,
	0.6, -3.4, -0.2,

	-1.7, 0.3, -0.5,
	1.2, -0.3, 0.5,
	-1.0, -3.1, -0.2,
	1.1, -3.2, 0.3,

];

const JUMPING_ANIM_FRAMES: usize = 1;

const RUNNING_ANIM_FRAMES: usize = 4;

const CHEERING_ANIM_FRAMES: usize = 2;

#[derive(Copy, Clone, Debug)]
pub enum PlayerState {
	Idle,
	Walking,
}

impl PlayerState {
	pub fn get_anim_slice(&self) -> &[f32] {
		match self {
			PlayerState::Idle => IDLE_ANIM.as_slice(),
			PlayerState::Walking => WALKING_ANIM.as_slice(),
		}
	}

	pub fn get_anim_num_frames(&self) -> usize {
		match self {
			PlayerState::Idle => IDLE_ANIM_FRAMES,
			PlayerState::Walking => WALKING_ANIM_FRAMES,
		}
	}
}

impl Default for PlayerState {
	fn default() -> Self {
		Self::Idle
	}
}

#[derive(Debug, Default, Component)]
pub struct AnimPos {
	start_pos: Vec3,
	end_pos: Vec3,
}

impl AnimPos {
	pub fn calc_curr_pos(&self, amount_through: f32) -> Vec3 {
		Vec3::lerp(self.start_pos, self.end_pos, amount_through)
	}

	pub fn change_pos(&mut self, anim: PlayerState, limb: Limb, index: usize) {
		self.start_pos = self.end_pos;
		self.end_pos = get_target_pos(anim, limb, index);
	}
}

pub fn get_trans_from_pos(limb: Limb, mut pos: Vec3) -> Transform {
	let (default_pos, pivot) = match limb {
		Limb::Arm(_) => (get_default_arm_pos(), get_arm_pivot()),
		Limb::Leg(_) => (get_default_leg_pos(), get_leg_pivot()),
	};

	let mut trans = Transform::from_translation(default_pos);
	//let quat = Quat::from_rotation_arc(default_pos.normalize(), pos.normalize());
	
	//let cross = Vec3::cross(default_pos, pos);
	//let quat = Quat::from_xyzw(cross[0], cross[1], cross[2], f32::sqrt(default_pos.length() * default_pos.length() * pos.length() * pos.length()) + Vec3::dot(default_pos, pos));

	pos = pos - default_pos;
	let quat = Quat::from_rotation_arc(Vec3::Y, pos.normalize());

	rotate_around(&mut trans, pivot, quat);

	trans
}

fn get_target_pos(anim: PlayerState, limb: Limb, index: usize) -> Vec3 {
	let anim_slice = anim.get_anim_slice();

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

	//first split into chunks then skip to correct index then to correct limb index
	//scuffed mega line
	let mut anim_iter = anim_slice.chunks(NUM_LIMBS * 3).skip(index).next().unwrap().iter().skip(limb_index * 3);

	Vec3::new(*anim_iter.next().unwrap(), *anim_iter.next().unwrap(), *anim_iter.next().unwrap())
}

fn get_default_arm_pos() -> Vec3 {
	Vec3::new(0.0, ARM_POS + MINOR_HEIGHT/2.0, 0.0)
}

fn get_arm_pivot() -> Vec3 {
	Vec3::new(0.0, ARM_POS, 0.0)
}

fn get_default_leg_pos() -> Vec3 {
	Vec3::new(0.0, - MAJOR_HEIGHT/2.0 + MINOR_HEIGHT/2.0, 0.0)
}

fn get_leg_pivot() -> Vec3 {
	Vec3::new(0.0, - MAJOR_HEIGHT/2.0, 0.0)
}

#[derive(Copy, Clone, Debug, Component)]
pub enum Limb {
	Arm(Pos),
	Leg(Pos),
}

#[derive(Copy, Clone, Debug)]
pub enum Pos {
	Left,
	Right,
}