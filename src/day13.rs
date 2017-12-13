/// Declares the FirewallBlock struct
pub struct FirewallBlock {
    depth: i32,
    range: i32
}

impl FirewallBlock {
    /// Constructor
    pub fn new(depth: i32, range: i32) -> Self {
        FirewallBlock {
            depth,
            range,
        }
    }

    /// Gets the severity of being scanned by this FirewallBlock
    pub fn get_severity(&self) -> i32 {
        self.depth * self.range
    }

    pub fn catches_packet_at(&self, time: i32) -> bool {
        let period: i32 = 2*(self.range-1);
        time % period == 0
    }
}

/// Generates the firewall, returning a tuple of a HashMap containing the FirewallBlocks identified by their depth, and the max depth that we're reaching
pub fn generate_firewall(input: &str) -> Vec<FirewallBlock> {
    input
        .lines()
        .map(|a| {
            let mut split = a.split(": ");
            let depth: i32 = split.next().unwrap().parse::<i32>().unwrap();
            let range: i32 = split.next().unwrap().parse::<i32>().unwrap();
            FirewallBlock::new(depth, range)
        })
        .collect()
}

/// Returns the path severity for traversing the firewall when not delayed
pub fn get_path_severity(firewall: &Vec<FirewallBlock>) -> i32 {
    firewall
    .iter()
    .map(|a| if a.catches_packet_at(a.depth) { a.get_severity() } else { 0 })
    .sum()
}

fn check_if_get_caught(firewall: &Vec<FirewallBlock>, delay: i32) -> bool {
    let res: i32 = 
    firewall
    .iter()
    .map(|a: &FirewallBlock| if a.catches_packet_at(a.depth + delay) { 1 } else { 0 })
    .sum();

    res == 0
}

pub fn get_min_delay(firewall: &Vec<FirewallBlock>) -> i32 {
    for i in 0.. {
        if check_if_get_caught(firewall, i) { return i; }
    }
    unreachable!();
}


