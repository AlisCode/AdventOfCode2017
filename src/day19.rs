#[derive(PartialEq, Debug)]
pub enum Case {
  VerticalMove,
  HorizontalMove,
  Corner,  
  Letter(char),
  Void,
}

#[derive(PartialEq, Debug)]
pub enum Direction {
  Left,
  Up,
  Right,
  Down
}

impl Case {
  pub fn from_char(input: char) -> Self {
    match input {
      '|' => Case::VerticalMove,
      '-' => Case::HorizontalMove,
      '+' => Case::Corner,
      ' ' => Case::Void,
      _ => Case::Letter(input),
    }
  }
}

pub fn parse_input(input: &str) -> Vec<Vec<Case>> {

    input.lines().map(|a| {
        let mut list: Vec<Case> = Vec::new();
        a.chars().for_each(|b| list.push(Case::from_char(b)));
        list
      }).collect()

}

pub fn get_start_pos(map: &Vec<Vec<Case>>) -> (usize, usize) {
  let x: usize = map[0].iter().position(|a| *a == Case::VerticalMove).unwrap();
  (x,0)
}

pub fn crawl_to_end(map: &Vec<Vec<Case>>, x_start: usize, y_start:usize) -> Vec<char> {

    let mut res: Vec<char> = Vec::new();

    let mut x = x_start.clone();
    let mut y = y_start.clone();

    let mut dir: Direction = Direction::Down;
    let mut continue_crawling: bool = true;

    while continue_crawling
    {
      let (opt_char, new_x, new_y) = crawl_to_event(map, x, y, &dir); 

      x = new_x;
      y = new_y;

      match dir {
        Direction::Down => y += 1,
        Direction::Up => y -= 1,
        Direction::Left => x += 1,
        Direction::Right => x -= 1,
      }
      
      match opt_char {
        Some(c) => {
          res.push(c);
        },
        _ => { 
          let opt_dir = get_next_direction(map, x, y, new_x, new_y);
          match opt_dir {
            Some(new_dir) => { println!("new dir: {:?}", new_dir); dir = new_dir; },
            _ => continue_crawling = false,
          }},
      }

      

    }

    res
}

pub fn crawl_to_event(map: &Vec<Vec<Case>>, x_start: usize, y_start: usize,dir: &Direction) -> (Option<char>, usize, usize) {

    let mut x = x_start;
    let mut y = y_start;

    let mut case = &map[y][x];

    let mut continue_crawling = true;

    while continue_crawling
    {
      match dir {
        &Direction::Left => x -= 1,
        &Direction::Up => y -= 1,
        &Direction::Right => x += 1,
        &Direction::Down => y += 1,
      }

      case = &map[y][x]; 
      match *case {
        Case::Corner | Case::Letter(_) => continue_crawling = false,
        _ => (),
      }
    }

    match *case {
      Case::Letter(c) => (Some(c), x, y),
      _ => (None, x, y),
    }
}

pub fn get_next_direction(map: &Vec<Vec<Case>>, old_x: usize, old_y: usize, x: usize, y:usize) -> Option<Direction> {

  if x as i32 - 1 >= 0 && x as i32 - 1 != old_x as i32 && map[y][x-1] != Case::Void {
    return Some(Direction::Left);
  }
  if x as i32 + 1 < map[y].len() as i32 && x as i32 + 1 != old_x as i32 && map[y][x+1] != Case::Void {
    return Some(Direction::Right);
  }
  if y as i32 - 1 >= 0 && y as i32 - 1 != old_y as i32 && map[y-1][x] != Case::Void {
    return Some(Direction::Up);
  }
  if y as i32 + 1 < map.len() as i32 && y as i32 + 1 != old_y as i32 && map[y+1][x] != Case::Void {
    return Some(Direction::Down);
  }

  println!("This is the end");
  None
}