use crate::{rotate_around, Sticky, Physics, Head};

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
	-0.9, -3.1, 0.1,
	0.7, -3.2, -0.4,
	-0.5, -5.9, -0.1,
	0.5, -5.9, 0.0,
];

const WALKING_ANIM_FRAMES: usize = 2;
const WALKING_ANIM: [f32; NUM_LIMBS * 3 * WALKING_ANIM_FRAMES] = [
	/*0.7, -3.2, -0.4,
	-0.9, -3.0, 0.5,
	0.5, -5.8, 0.4,
	-0.5, -5.9, -0.5,

	-1.0, -2.9, -0.6,
	0.9, -3.1, 0.5,
	0.5, -5.8, -0.4,
	-0.5, -5.9, 0.3,*/
	-0.6, -3.1, -0.6,
	1.2, -2.9, 0.5,
	-0.7, -5.8, 0.4,
	0.9, -5.7, -0.4,

	-0.7, -2.9, 0.9,
	1.1, -2.9, -0.7,
	-0.7, -5.8, -0.5,
	0.9, -5.6, 0.6,
];

const JUMPING_ANIM_FRAMES: usize = 1;
const JUMPING_ANIM: [f32; NUM_LIMBS * 3 * JUMPING_ANIM_FRAMES] = [
	-1.8, -2.1, 0.4,
	1.9, -1.3, 0.4,
	-0.6, -5.5, -1.1,
	1.0, -5.4, 1.0,
];

const RUNNING_ANIM_FRAMES: usize = 4;
const RUNNING_ANIM: [f32; NUM_LIMBS * 3 * RUNNING_ANIM_FRAMES] = [
	-1.1, -2.9, -0.6,
	1.1, -2.9, 0.4,
	-0.9, -5.7, -0.4,
	1.0, -5.7, 0.4,

	-0.9, -2.9, 0.9,
	0.9, -2.9, -0.8,
	-0.8, -5.7, 0.5,
	0.6, -5.7, -0.9,

	-1.6, -2.4, -0.7,
	1.4, -2.3, 0.9,
	-0.5, -5.5, -1.1,
	0.6, -5.5, 1.1,

	-1.2, -2.5, 0.9,
	1.2, -2.8, -0.7,
	-0.8, -5.4, 1.1,
	0.7, -5.3, -1.3,
];

const SPIN_ANIM_FRAMES: usize = 1;
const SPIN_ANIM: [f32; NUM_LIMBS * 3 * SPIN_ANIM_FRAMES] = [
	-1.8, -1.6, -0.6,
	1.9, -1.2, 0.5,
	0.0, -5.9, 0.3,
	1.8, -4.5, -0.4,
];

const IDLE_ANIM_TIME: f32 = 1.0;
const WALKING_ANIM_TIME: f32 = 0.3;
const RUNNING_ANIM_TIME: f32 = 1.0;
const JUMPING_ANIM_TIME: f32 = 0.8;
const SPIN_ANIM_TIME: f32 = 1.0;

//const CHEERING_ANIM_FRAMES: usize = 2;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PlayerState {
	Idle,
	Walking,
	Running,
	Jumping,
	Spin,
}

impl PlayerState {
	pub fn get_anim_slice(&self) -> &[f32] {
		match self {
			PlayerState::Idle => IDLE_ANIM.as_slice(),
			PlayerState::Walking => WALKING_ANIM.as_slice(),
			PlayerState::Running => RUNNING_ANIM.as_slice(),
			PlayerState::Jumping => JUMPING_ANIM.as_slice(),
			PlayerState::Spin => SPIN_ANIM.as_slice(),
		}
	}

	pub fn get_anim_num_frames(&self) -> usize {
		match self {
			PlayerState::Idle => IDLE_ANIM_FRAMES,
			PlayerState::Walking => WALKING_ANIM_FRAMES,
			PlayerState::Running => RUNNING_ANIM_FRAMES,
			PlayerState::Jumping => JUMPING_ANIM_FRAMES,
			PlayerState::Spin => SPIN_ANIM_FRAMES,
		}
	}

	pub fn get_anim_time(&self) -> f32 {
		match self {
			PlayerState::Idle => IDLE_ANIM_TIME,
			PlayerState::Walking => WALKING_ANIM_TIME,
			PlayerState::Running => RUNNING_ANIM_TIME,
			PlayerState::Jumping => JUMPING_ANIM_TIME,
			PlayerState::Spin => SPIN_ANIM_TIME,
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

	pub fn change_pos(&mut self, anim: PlayerState, limb: Limb, amount_through: f32, index: usize) {
		self.start_pos = self.calc_curr_pos(amount_through);
		self.end_pos = get_target_pos(anim, limb, index);
	}

	pub fn default_from_limb(limb: Limb, anim: PlayerState) -> AnimPos {
		let pos = get_target_pos(anim, limb, 0);
		AnimPos {
			start_pos: pos,
			end_pos: pos,
		}
	}
}

pub fn get_trans_from_pos(limb: Limb, mut pos: Vec3) -> Transform {
	let (default_pos, pivot) = match limb {
		Limb::Arm(_) => (get_default_arm_pos(), get_arm_pivot()),
		Limb::Leg(_) => (get_default_leg_pos(), get_leg_pivot()),
	};

	let mut trans = Transform::from_translation(default_pos);
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

pub fn anim_choose_system(
	mut player_head_query: Query<(&mut AnimInfo, &Physics, &Sticky)>,
	mut limb_query: Query<(&mut AnimPos, &Limb, &Sticky)>,
) {
	for (mut anim_info, physics, sticky) in player_head_query.iter_mut() {
		let speed = physics.velocity.length_squared();

		let mut changed = false;
		if !physics.grounded {
			if anim_info.anim != PlayerState::Jumping {
				anim_info.change_anim(PlayerState::Jumping);
				changed = true;
			}
		} else {
			if speed > 3.0 {
				if anim_info.anim != PlayerState::Running {
					anim_info.change_anim(PlayerState::Running);
					changed = true;
				}
			} else if speed > 0.5 {
				if anim_info.anim != PlayerState::Walking {
					anim_info.change_anim(PlayerState::Walking);
					changed = true;
				}
			} else {
				if anim_info.anim != PlayerState::Idle {
					anim_info.change_anim(PlayerState::Idle);
					changed = true;
				}
			}
		}

		if changed {
			for (mut anim_pos, limb, limb_sticky) in limb_query.iter_mut() {
				if sticky == limb_sticky {
					anim_pos.change_pos(anim_info.anim, *limb, anim_info.amount_through, 0);
				}
			}
		}
	}
}

pub fn update_anims(
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
                    anim_pos.change_pos(anim_info.anim, *limb, 1.0, anim_info.index);
                }
                *transform = get_trans_from_pos(*limb, anim_pos.calc_curr_pos(anim_info.amount_through));
            }
        }
    }
}

#[derive(Component)]
pub struct AnimInfo {
    pub time_takes: f32,
    pub amount_through: f32,
    pub index: usize,
    pub anim: PlayerState,
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

    pub fn change_anim(&mut self, new_anim: PlayerState) {
    	self.index = new_anim.get_anim_num_frames() - 1;
    	self.amount_through = 0.0;
    	self.time_takes = new_anim.get_anim_time();
    	self.anim = new_anim;
    }
}

pub fn spin_sticky_system(
	mut query: Query<(&mut Transform, &AnimInfo), With<Head>>
) {
	for (mut trans, anim_info) in query.iter_mut() {
		if anim_info.anim == PlayerState::Spin {
			trans.rotation = trans.rotation * Quat::from_axis_angle(Vec3::Y, 0.1);
		}
	}
}