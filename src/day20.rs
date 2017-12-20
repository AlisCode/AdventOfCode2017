extern crate day20_parser;

use self::day20_parser::{do_parse_line, Vector3};

use std::cmp::Ordering;
use itertools::Itertools;

#[derive(Debug, Eq, PartialEq)]
pub struct Particle {
	pos: Vector3,
	vel: Vector3,
	accel: Vector3,
}

impl Particle {
	pub fn new() -> Self {
		Particle {
			pos: Vector3::new(),
			vel: Vector3::new(),
			accel: Vector3::new(),
		}
	}

	pub fn new_from_val(p: Vector3, v: Vector3, a: Vector3) -> Self {
		Particle {
			pos: p,
			vel: v,
			accel: a,
		}
	}

	pub fn pos_at(&self, time: i32) -> Vector3 {
		let x = self.pos.x + self.vel.x * time + time * (time + 1) / 2 * self.accel.x;
		let y = self.pos.y + self.vel.y * time + time * (time + 1) / 2 * self.accel.y;
		let z = self.pos.z + self.vel.z * time + time * (time + 1) / 2 * self.accel.z;

		Vector3::new_from_val(x, y, z)
	}

	pub fn collide_time(&self, other: &Particle) -> Option<i32> {
		let delta_acc = self.accel.x - other.accel.x;
		let delta_vel = self.vel.x - other.vel.x;
		let delta_pos = self.pos.x - other.pos.x;

		let collide_times: Vec<f64> =
			solve_quadratic_root(delta_acc, 2 * delta_vel + delta_acc, 2 * delta_pos);

		let mut collide_times_int: Vec<i32> = collide_times
			.iter()
			.filter_map(|a| if a.round() == *a && *a >= 0f64 {
				Some(*a as i32)
			} else {
				None
			})
			.collect();
		collide_times_int.sort();

		for time in collide_times_int {
			if self.pos_at(time) == other.pos_at(time) {
				return Some(time);
			}
		}
		None
	}
}

pub fn solve_linear_root(b: i32, c: i32) -> Vec<f64> {
	if b == 0 {
		return Vec::new();
	}
	let root: f64 = (-c / b) as f64;
	vec![root]
}

pub fn solve_quadratic_root(a: i32, b: i32, c: i32) -> Vec<f64> {
	if a == 0 {
		return solve_linear_root(b, c);
	}

	let delta: f64 = (b * b - 4 * a * c) as f64;

	if delta > 0f64 {
		let root_one = (-b as f64 - (delta as f64).sqrt()) / (2 * a) as f64;
		let root_two = (-b as f64 + (delta as f64).sqrt()) / (2 * a) as f64;
		return vec![root_one, root_two];
	} else if delta == 0f64 {
		let root = (-b / 2 * a) as f64;
		return vec![root];
	}

	Vec::new()
}

pub fn parse_input(input: &str) -> Vec<Particle> {
	input
		.lines()
		.map(|a| {
			let (p, v, a) = do_parse_line(a);
			Particle::new_from_val(p, v, a)
		})
		.collect()
}

impl PartialOrd for Particle {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Particle {
	fn cmp(&self, other: &Self) -> Ordering {
		let acc_manhattan = self.accel.manhattan_distance();
		let other_acc_manhattan = other.accel.manhattan_distance();
		if acc_manhattan > other_acc_manhattan {
			return Ordering::Greater;
		} else if acc_manhattan < other_acc_manhattan {
			return Ordering::Less;
		}

		let vel_manhattan = self.accel.manhattan_distance();
		let other_vel_manhattan = other.accel.manhattan_distance();
		if vel_manhattan > other_vel_manhattan {
			return Ordering::Greater;
		} else if vel_manhattan < other_vel_manhattan {
			return Ordering::Less;
		}

		let pos_manhattan = self.accel.manhattan_distance();
		let other_pos_manhattan = other.accel.manhattan_distance();
		if pos_manhattan > other_pos_manhattan {
			return Ordering::Greater;
		} else if pos_manhattan < other_pos_manhattan {
			return Ordering::Less;
		}

		Ordering::Equal
	}
}

pub fn resolve_part_one(list: &Vec<Particle>) -> usize {
	let mut sorted_list: Vec<(usize, &Particle)> = list.iter().enumerate().collect();
	sorted_list.sort_by_key(|&(_, key)| key);
	sorted_list[0].0
}

pub fn resolve_part_two(list: &Vec<Particle>) -> u32 {
	// Stores collisions as (index_a, index_b, time of collision)
	let mut collisions: Vec<(usize, usize, i32)> = Vec::new();

	for i in 0..list.len() {
		for j in i + 1..list.len() {
			match list[i].collide_time(&list[j]) {
				Some(time) => {
					collisions.push((i, j, time));
				}
				_ => {}
			}
		}
	}

	// Computes the surviving particles count
	let mut surviving_particles: Vec<usize> = (0..list.len()).collect();
	collisions.sort_by_key(|&(_, _, time)| time);

	// Groups by time, because we need to remove all the particles that collided at time x
	let groups = collisions.iter().group_by(|&&(_, _, time)| time);
	for (_, collision) in groups.into_iter() {
		let grouped_collision_list: Vec<&(usize, usize, i32)> = collision.into_iter().collect();
		grouped_collision_list.iter().for_each(|&&(a, b, _)| {
			// Removes the objects from the list
			if let Some(index_i) = surviving_particles.iter().position(|&item| a == item) {
				surviving_particles.remove(index_i);
			}
			if let Some(index_j) = surviving_particles.iter().position(|&item| b == item) {
				surviving_particles.remove(index_j);
			}
		});
	}

	surviving_particles.len() as u32
}
