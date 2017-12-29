use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct Grid {
    pub size: usize,
    pub values: Vec<Vec<bool>>,
}

impl PartialEq for Grid {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }

        // ID
        if self.values == other.values {
            return true;
        }

        // FlipX
        let flip = other.flip_vertical();
        if self.values == flip.values {
            return true;
        }

        // Rot 90
        let rot_one = other.rotate_right();
        if self.values == rot_one.values {
            return true;
        }

        // Rot 180
        let rot_two = rot_one.rotate_right();
        if self.values == rot_two.values {
            return true;
        }

        // Rot 270
        let rot_three = rot_two.rotate_right();
        if self.values == rot_three.values {
            return true;
        }

        // Flip after Rot90
        let flip_after_rot_one = rot_one.flip_vertical();
        if self.values == flip_after_rot_one.values {
            return true;
        }

        // Flip after Rot180
        let flip_after_rot_two = rot_two.flip_vertical();
        if self.values == flip_after_rot_two.values {
            return true;
        }

        // Flip after Rot270
        let flip_after_rot_three = rot_three.flip_vertical();
        if self.values == flip_after_rot_three.values {
            return true;
        }

        false
    }
}

impl Hash for Grid {
    fn hash<H: Hasher>(&self, hasher: &mut H) {

        let mut global_values = Vec::new();

        global_values.push(self.values.clone());

        let flip = self.flip_vertical();
        global_values.push(flip.values.clone());

        let rot = self.rotate_right();
        global_values.push(rot.values.clone());

        let rot_flip = rot.flip_vertical();
        global_values.push(rot_flip.values.clone());

        let rot_two = rot.rotate_right();
        global_values.push(rot_two.values.clone());

        let rot_two_flip = rot_two.flip_vertical();
        global_values.push(rot_two_flip.values.clone());

        let rot_three = rot_two.rotate_right();
        global_values.push(rot_three.values.clone());

        let rot_three_flip = rot_three.flip_vertical();
        global_values.push(rot_three_flip.values.clone());

        global_values.sort();
        global_values.hash(hasher);
    }
}


impl Eq for Grid {}

impl Grid {
    pub fn new(values: Vec<Vec<bool>>) -> Self {
        Grid {
            size: values.len(),
            values,
        }
    }

    pub fn transpose(&self) -> Grid {
        let mut new_grid: Grid = Grid::new(self.values.clone());

        for i in 0..new_grid.size {
            for j in 0..new_grid.size {
                new_grid.values[i][j] = self.values[j][i];
            }
        }

        new_grid
    }

    pub fn rotate_right(&self) -> Grid {
        let mut new_grid: Grid = Grid::new(self.values.clone());

        for i in 0..new_grid.size / 2 {
            for j in 0..(new_grid.size + 1) / 2 {
                let a = self.values[i][j];
                let b = self.values[new_grid.size - 1 - j][i];
                let c = self.values[new_grid.size - 1 - i][new_grid.size - 1 - j];
                let d = self.values[j][new_grid.size - 1 - i];

                new_grid.values[new_grid.size - 1 - i][new_grid.size - 1 - j] = d;
                new_grid.values[new_grid.size - 1 - j][i] = c;
                new_grid.values[i][j] = b;
                new_grid.values[j][new_grid.size - 1 - i] = a;
            }
        }

        new_grid
    }

    pub fn flip_horizontal(&self) -> Grid {
        let mut new_grid: Grid = Grid::new(self.values.clone());

        for i in 0..new_grid.size {
            for j in 0..new_grid.size {
                new_grid.values[i][j] = self.values[new_grid.size - 1 - i][j];
            }
        }

        new_grid
    }

    pub fn flip_vertical(&self) -> Grid {
        let mut new_grid: Grid = Grid::new(self.values.clone());

        for i in 0..new_grid.size {
            for j in 0..new_grid.size {
                new_grid.values[i][j] = self.values[i][new_grid.size - 1 - j];
            }
        }

        new_grid
    }

    pub fn split(&self) -> Vec<Grid> {
        let block_size = match self.size % 2 {
            0 => 2,
            _ => 3,
        };
        if block_size == self.size {
            return vec![self.clone()];
        }

        let grid_per_line = self.size / block_size;
        let num_iter: usize = grid_per_line * grid_per_line;

        (0..num_iter)
            .map(|i| {
                let mut values: Vec<Vec<bool>> = Vec::new();
                let j = i % grid_per_line;
                let k = i / grid_per_line;

                for x in 0..block_size {
                    let mut line_value: Vec<bool> = Vec::new();
                    for y in 0..block_size {
                        line_value.push(
                            self.values[k * block_size + x][j * block_size +
                                                                y]
                                .clone(),
                        );
                    }
                    values.push(line_value);
                }
                Grid::new(values)
            })
            .collect()
    }

    pub fn reconstruct(list: &mut Vec<Grid>) -> Grid {
        if list.len() == 1 {
            return list[0].clone();
        }

        let sub_grids = (list.len() as f64).sqrt() as usize;
        let small_grid_size = list[0].size.clone();

        let num_iter = sub_grids * small_grid_size;

        let mut values: Vec<Vec<bool>> = Vec::new();

        for i in 0..num_iter {
            let base_other = i % small_grid_size;
            let offset_other = i / small_grid_size;

            let mut line_value: Vec<bool> = Vec::new();
            for j in offset_other * sub_grids..offset_other * sub_grids + sub_grids {
                line_value.append(&mut list[j].values[base_other]);
            }
            values.push(line_value);
        }

        Grid::new(values)
    }

    pub fn count_on_cases(&self) -> usize {
        self.values
            .iter()
            .map(|a| a.iter().filter(|&&b| b).count())
            .sum()
    }
}

pub fn parse_input(input: &str) -> HashMap<Grid, Grid> {
    let mut map: HashMap<Grid, Grid> = HashMap::new();

    input.lines().for_each(|a| {
        let mut grids: Vec<Grid> = a.split(" => ").map(|b| parse_grid(b)).collect();

        map.insert(grids.remove(0), grids.remove(0));
    });

    map
}

pub fn parse_grid(input: &str) -> Grid {
    Grid::new(
        input
            .split("/")
            .map(|c| {
                c.chars().fold(Vec::new(), |mut vec, d| {
                    vec.push(map_char(d));
                    vec
                })
            })
            .collect(),
    )
}

fn map_char(c: char) -> bool {
    match c {
        '.' => false,
        '#' => true,
        _ => unreachable!(),
    }
}

pub fn image_processing_iteration(grid: Grid, rules: &HashMap<Grid, Grid>) -> Grid {
    let mut grid: Vec<Grid> = grid.split()
        .iter()
        .filter_map(|a| match rules.get(a) {
            Some(new_grid) => Some(new_grid.clone()),
            _ => Some(a.clone()),
        })
        .collect();

    Grid::reconstruct(&mut grid)
}

pub fn resolve_part_one(input: &str) -> usize {
    let rules = parse_input(input);
    let mut base_grid = parse_grid(".#./..#/###");

    for _ in 0..5 {
        base_grid = image_processing_iteration(base_grid, &rules);
    }

    base_grid.count_on_cases()
}

pub fn resolve_part_two(input: &str) -> usize {
    let rules = parse_input(input);
    let mut base_grid = parse_grid(".#./..#/###");

    for _ in 0..18 {
        base_grid = image_processing_iteration(base_grid, &rules);
    }

    base_grid.count_on_cases()
}
