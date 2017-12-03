#[derive(Clone, Copy)]
pub enum Direction {
    Right,
    Up,
    Left,
    Down
}

pub struct SpiralMemoryCase {

    x: i32,
    y: i32,
    dir: Direction,
    items_left_on_line: i32,
    current_items_on_line: i32,
    is_second_line: bool,

}

impl SpiralMemoryCase {

    /// Returns a new SpiralMemoryCase sequence generator
    pub fn new_seq_generator() -> Self {
        SpiralMemoryCase {
            x: 0,
            y: 0,
            dir: Direction::Right,
            current_items_on_line: 1,
            items_left_on_line: 1, 
            is_second_line: false,
        }
    }

    pub fn new(pos_x: i32, pos_y: i32, direction: Direction, current_items: i32, items_left: i32, is_second: bool) -> Self {
        SpiralMemoryCase {
            x: pos_x,
            y: pos_y,
            dir: direction,
            current_items_on_line: current_items,
            items_left_on_line: items_left,
            is_second_line: is_second,
        }
    }

    pub fn get_manhattan_distance(&self) -> i32
    {
        self.x.abs() + self.y.abs() 
    }
}


impl Iterator for SpiralMemoryCase {
    type Item = SpiralMemoryCase;

    fn next(&mut self) -> Option<SpiralMemoryCase>
    {
         
        // Calculates new coordinates
        let mut pos_x: i32 = 0;
        let mut pos_y: i32 = 0;

        // Sets the new X and Y coordinates using direction
        match self.dir {
            Direction::Right => pos_x = self.x + 1,
            Direction::Up => pos_y = self.y + 1,
            Direction::Left => pos_x = self.x - 1,
            Direction::Down => pos_y = self.y - 1,
        }

        // Sets the new direction
        let mut direction: Direction = self.dir.clone();

        // Sets the new is_second_line 
        let mut is_second: bool = self.is_second_line.clone();

        // Sets the new items_left_on_line
        let mut items_left: i32 = self.items_left_on_line.clone();

        // Sets the new current_items_on_line :
        let current_items: i32 = self.current_items_on_line - 1;

        // if current line will overflow when taking the next item
        if current_items <= 0 
        { 
            // Rotates the direction by -90°
            direction = match direction
            {
                Direction::Right => Direction::Up,
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Right,
            };

            // Checks if is_second is true :  
            if self.is_second_line { 
                // if so, init a new line with one more item
                is_second = false;
                items_left = items_left + 1;
            }
            else {
                // if not, then we set it to true so the next line will change its items_left counter
                is_second = true;
            }
        }
        
        let new_item = SpiralMemoryCase::new(
            pos_x,
            pos_y,
            direction,
            current_items,
            items_left,
            is_second
        );

        Some(new_item)
    }

}