/// Resolves the first part for the given input
pub fn count_steps_first(input: &str) -> i32 {
    
    // Collects the offset_list;
    // For some reason, lines() doesn't work ???
    let mut offset_list: Vec<i32> = input
        .split_whitespace()
        .filter_map(|a| a.parse::<i32>().ok())
        .collect();

    // Initializes the index
    let mut index: usize = 0;

    // Infinite for loop 
    for counted_steps in 1.. {

        // Takes the offset that we're going to add
        let offset_to_add: i32 = offset_list[index];

        // Because we're adding to an usize, we need to use the checked_add and checked_sub functions
        let calculated_offset = if offset_to_add < 0 {
            index.checked_sub(offset_to_add.abs() as usize)
        } else {
            index.checked_add(offset_to_add as usize)
        };

        // This gives us an option on a usize variable that we can then match in order to know 
        // whether we should return the number of steps that the for loop counted 
        // or simply continue doing "the offset game"
        match calculated_offset {
            // Conditional matching. I love Rust.
            Some(offset) if offset < offset_list.len() => {
                offset_list[index] = offset_list[index] + 1;
                index = offset as usize;
            }
            // If we didnt get an offset, or it is off bounds, just return the counted steps :)
            _ => return counted_steps,
        }
    }
    // Won't compile without this, since this function needs to return an i32.
    // Basically, this is unreachable code ^^
    0
}

/// Resolves the second part for the given input
pub fn count_steps_second(input: &str) -> i32 {

    // BASICALLY THE SAME AS PART 1
    let mut offset_list: Vec<i32> = input
        .split_whitespace()
        .filter_map(|a| a.parse::<i32>().ok())
        .collect();

    let mut index: usize = 0;

    for counted_steps in 1.. {
        let offset_to_add: i32 = offset_list[index];
        let calculated_offset = if offset_to_add < 0 {
            index.checked_sub(offset_to_add.abs() as usize)
        } else {
            index.checked_add(offset_to_add as usize)
        };

        match calculated_offset {
            Some(offset) if offset < offset_list.len() => {
                // The thing here is that we check what the offset that we added was
                // in order to follow the simple/weird rule that part. 2 gave us
                if offset_to_add >= 3 {
                    offset_list[index] = offset_list[index] - 1;
                } else {
                    offset_list[index] = offset_list[index] + 1;
                }
                index = offset as usize;
            }
            // Return the result :)
            _ => return counted_steps,
        }
    }
    // Same as before :)
    0
}