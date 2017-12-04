pub fn is_passphrase_ok_first(input: &str) -> bool {
    
	// Get all the words as a list
	let mut list: Vec<&str> =
		input
		.split_whitespace()
		.collect();
		
	// Get the length of the list
	let len: usize = list.len();

	// Sorts, then removes duplicates within the list
	list.sort();
	list.dedup();
	    
	// Compares the new length of the list with the previous one.
	// The idea here is that if the length isn't the same as before, it means that we removed a duplicated word,
	// so the passphrase isn't valid.
	len == list.len()
}

/// Second part
pub fn is_passphrase_ok_second(input: &str) -> bool {
    
	// Gets all word's hashes in a list
	let mut list: Vec<i32> =
    input
    	.split_whitespace()
    	.map(|a| prime_sum(a))
    	.collect();
	
	// Same as part 1 :)
    let len: usize = list.len();
    
	list.sort();
    list.dedup();
    
	// Return the passphrase's "validity"
    len == list.len()
}

/// Calculates the prime hash for a given word
pub fn prime_sum(input: &str) -> i32 {

	// Stores the 26 first primes so we can map each char to its prime number
    let primes = vec!(2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101);

	// For each char, multiply its ASCII value (char as u8 or u16 in this case) 
	// by the corresponding prime in the primes array 
    // then sum all the chars up, we then get the hash of the word's letters 
	input
	    .chars()
	    .map(|a| (((a as u16) * primes[((a as u16) - 97) as usize])) as i32)
	    .sum()

	// A word's hash is by nature unique, because of the multiplication with prime numbers. 
	// this means that if two words have the same hash, they can be formed with the same letters
	// i.e. : they are anagrams.
}
