extern crate advent_of_code_2017;

use advent_of_code_2017::day13::{generate_firewall, get_min_delay};

fn main() {
    let input: &str = "0: 3
1: 2
2: 4
4: 4
6: 5
8: 8
10: 6
12: 6
14: 8
16: 6
18: 6
20: 8
22: 12
24: 8
26: 8
28: 12
30: 8
32: 12
34: 9
36: 14
38: 12
40: 12
42: 12
44: 14
46: 14
48: 10
50: 14
52: 12
54: 14
56: 12
58: 17
60: 10
64: 14
66: 14
68: 12
70: 12
72: 18
74: 14
78: 14
82: 14
84: 24
86: 14
94: 14";

    /*let input: &str = "0: 3
1: 2
4: 4
6: 4";*/

    let (mut hash_map, max_depth) = generate_firewall(input);
    let res = get_min_delay(&mut hash_map, max_depth);

    println!("Result: {}", res);
}
