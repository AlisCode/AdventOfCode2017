pub struct HexaCase {
    x: i32,
    y: i32,
    z: i32,
    furthest: i32,
}

impl HexaCase {
    pub fn new() -> Self {
        HexaCase {
            x: 0,
            y: 0,
            z: 0,
            furthest: 0,
        }
    }

    pub fn follow_path(&mut self, input: &str) {
        input.split(",").for_each(|a| { match a {
            "nw" => { self.y += 1; self.x -= 1;},
            "n" => { self.y += 1; self.z -= 1;},
            "ne" => { self.x += 1; self.z -= 1;},
            "sw" => { self.z += 1; self.x -= 1;},
            "s" => { self.z += 1; self.y -= 1;},
            "se" => { self.x += 1; self.y -= 1;},
            _ => unreachable!(),
        };

        let steps: i32 = self.get_steps();
        if self.furthest < steps { self.furthest = steps; }

        });
    }

    pub fn get_steps(&self) -> i32 {

        let vec_compare: Vec<i32> = vec!(self.x, self.y, self.z);
        vec_compare.into_iter().max().unwrap()

    }

    pub fn get_furthest(&self) -> i32 {
        self.furthest
    } 
}
