use day9_parser::Content;

pub fn calc_total_score(content: Content, curr_val: u32) -> u32 {

    let mut to_return: u32 = curr_val;

    match content {
        Content::Garbage(_) => {
            return 0;
        }
        Content::Group(groups) => {
            for group in groups {
                to_return += calc_total_score(group, curr_val + 1);
            }
        }
    };

    to_return

}

pub fn calc_total_garbage(content: &Content) -> u32 {
    match content {
        &Content::Garbage(val) => val,
        &Content::Group(ref groups) => groups.iter().map(|a| calc_total_garbage(a)).sum(),
    }
}