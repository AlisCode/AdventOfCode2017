extern crate day20_parser;

use self::day20_parser::{Vector3, do_parse_line};

#[derive(Debug)]
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