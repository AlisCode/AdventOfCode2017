pub struct HexaPath {
    nw_count: i32,
    n_count: i32,
    ne_count: i32,
    sw_count: i32,
    s_count: i32,
    se_count: i32,
}

impl HexaPath {
    pub fn new() -> Self {
        HexaPath {
            nw_count: 0,
            n_count: 0,
            ne_count: 0,
            sw_count: 0,
            s_count: 0,
            se_count: 0,
        }
    }

    pub fn populate(&mut self, input: &str) {
        input.split(",").for_each(|a| match a {
            "nw" => self.nw_count += 1,
            "n" => self.n_count += 1,
            "ne" => self.ne_count += 1,
            "sw" => self.sw_count += 1,
            "s" => self.s_count += 1,
            "se" => self.se_count += 1,
            _ => unreachable!(),
        });
    }

    pub fn get_steps(&self) -> i32 {
        let east_count: i32 = self.ne_count - self.sw_count;
        let west_count: i32 = self.nw_count - self.se_count;
        let center_count: i32 = self.n_count - self.s_count;

        east_count.abs() + west_count.abs() + center_count.abs()
    }
}
