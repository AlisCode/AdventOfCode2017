use std::collections::HashSet;
use std::collections::HashMap;

/// processes a redistribution cycle
pub fn redistribution_cycle(list: &mut Vec<i32>) {
    // Finds the index of the max element (ties decided by lower index)
    let (index, &v) = list.iter()
        .enumerate()
        .rev()
        .max_by_key(|&(_, &v)| v)
        .unwrap();

    // Gets the current index
    let mut curr_index: usize = index.clone();

    // Sets the item at current index to 0
    list[curr_index] = 0;

    // Redistributes its value, cycling when it ends
    for _ in 0..v {
        curr_index = curr_index + 1;
        if curr_index >= list.len() {
            curr_index = 0
        }
        list[curr_index] = list[curr_index] + 1;
    }
}

// resolves the first part
pub fn count_redistribution_cycle(input: &str) -> i32 {
    // Instantiates a list with the given input
    let mut list: Vec<i32> = input
        .split_whitespace()
        .filter_map(|a| a.parse::<i32>().ok())
        .collect();

    // HashSet of previous lists : won't allow us to insert two times the same list
    let mut previous_lists: HashSet<Vec<i32>> = HashSet::new();

    // Adds the first list to the previous lists
    previous_lists.insert(list.clone());
    for i in 1.. {
        // Redistributes the list
        redistribution_cycle(&mut list);

        // If we already inserted this list, previous_lists.insert() will return false
        if !previous_lists.insert(list.clone()) {
            // So we return the steps
            return i;
        }
    }

    // This should be unreachable
    unreachable!()
}

/// resolves the second part
pub fn count_redistribution_cycle_loop(input: &str) -> i32 {
    // Essentially the same thing as part 1
    let mut list: Vec<i32> = input
        .split_whitespace()
        .filter_map(|a| a.parse::<i32>().ok())
        .collect();

    // ... except that we are now using HashMap, because this allows us to keep track
    // of the indexes of our lists.
    let mut previous_lists = HashMap::new();

    previous_lists.insert(list.clone(), 0);
    for i in 1.. {
        redistribution_cycle(&mut list);

        // previous_lists.insert() now gives us an option on the index of the
        // previously inserted list
        if let Some(prev_index) = previous_lists.insert(list.clone(), i) {
            // So we can return it by substracting it to the index of the "déjà vu" situation
            return (i - prev_index) as i32;
        }
    }

    // Same as before, we should not be able to get out of the for loop
    unreachable!()
}

/// function to provide a convenient way of testing for the redistribution cycle
pub fn do_redistribution_cycle(input: &str) -> Vec<i32> {
    let mut list: Vec<i32> = input
        .split_whitespace()
        .filter_map(|a| a.parse::<i32>().ok())
        .collect();

    redistribution_cycle(&mut list);
    list
}
