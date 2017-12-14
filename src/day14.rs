use day10::KnotHasher;

pub fn create_grid_for_input(input: &str) -> Vec<Vec<i32>> {
    let mut hasher = KnotHasher::new();

    (0..128)
        .map(|a| {
            let new_input: String = format!("{}-{}", input, a);
            hasher.do_knot_for_input_second(new_input.as_str());
            hash_to_binary(hasher.get_dense_hash())
        })
        .map(|a| {
            let mut line: Vec<i32> = Vec::new();
            a.chars().for_each(|b| if b == '1' {
                line.push(1);
            } else {
                line.push(0);
            });
            line
        })
        .collect()
}

pub fn hash_to_binary(input: String) -> String {
    input
        .chars()
        .map(|a| format!("{:04b}", a.to_digit(16).unwrap()))
        .collect()
}

pub fn count_used_cells(grid: &Vec<Vec<i32>>) -> i32 {
    grid.iter()
        .map(|a| a.iter().filter(|b| *b == &1).count() as i32)
        .sum()
}

pub fn count_groups(grid: &Vec<Vec<i32>>) -> i32 {
    let mut nb_groups: i32 = 0;

    for i in 0..128 {
        for j in 0..128 {
            if get_case(grid, i, j) == 1 {
                if !(get_case(grid, i as i32 - 1, j as i32) == 1
                    || get_case(grid, i as i32 + 1, j as i32) == 1
                    || get_case(grid, i as i32, j as i32 - 1) == 1
                    || get_case(grid, i as i32, j as i32 + 1) == 1)
                {
                    nb_groups += 1;
                }
            }
        }
    }

    nb_groups
}

fn get_case(grid: &Vec<Vec<i32>>, x: i32, y: i32) -> i32 {
    if x < 0 || x >= 128 || y < 0 || y >= 128 {
        return 0;
    }
    grid[x as usize][y as usize]
}
