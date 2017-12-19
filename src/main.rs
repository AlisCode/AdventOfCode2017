extern crate advent_of_code_2017;

use advent_of_code_2017::day19::{parse_input, get_start_pos, crawl_to_end};


fn main() {
    
    let input: &str = "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ ";

    let test_map = parse_input(input); 

    let (x,y) = get_start_pos(&test_map);
    let res: Vec<char> = crawl_to_end(&test_map, x, y);

    println!("{:?}", res);
}
