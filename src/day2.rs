/// Resolves the first part of the problem
pub fn decode_spreadsheet(sheet: &str) -> u32 {
    sheet
        .lines()
        .filter_map(find_min_max)
        .map(|(min, max)| max-min)
        .sum()
}

/// Resolves the second part of the problem
pub fn decode_spreadsheet_second(sheet: &str) -> u32 {
    sheet
        .lines()
        .filter_map(find_evenly_divisible_numbers)
        .map(|(a,b)| a / b)
        .sum()
}

/// Finds the minimum and maximum of a list given as white-spaced u32-containable numbers 
/// returns a tuple containing (min, max)
pub fn find_min_max(line: &str) -> Option<(u32, u32)>
{
    line
        .split_whitespace()
        .filter_map(|a| a.parse::<u32>().ok())
        .fold(
            None,
            |min_max, number| if let Some((min, max)) = min_max {
                Some((u32::min(min, number), u32::max(max, number)))
            } else {
                Some((number, number))
        })
}

/// Finds the two only evenly divisible numbers of a list give as white-spaced u32-containable numbers
/// returns a tuple containing (a,b) the two evenly divisible numbers of the list, or None if we didn't find any
pub fn find_evenly_divisible_numbers(line: &str) -> Option<(u32, u32)> {
    
    // Gets the list as u32 numbers
    // NB : the list is mutable because we need to sort it
    let mut list: Vec<u32> = 
    line
    .split_whitespace()
    .filter_map(|a| a.parse::<u32>().ok())
    .collect();

    // Sorts the list so we have numbers in ascending order (i.e here, number a evenly divides number b)
    list.sort();

    // Iterates over the rest of the list, searching for the number that would evenly divide the number a that we're observing
    for (i, &a) in list.iter().enumerate() {
        for &b in list[i + 1..].iter() {
            // Found it! Returning the tuple ...
            if b % a == 0 { return Some((b,a)); }
        }
    }

    // In case we haven't found anything, though it is impossible in this case theoretically... 
    None
}