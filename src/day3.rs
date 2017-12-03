#[derive(Clone, Copy)]
pub enum Direction {
    Right,
    Up,
    Left,
    Down
}

#[derive(Clone, Copy)]
pub struct SpiralMemoryCase {

    pub x: i32,
    pub y: i32,
    pub value: i64,

}

pub struct SpiralMemoryCaseSequence {

    pub x: i32,
    pub y: i32,
    pub dir: Direction,
    pub items_left_on_line: i32,
    pub current_items_on_line: i32,
    pub is_second_line: bool,
    pub registered_cases: Vec<SpiralMemoryCase>,

}

impl SpiralMemoryCase {

    pub fn new(pos_x: i32, pos_y: i32, val: i64) -> Self {
        SpiralMemoryCase {
            x: pos_x,
            y: pos_y,
            value: val,
        }
    }

    pub fn get_manhattan_distance(&self) -> i32
    {
        self.x.abs() + self.y.abs() 
    }

    pub fn compute_value(&mut self, list: &Vec<SpiralMemoryCase>)
    {
        self.value = 
        list
            .iter()
            .filter(|a| 
                a.x >= self.x-1 
            &&  a.x <= self.x+1 
            &&  a.y >= self.y-1 
            &&  a.y <= self.y+1 
            && !(a.x == self.x && a.y == self.y)
            )
            .map(|a| a.value)
            .sum();  

        if self.value == 0 { self.value = 1; }
        
        /* Enable this to see when the value is above the required input
        if self.value >= 347991
        {
            println!("value above puzzle input {}", self.value);
        }*/
    }
}

impl SpiralMemoryCaseSequence {

    /// Returns a new SpiralMemoryCase sequence generator
    pub fn new() -> Self {
        SpiralMemoryCaseSequence {
            x: -1,
            y: 0,
            dir: Direction::Right,
            current_items_on_line: 2,
            items_left_on_line: 1, 
            is_second_line: false,
            registered_cases: Vec::new(),
        }
    }
}


impl Iterator for SpiralMemoryCaseSequence {
    type Item = SpiralMemoryCase;

    fn next(&mut self) -> Option<SpiralMemoryCase>
    {
        // Calculates new coordinates
        let pos_x: i32 = match self.dir {
            Direction::Right => self.x + 1,
            Direction::Left => self.x - 1,
            _ => self.x,
        };

        let pos_y: i32 = match self.dir {
            Direction::Up => self.y + 1,
            Direction::Down => self.y - 1,
            _ => self.y,
        };


        // Sets the new direction
        let mut direction: Direction = self.dir.clone();

        // Sets the new is_second_line 
        let mut is_second: bool = self.is_second_line.clone();

        // Sets the new items_left_on_line
        let mut items_left: i32 = self.items_left_on_line.clone();

        // Sets the new current_items_on_line :
        let mut current_items: i32 = self.current_items_on_line - 1;

        // if current line will overflow when taking the next item
        if current_items == 0 
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
            
            current_items = items_left;
        }
        
        let mut new_item = SpiralMemoryCase::new(
            pos_x,
            pos_y,
            1,
        );

        self.x = pos_x;
        self.y = pos_y;
        self.dir = direction;
        self.current_items_on_line = current_items;
        self.items_left_on_line = items_left;
        self.is_second_line = is_second;

        new_item.compute_value(&self.registered_cases);
        self.registered_cases.push(new_item.clone());

        Some(new_item)
    }

}