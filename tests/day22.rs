extern crate advent_of_code_2017;

use self::advent_of_code_2017::day22::{VirusCarrier, Direction, GridCase, GridCaseState, Grid,
                                       Position, resolve_part_one, resolve_part_two};

// Tests turning left and right
#[test]
pub fn test_day22_turning_sample1() {

    let mut virus: VirusCarrier = VirusCarrier::new(0, 0);

    assert_eq!(virus.dir, Direction::Up);
    virus.turn_left();
    assert_eq!(virus.dir, Direction::Left);
    virus.turn_left();
    assert_eq!(virus.dir, Direction::Down);
    virus.turn_left();
    assert_eq!(virus.dir, Direction::Right);
    virus.turn_left();
    assert_eq!(virus.dir, Direction::Up);
    virus.turn_right();
    assert_eq!(virus.dir, Direction::Right);
    virus.turn_right();
    assert_eq!(virus.dir, Direction::Down);
    virus.turn_right();
    assert_eq!(virus.dir, Direction::Left);
    virus.turn_right();
    assert_eq!(virus.dir, Direction::Up);
}

// Turning while being on an infected case should make the virus carrier turn right
// On a non-infected case, it should make it turn left
#[test]
pub fn test_day22_turning_sample2() {

    let mut virus: VirusCarrier = VirusCarrier::new(0, 0);

    let infected_case: GridCase = GridCase::new_from(0, 0, true);
    let normal_case: GridCase = GridCase::new_from(0, 0, false);

    virus.turn(&infected_case);
    assert_eq!(virus.dir, Direction::Right);

    virus.turn(&normal_case);
    assert_eq!(virus.dir, Direction::Up);

}

// Tests grid creation from input &str
#[test]
fn test_day22_grid_creation() {
    let input: &str = "...\n.##\n..#";
    let grid: Grid = Grid::new_from_input(input);

    assert_eq!(
        grid.cases,
        vec![
            GridCase::new_from(0, 0, false),
            GridCase::new_from(1, 0, false),
            GridCase::new_from(2, 0, false),
            GridCase::new_from(0, 1, false),
            GridCase::new_from(1, 1, true),
            GridCase::new_from(2, 1, true),
            GridCase::new_from(0, 2, false),
            GridCase::new_from(1, 2, false),
            GridCase::new_from(2, 2, true),
        ]
    );

    assert_eq!(grid.width, 3);
    assert_eq!(grid.height, 3);

    let start_pos: Position = grid.get_middle_pos();
    assert_eq!(start_pos.x, 1);
    assert_eq!(start_pos.y, 1);
}

// Calling an infection on an infected node should return false and clean the cell
// Calling an infection on a non-infected node should return true and infect the cell
#[test]
pub fn test_day22_infection_part_one_sample1() {
    let mut infected_case: GridCase = GridCase::new_from(0, 0, true);
    let mut normal_case: GridCase = GridCase::new_from(0, 0, false);

    assert_eq!(infected_case.get_infected_one(), false);
    assert_eq!(normal_case.get_infected_one(), true);

    assert_eq!(infected_case.state, GridCaseState::Clean);
    assert_eq!(normal_case.state, GridCaseState::Infected);
}

// Using a given grid, infection should vary following the
#[test]
pub fn test_day22_infection_part_one_sample2() {

    let input: &str = "...\n.##\n..#";
    let mut grid: Grid = Grid::new_from_input(input);
    let mut virus: VirusCarrier = VirusCarrier::new(1, 1);

    assert_eq!(virus.infects(&mut grid, true), false);
    assert_eq!(grid.cases[4].state, GridCaseState::Clean);

    virus.pos.x = 0;
    virus.pos.y = 0;

    assert_eq!(virus.infects(&mut grid, true), true);
    assert_eq!(grid.cases[0].state, GridCaseState::Infected);

}

#[test]
pub fn test_day22_movement_sample1() {

    let mut virus: VirusCarrier = VirusCarrier::new(1, 1);
    virus.go_forward();

    assert_eq!(virus.pos, Position::new_from(1, 0));

    virus.turn_left();
    virus.go_forward();

    assert_eq!(virus.pos, Position::new_from(0, 0));

    virus.turn_left();
    virus.go_forward();

    assert_eq!(virus.pos, Position::new_from(0, 1));

    virus.turn_left();
    virus.go_forward();

    assert_eq!(virus.pos, Position::new_from(1, 1));
}

#[test]
fn test_day22_movement_part_one_sample2() {

    let input: &str = "...\n.##\n..#";
    let mut grid: Grid = Grid::new_from_input(input);
    let mut virus: VirusCarrier = VirusCarrier::new(1, 1);

    assert_eq!(virus.infects(&mut grid, true), false);
    virus.go_forward();
    assert_eq!(virus.infects(&mut grid, true), true);
    virus.go_forward();
    assert_eq!(virus.infects(&mut grid, true), true);

}

#[test]
fn test_day22_part_one_sample1() {

    let input: &str = "..#\n#..\n...";
    assert_eq!(resolve_part_one(input, 7), 5);
    assert_eq!(resolve_part_one(input, 70), 41);
    assert_eq!(resolve_part_one(input, 10000), 5587);

}

#[test]
fn test_day22_part_two_sample1() {

    let input: &str = "..#\n#..\n...";
    assert_eq!(resolve_part_two(input, 100), 26);
}