use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Grid {

  pub size: usize,
  pub values: Vec<Vec<bool>>,

}

impl Grid {

  pub fn new(values: Vec<Vec<bool>>) -> Self {
    Grid {
      size: values.len(),
      values,
    }
  }

}

pub fn parse_input(input: &str) -> HashMap<Grid, Grid> {

  let mut map: HashMap<Grid, Grid> = HashMap::new();

  input.lines().for_each(|a| {
    let mut grids: Vec<Grid> = a.split(" => ").map(|b| {
      Grid::new(b.split("/")
      .map(|c| 
      { c.chars()
        .fold(Vec::new(), |mut vec, d| { vec.push(map_char(d)); vec })
      }).collect())
      }).collect();

      map.insert(grids.remove(0), grids.remove(0));
  });

  map
}

fn map_char(c: char) -> bool 
{
  match c {
    '.' => false,
    '#' => true,
    _ => unreachable!(),
  }
}