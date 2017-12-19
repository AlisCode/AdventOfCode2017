extern crate advent_of_code_2017;

use advent_of_code_2017::day19::{crawl_to_end, parse_input, get_start_pos};

#[test]
pub fn test_day19_path_sample1() {
    let input: &str = "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ ";


    let map = parse_input(input);
    let (x,y) = get_start_pos(&map);
    let (res, steps) = crawl_to_end(&map, x, y);

    assert_eq!(res, vec!('A','B','C','D','E','F'));
    assert_eq!(steps, 38);

}
