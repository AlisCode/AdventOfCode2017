#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GridCase {
    pub pos: Position,
    pub state: GridCaseState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GridCaseState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

pub struct VirusCarrier {
    pub pos: Position,
    pub dir: Direction,
}

pub struct Grid {
    pub cases: Vec<GridCase>,
    pub width: usize,
    pub height: usize,
}

impl Position {
    pub fn new() -> Self {
        Position { x: 0, y: 0 }
    }

    pub fn new_from(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

impl GridCase {
    pub fn new_from(x: i32, y: i32, infected: bool) -> Self {

        if infected {
            GridCase {
                pos: Position::new_from(x, y),
                state: GridCaseState::Infected,
            }
        } else {
            GridCase {
                pos: Position::new_from(x, y),
                state: GridCaseState::Clean,
            }
        }

    }

    pub fn get_infected_one(&mut self) -> bool {
        match self.state {
            GridCaseState::Infected => self.state = GridCaseState::Clean,
            GridCaseState::Clean => self.state = GridCaseState::Infected,
            _ => unreachable!(),
        }

        self.state == GridCaseState::Infected
    }

    pub fn get_infected_two(&mut self) -> bool {
        match self.state {
            GridCaseState::Clean => self.state = GridCaseState::Weakened,
            GridCaseState::Weakened => self.state = GridCaseState::Infected,
            GridCaseState::Infected => self.state = GridCaseState::Flagged,
            GridCaseState::Flagged => self.state = GridCaseState::Clean,
        }

        self.state == GridCaseState::Infected
    }
}

impl VirusCarrier {
    pub fn new(x: i32, y: i32) -> Self {
        VirusCarrier {
            pos: Position::new_from(x, y),
            dir: Direction::Up,
        }
    }

    pub fn turn(&mut self, case: &GridCase) {
        match case.state {
            GridCaseState::Infected => self.turn_right(),
            GridCaseState::Clean => self.turn_left(),
            _ => unreachable!(),
        }
    }

    pub fn turn_two(&mut self, case: &GridCase) {
        match case.state {
            GridCaseState::Infected => self.turn_right(),
            GridCaseState::Clean => self.turn_left(),
            GridCaseState::Weakened => (),
            GridCaseState::Flagged => {
                self.turn_left();
                self.turn_left();
            }
        }
    }

    pub fn turn_left(&mut self) {
        match self.dir {
            Direction::Up => self.dir = Direction::Left,
            Direction::Down => self.dir = Direction::Right,
            Direction::Left => self.dir = Direction::Down,
            Direction::Right => self.dir = Direction::Up,
        }
    }

    pub fn turn_right(&mut self) {
        match self.dir {
            Direction::Up => self.dir = Direction::Right,
            Direction::Down => self.dir = Direction::Left,
            Direction::Left => self.dir = Direction::Up,
            Direction::Right => self.dir = Direction::Down,
        }
    }

    pub fn go_forward(&mut self) {
        match self.dir {
            Direction::Up => self.pos.y -= 1,
            Direction::Down => self.pos.y += 1,
            Direction::Left => self.pos.x -= 1,
            Direction::Right => self.pos.x += 1, 
        }
    }

    pub fn infects(&self, grid: &mut Grid, is_one: bool) -> bool {

        // Creates the GridCase if it doesnt already exists
        grid.take_at_pos(&self.pos);

        let index_case: usize = grid.cases
            .iter()
            .enumerate()
            .filter_map(|(index, elem)| if elem.pos.x == self.pos.x &&
                elem.pos.y == self.pos.y
            {
                Some(index)
            } else {
                None
            })
            .next()
            .unwrap();

        if is_one {
            grid.cases[index_case].get_infected_one()
        } else {
            grid.cases[index_case].get_infected_two()
        }


    }
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            cases: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn new_from_input(input: &str) -> Self {
        let mut this = Grid::new();

        let values: Vec<Vec<bool>> = input
            .lines()
            .map(|a| {
                let mut cases_status: Vec<bool> = Vec::new();
                a.chars().for_each(|b| match b {
                    '.' => cases_status.push(false),
                    '#' => cases_status.push(true),
                    _ => unreachable!(),
                });
                cases_status
            })
            .collect();

        this.height = values.len();
        this.width = values[0].len();

        let mut y: i32 = 0;
        for line in values {
            let mut x: i32 = 0;
            for val in line {
                this.cases.push(GridCase::new_from(x, y, val));
                x += 1;
            }
            y += 1;
        }

        this
    }

    pub fn get_middle_pos(&self) -> Position {

        Position::new_from((self.width / 2) as i32, (self.height / 2) as i32)
    }

    fn create_new_case(&mut self, pos: &Position) {
        self.cases.push(GridCase::new_from(pos.x, pos.y, false));
    }

    pub fn take_at_pos(&mut self, pos: &Position) -> &GridCase {

        let exists: bool = self.cases.iter().filter(|a| &a.pos == pos).count() != 0;
        if !exists {
            self.create_new_case(pos);
        }

        self.cases.iter().filter(|a| &a.pos == pos).next().unwrap()
    }
}

pub fn do_burst_one(virus_carrier: &mut VirusCarrier, grid: &mut Grid) -> bool {

    let pos = virus_carrier.pos.clone();
    virus_carrier.turn(grid.take_at_pos(&pos));
    let has_infected = virus_carrier.infects(grid, true);
    virus_carrier.go_forward();

    has_infected
}

pub fn do_burst_two(virus_carrier: &mut VirusCarrier, grid: &mut Grid) -> bool {
    let pos = virus_carrier.pos.clone();
    virus_carrier.turn_two(grid.take_at_pos(&pos));
    let has_infected = virus_carrier.infects(grid, false);
    virus_carrier.go_forward();

    has_infected
}

pub fn resolve_part_one(input: &str, operations: i32) -> usize {

    let mut grid: Grid = Grid::new_from_input(input);
    let start_pos: Position = grid.get_middle_pos();
    let mut virus: VirusCarrier = VirusCarrier::new(start_pos.x, start_pos.y);

    (0..operations)
        .filter_map(|_| if do_burst_one(&mut virus, &mut grid) {
            Some(true)
        } else {
            None
        })
        .count()

}

pub fn resolve_part_two(input: &str, operations: i32) -> usize {

    let mut grid: Grid = Grid::new_from_input(input);
    let start_pos: Position = grid.get_middle_pos();
    let mut virus: VirusCarrier = VirusCarrier::new(start_pos.x, start_pos.y);

    (0..operations)
        .filter_map(|_| if do_burst_two(&mut virus, &mut grid) {
            Some(true)
        } else {
            None
        })
        .count()

}