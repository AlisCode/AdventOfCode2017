use std::collections::HashMap;

/// Declares the FirewallBlock struct
#[derive(Clone)]
pub struct FirewallBlock {
    depth: i32,
    range: i32,
    cursor_pos: i32,
    going_down: bool,
}

impl FirewallBlock {
    /// Constructor
    pub fn new(depth: i32, range: i32) -> Self {
        FirewallBlock {
            depth,
            range,
            cursor_pos: 1,
            going_down: true,
        }
    }

    /// Gets the severity of being scanned by this FirewallBlock
    pub fn get_severity(&self) -> i32 {
        self.depth * self.range
    }

    /// Makes the cursor move
    pub fn tick(&mut self) {
        if self.going_down {
            self.cursor_pos += 1;
        } else {
            self.cursor_pos -= 1;
        }
        if self.cursor_pos == 1 {
            self.going_down = true;
        } else if self.cursor_pos == self.range {
            self.going_down = false;
        }
    }

    /// Allows us to know if the firewall is scanning the index that we're asking
    pub fn is_scanning_line(&self, index: i32) -> bool {
        index == self.cursor_pos
    }
}

/// Generates the firewall, returning a tuple of a HashMap containing the FirewallBlocks identified by their depth, and the max depth that we're reaching
pub fn generate_firewall(input: &str) -> (HashMap<i32, FirewallBlock>, i32) {
    let mut hash_map: HashMap<i32, FirewallBlock> = HashMap::new();
    let mut max_depth: i32 = 0;

    input
        .lines()
        .map(|a| {
            let mut split = a.split(": ");
            let depth: i32 = split.next().unwrap().parse::<i32>().unwrap();
            let range: i32 = split.next().unwrap().parse::<i32>().unwrap();
            if depth > max_depth {
                max_depth = depth;
            }
            FirewallBlock::new(depth, range)
        })
        .for_each(|a| {
            hash_map.insert(a.depth, a);
        });

    (hash_map, max_depth)
}

pub fn get_path_severity(hash_map: &mut HashMap<i32, FirewallBlock>, max_depth: i32, delay: i32) -> (i32, bool) {
    
    let mut curr_index: i32 = 0;
    let mut global_severity: i32 = 0;
    let mut to_delay: i32 = delay.clone();
    let mut got_caught: bool = false;
    let mut _i: i32 = 0;

    for _ in 0.. {
        if to_delay > 0 {
            to_delay -= 1;
        }
        else {
            match hash_map.get(&curr_index.clone()) {
                Some(block) => if block.is_scanning_line(1) {
                    global_severity += block.get_severity();
                    got_caught = true;
                },
                None => _i += 1,
            }
            if curr_index == max_depth {
                return (global_severity, got_caught);
            }
            curr_index += 1;
        }
        hash_map.values_mut().for_each(|a| a.tick());
    }

    unreachable!();
}

pub fn get_min_delay(hash_map: &mut HashMap<i32, FirewallBlock>, max_depth: i32) -> i32 {
    for i in 1.. {
        let mut copied_hash = hash_map.clone();
        let (sev, got_caught) = get_path_severity(&mut copied_hash, max_depth, i);
        if !got_caught {
            return i;
        }
        else {
            println!("{} Got caught: {}", i, sev);
        }
    }

    unreachable!();
}


