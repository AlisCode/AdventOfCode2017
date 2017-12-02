pub fn decode_spreadsheet(sheet: &str) -> u32 {
    sheet
        .split("\n")
        .map(|a| {
            a.split_whitespace()
                .map(|b| b.to_string().parse::<u32>().unwrap())
                .max()
                .unwrap() -
                a.split_whitespace()
                    .map(|b| b.to_string().parse::<u32>().unwrap())
                    .min()
                    .unwrap()
        })
        .sum()
}

fn find_evenly_divisible_values(list: Vec<u32>) -> (u32, u32)
{
    let todo: Vec<u32> = list.clone();
    let mut return_tuple: (u32,u32) = (1,1);
    todo.into_iter().for_each(|a| {
        let elem: u32 = list.iter().cloned().filter(|b| are_evenly_divisible(a,*b)).last().unwrap_or(0);
        if elem != 0 { return_tuple = ordered_tuple(a, elem); return; }
    });

    return_tuple
}

fn are_evenly_divisible(a: u32, b: u32) -> bool
{
    if a > b { return a % b == 0 }
    else if a < b { return b % a == 0 }
    false
}

fn ordered_tuple(a: u32, b: u32) -> (u32, u32)
{
    if a > b { return (a,b); }
    return (b,a);
}

pub fn decode_spreadsheet_second(sheet: &str) -> u32 {

    sheet
        .split("\n")
        .map(|a| {

            let (b, c) = find_evenly_divisible_values(
                a.split_whitespace()
                    .map(|b| b.to_string().parse::<u32>().unwrap())
                    .collect()
            );

            b / c
        })
        .sum()
}
