use crate::rotate_around;
use crate::Player;
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

pub fn set_idle(mut transforms: Query<&mut Transform>, player: &mut Player) {

	let mut left_arm_trans = transforms.get_mut(player.left_arm).unwrap();
	let limb = Limb::Arm(Pos::Left);
	*left_arm_trans = get_trans_from_pos(limb, get_target_pos(PlayerState::Idle, limb, 0));
	player.left_arm_anim = AnimInfo::finished(PlayerState::Idle, limb);

	let mut right_arm_trans = transforms.get_mut(player.right_arm).unwrap();
	let limb = Limb::Arm(Pos::Right);
	*right_arm_trans = get_trans_from_pos(limb, get_target_pos(PlayerState::Idle, limb, 0));
	player.right_arm_anim = AnimInfo::finished(PlayerState::Idle, limb);

	let mut left_leg_trans = transforms.get_mut(player.left_leg).unwrap();
	let limb = Limb::Leg(Pos::Left);
	*left_leg_trans = get_trans_from_pos(limb, get_target_pos(PlayerState::Idle, limb, 0));
	player.left_leg_anim = AnimInfo::finished(PlayerState::Idle, limb);

	let mut right_leg_trans = transforms.get_mut(player.right_leg).unwrap();
	let limb = Limb::Leg(Pos::Right);
	*right_leg_trans = get_trans_from_pos(limb, get_target_pos(PlayerState::Idle, limb, 0));
	player.right_leg_anim = AnimInfo::finished(PlayerState::Idle, limb);

}

pub fn start_anim(new_state: PlayerState, player: &mut Player) {
	player.left_arm_anim = AnimInfo::start_anim(new_state, LEFT_ARM, player.left_arm_anim.curr_pos);
	player.right_arm_anim = AnimInfo::start_anim(new_state, RIGHT_ARM, player.right_arm_anim.curr_pos);
	player.left_leg_anim = AnimInfo::start_anim(new_state, LEFT_LEG, player.left_leg_anim.curr_pos);
	player.right_leg_anim = AnimInfo::start_anim(new_state, RIGHT_LEG, player.right_leg_anim.curr_pos);
}

#[derive(Debug, Default)]
pub struct AnimInfo {
	start_pos: Vec3,
	curr_pos: Vec3,
	end_pos: Vec3,
	time_takes: f32,
	amount_through: f32,
	index: usize,
	anim: PlayerState,
}

impl AnimInfo {
	//called on update, 
	pub fn add_time(&mut self, delta_time: f32, obj_trans: &mut Transform, limb: Limb) {
		self.amount_through = self.amount_through + delta_time / self.time_takes;

		if self.amount_through > 1.0 {
			//go to next anim
			self.amount_through = self.amount_through - 1.0;
			self.start_pos = self.end_pos;
			self.index = self.index + 1;
			if self.index == self.anim.get_anim_num_frames() {
				self.index = 0;
			}
			self.end_pos = get_target_pos(self.anim, limb, self.index);
		}

		self.curr_pos = Vec3::lerp(self.start_pos, self.end_pos, self.amount_through);
		*obj_trans = get_trans_from_pos(limb, self.curr_pos);
	}

    //returns a finished info anim, maybe temp function
	pub fn finished(anim: PlayerState, limb: Limb) -> AnimInfo {
		let curr_pos = get_target_pos(anim, limb, 0);
		AnimInfo {
			start_pos: curr_pos,
			end_pos: curr_pos,
			curr_pos,
			time_takes: 1.0,
			amount_through: 0.0,
			index: 0,
			anim,
		}
	}

	pub fn start_anim(new_state: PlayerState, limb: Limb, curr_pos: Vec3) -> AnimInfo {
		AnimInfo {
			start_pos: curr_pos,
			curr_pos: curr_pos,
			end_pos: get_target_pos(new_state, limb, 0),
			time_takes: 0.5, 
			amount_through: 0.0,
			index: 0,
			anim: new_state,
		}
	}
}

fn get_trans_from_pos(limb: Limb, mut pos: Vec3) -> Transform {
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