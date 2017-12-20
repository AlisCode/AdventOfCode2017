#[macro_use]
extern crate nom;

use nom::digit;
use std::str;


#[derive(Debug, Eq, PartialEq)]
pub struct Vector3 {

  pub x: i32,
  pub y: i32,
  pub z: i32,

}

impl Vector3 {
  
  pub fn new_from_val(x: i32, y: i32, z: i32) -> Self {
    Vector3 {
      x,
      y,
      z,
    }
  }

  pub fn new() -> Self {
    Vector3 {
      x: 0,
      y: 0,
      z: 0,
    }
  }

  pub fn manhattan_distance(&self) -> u32 {
    (self.x.abs() + self.y.abs() + self.z.abs()) as u32
  }

  pub fn add(&mut self, other: Vector3) {
    self.x += other.x;
    self.y += other.y;
    self.z += other.z;
  }

  pub fn mul(&mut self, factor: i32) {
    self.x *= factor;
    self.y *= factor;
    self.z *= factor;
  }
}

named!(number<i32>,
  map_res!(
    map_res!(recognize!(pair!(opt!(tag!("-")), digit)), str::from_utf8),
    str::parse
  )
);

named!(vector3<Vector3>, 
  do_parse!(
    tag!("<") >>
    x: number >>
    tag!(",") >>
    y: number >>
    tag!(",") >>
    z: number >>
    tag!(">") >>
    (Vector3::new_from_val(x, y, z))
));

named!(parse_line<(Vector3, Vector3, Vector3)>, 
  do_parse!(
    tag!("p=") >>
    p: vector3 >>
    tag!(", v=") >>
    v: vector3 >>
    tag!(", a=") >>
    a: vector3 >>
    (p, v, a)
));

pub fn do_parse_line(input: &str) -> (Vector3,Vector3,Vector3)
{
  parse_line(input.as_bytes()).to_result().unwrap()
}