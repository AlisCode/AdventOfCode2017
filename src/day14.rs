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

pub fn count_groups(grid: &mut Vec<Vec<i32>>) -> i32 {
    let mut nb_groups: i32 = 0;

    for i in 0..128 {
        for j in 0..128 {
            if grid[i][j] == 1 {
                recursive_check_neighbors(grid, i, j);
                nb_groups += 1;
            }
        }
    }

    nb_groups
}

fn recursive_check_neighbors(grid: &mut Vec<Vec<i32>>, x: usize, y: usize)
{  
    grid[x][y] = 0;
    if x <= 126 && grid[x+1][y] == 1 { recursive_check_neighbors(grid, x+1, y); }
    if y <= 126 && grid[x][y+1] == 1 { recursive_check_neighbors(grid, x, y+1); }
    if x >= 1 && grid[x-1][y] == 1 { recursive_check_neighbors(grid, x-1, y); }
    if y >= 1 && grid[x][y-1] == 1 { recursive_check_neighbors(grid, x, y-1); }
    
    if x <= 126 { grid[x+1][y] = 0; }
    if y <= 126 { grid[x][y+1] = 0; }
    if x >= 1 { grid[x-1][y] = 0; }
    if y >= 1 { grid[x][y-1] = 0; }
} 