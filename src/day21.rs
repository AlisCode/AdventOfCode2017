use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Hash, Clone, Debug)]
pub struct Grid {
    pub size: usize,
    pub values: Vec<Vec<bool>>,
}

impl PartialEq for Grid {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }
        if self.values == other.values {
            return true;
        }

        let one = self.transpose();
        if self.values == one.values {
            return true;
        }
        let two = one.flip_vertical();
        if self.values == two.values {
            return true;
        }
        let three = two.transpose();
        if self.values == three.values {
            return true;
        }
        let four = three.flip_vertical();
        if self.values == four.values {
            return true;
        }
        let five = four.transpose();
        if self.values == five.values {
            return true;
        }
        let six = five.flip_vertical();
        if self.values == six.values {
            return true;
        }
        let seven = six.transpose();
        if self.values == seven.values {
            return true;
        }
        let eight = seven.flip_vertical();
        if self.values == eight.values {
            return true;
        }

        false
    }
}

impl Hash for Grid {
    fn hash<H: Hasher>(&self, hasher: H) {
        let one = self.transpose();

        let two = one.flip_vertical();

        let three = two.transpose();

        let four = three.flip_vertical();

        let five = four.transpose();

        let six = five.flip_vertical();

        let seven = six.transpose();

        let eight = seven.flip_vertical();
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
        let block_size = match self.size % 3 {
            0 => 3,
            _ => 2,
        };
        if block_size == self.size {
            return vec![self.clone()];
        }

        (0..self.size)
            .map(|i| {
                let mut values: Vec<Vec<bool>> = Vec::new();
                let j = i % block_size;
                let k = i / block_size;
                for x in 0..block_size {
                    let mut line_value: Vec<bool> = Vec::new();
                    for y in 0..block_size {
                        line_value
                            .push(self.values[k * block_size + x][j * block_size + y].clone());
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

        let mut base_other: usize = 0;
        let mut offset: usize = 0;
        let mut offset_other: usize = 0;

        for i in 0..num_iter {
            offset = i / sub_grids;
            base_other = i % small_grid_size;
            offset_other = i / small_grid_size;

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
        .filter_map(|a| {
            println!("{:?}", a);
            match rules.get(a) {
                Some(new_grid) => {
                    println!("Matched some");
                    Some(new_grid.clone())
                }
                _ => {
                    println!("Didnt match some");
                    Some(a.clone())
                }
            }
        })
        .collect();

    Grid::reconstruct(&mut grid)
}

pub fn resolve_part_one(input: &str) -> usize {
    let rules = parse_input(input);
    let mut base_grid = parse_grid(".#./..#/###");
    base_grid = image_processing_iteration(base_grid, &rules);
    base_grid = image_processing_iteration(base_grid, &rules);

    base_grid.count_on_cases()
}
