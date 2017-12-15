struct Generator {
    pub value: i64,
    pub factor: i64,
}

struct GeneratorSecond {
    pub value: i64,
    pub factor: i64,
    pub criteria: i64,
}

impl Iterator for Generator {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        let new_val = (self.value * self.factor) % 2147483647;

        self.value = new_val;
        Some(new_val)
    }
}

impl Iterator for GeneratorSecond {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        let mut new_val: i64 = (self.value * self.factor) % 2147483647;
        while (new_val % self.criteria) != 0 {
            self.value = new_val;
            new_val = (self.value * self.factor) % 2147483647;
        }
        self.value = new_val;
        Some(new_val)
    }
}

impl Generator {
    pub fn new(start_value: i64, factor: i64) -> Self {
        Generator {
            value: start_value,
            factor,
        }
    }
}

impl GeneratorSecond {
    pub fn new(start_value: i64, factor: i64, criteria: i64) -> Self {
        GeneratorSecond {
            value: start_value,
            factor,
            criteria,
        }
    }
}

pub fn resolve_part_one(a_start: i64, b_start: i64) -> i64 {
    let mut gen_a: Generator = Generator::new(a_start, 16807);
    let mut gen_b: Generator = Generator::new(b_start, 48271);

    let mut count: i64 = 0;
    for _ in 0..40_000_000 {
        let val_a: i64 = gen_a.next().unwrap();
        let val_b: i64 = gen_b.next().unwrap();
        if val_a & 65535 == val_b & 65_535 {
            count += 1;
        }
    }

    count
}

pub fn resolve_part_two(a_start: i64, b_start: i64) -> i64 {
    let mut gen_a: GeneratorSecond = GeneratorSecond::new(a_start, 16807, 4);
    let mut gen_b: GeneratorSecond = GeneratorSecond::new(b_start, 48271, 8);

    let mut count: i64 = 0;
    for _ in 0..5_000_000 {
        let val_a: i64 = gen_a.next().unwrap();
        let val_b: i64 = gen_b.next().unwrap();
        if val_a & 65535 == val_b & 65_535 {
            count += 1;
        }
    }

    count
}