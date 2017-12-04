pub fn is_passphrase_ok_first(input: &str) -> bool {
    
    let mut list: Vec<&str> =
    input
        .split_whitespace()
        .collect();
        
    let len: usize = list.len();

    list.sort();
    list.dedup();
    
    len
    ==
    list.len()
}

pub fn is_passphrase_ok_second(input: &str) -> bool {
    let mut list: Vec<i32> =
    input
        .split_whitespace()
        .map(|a| prime_sum(a))
        .collect();
        
    let len: usize = list.len();

    list.sort();
    list.dedup();
    
    len
    ==
    list.len()
}

fn prime_sum(input: &str) -> i32 {

    let primes = vec!(2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101);

    input
    .chars()
    .map(|a| (((a as u16) * primes[((a as u16) - 97) as usize])) as i32)
    .sum()
}
