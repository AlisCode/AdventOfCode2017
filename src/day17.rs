pub struct Spinlock {
    current_position: usize,
    values: Vec<u32>,
    step_factor: usize,
}

/// Cleanest solution I could come up with...
impl Spinlock {
    pub fn new(step_factor: usize) -> Self {
        Spinlock {
            current_position: 0,
            values: vec![0],
            step_factor,
        }
    }

    /// Inserts the next value
    pub fn insert_next_val(&mut self, val: u32) {
        self.current_position = (self.current_position + self.step_factor) % self.values.len() + 1;
        self.values.insert(self.current_position, val);
    }

    /// Get the value to solve part one
    pub fn get_val_after_current(&self) -> u32 {
        self.values[(self.current_position + 1) % self.values.len()]
    }
}

/// Uses the struct to solve part one
pub fn resolve_part_one(step_factor: usize) -> u32 {
    let mut spinlock: Spinlock = Spinlock::new(step_factor);
    for i in 1..2018 {
        spinlock.insert_next_val(i);
    }
    
    spinlock.get_val_after_current()
}


/// Using the spinlock struct is not viable here : time is rising up fast as we're adding values to the spinlock's values Vector.
/// Instead, because we are inserting the values AFTER the other, we know for sure that 0 is going to stay to the left of the list
/// if that makes sense ...
/// The idea is to keep track of what the value at position 1 (i.e. just after 0) is. (Also, we dont care about the other values)
pub fn resolve_part_two(step_factor: u32) {

    let mut curr_pos: u32 = 0;
    let mut val_at_pos_one: u32 = 0;

    for i in 1..50_000_001 {
        curr_pos = (curr_pos + step_factor) % i + 1;
        if curr_pos == 1 {
            val_at_pos_one = i;
        }
    }

    println!("Result: {}", val_at_pos_one);
}
