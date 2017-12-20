extern crate day20_parser;

use self::day20_parser::{Vector3, do_parse_line};

use std::cmp::Ordering;

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

}

pub fn parse_input(input: &str) -> Vec<Particle> {

  input
  .lines()
  .map(|a| {
    let (p,v,a) = do_parse_line(a);
    Particle::new_from_val(p,v,a)
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
    if acc_manhattan > other_acc_manhattan { return Ordering::Greater }
    else if acc_manhattan < other_acc_manhattan { return Ordering::Less }
    
    let vel_manhattan = self.accel.manhattan_distance();
    let other_vel_manhattan = other.accel.manhattan_distance();
    if vel_manhattan > other_vel_manhattan { return Ordering::Greater }
    else if vel_manhattan < other_vel_manhattan { return Ordering::Less }
    
    let pos_manhattan = self.accel.manhattan_distance();
    let other_pos_manhattan = other.accel.manhattan_distance();
    if pos_manhattan > other_pos_manhattan { return Ordering::Greater }
    else if pos_manhattan < other_pos_manhattan { return Ordering::Less }

    Ordering::Equal

  }
}

pub fn resolve_part_one(list: &Vec<Particle>) -> usize {

  let mut sorted_list: Vec<(usize, &Particle)> = list.iter().enumerate().collect();
  sorted_list.sort_by_key(|&(_, key)| key);

  sorted_list[0].0
}