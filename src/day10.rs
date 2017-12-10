pub struct KnotHasher {
    pub list: Vec<i32>,
    pub current_position: usize,
    pub skip_size: usize,
}

impl KnotHasher {
    pub fn new() -> Self {
        
        KnotHasher {
            list: (0..256).collect(),
            current_position: 0,
            skip_size: 0,
        }
    }

    pub fn do_knot_for_input_first(&mut self, input: &str) {
        input.split(",").for_each(|a| {
            self.do_knot(a.parse::<usize>().unwrap());
        });
    }

    pub fn do_knot_for_input_second(&mut self, input: &str) {
        
        // suffixes the input stream
        let mut input_suffixed: Vec<u8> = input.chars().map(|a| a as u8).collect();
        
        input_suffixed.push(17);
        input_suffixed.push(31);
        input_suffixed.push(73);
        input_suffixed.push(47);
        input_suffixed.push(23);

        // Do the knotting on the suffixed input stream
        input_suffixed.into_iter().for_each(|a| {
            let val = a as u8;
            for _ in 1..64 {
                self.do_knot(val as usize);
            }
        });
    }

    pub fn do_knot(&mut self, knot_length: usize) {
        let mut curr_pos: usize = self.current_position.clone();
        let mut reversed_list: Vec<i32> = Vec::new();
        for _ in 0..knot_length {
            reversed_list.push(self.list[curr_pos].clone());
            curr_pos = curr_pos + 1;
            if curr_pos >= self.list.len() {
                curr_pos = 0;
            }
        }

        let mut iter_reversed_list = reversed_list.into_iter().rev();

        curr_pos = self.current_position.clone();
        for _ in 0..knot_length {
            let val: i32 = iter_reversed_list.next().unwrap();
            self.list[curr_pos] = val;
            curr_pos = curr_pos + 1;
            if curr_pos >= self.list.len() {
                curr_pos = 0;
            }
        }

        self.current_position = self.current_position + knot_length + self.skip_size;
        self.current_position = self.current_position % self.list.len();
        self.skip_size = self.skip_size + 1;
    }

    pub fn get_result(&mut self) -> i32 {
        self.list[0] * self.list[1]
    }
}