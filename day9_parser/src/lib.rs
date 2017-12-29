#[macro_use]
extern crate nom;

use nom::anychar;

#[derive(Debug, PartialEq, Eq)]
pub enum Content {
    Group(Vec<Content>),
    Garbage(u32),
}

named!(
    get_content<Content>,
    alt!(
        get_garbage |
        get_group
    )
);

named!(get_garbage<Content>,
    map!(
        delimited!(
            tag!("<"),
            fold_many0!(get_garbage_val, 0, |sum, acc| sum + acc),
            tag!(">")
        )
        , Content::Garbage
    )
);

named!(get_garbage_val<u32>,
    alt!(
        value!(0, tuple!(tag!("!"), anychar)) | 
        value!(1, tuple!(not!(char!('>')), anychar))
    )
);

named!(
    get_group<Content>,
    map!(
        delimited!(
        tag!("{"),
        separated_list_complete!(tag!(","), get_content),
        tag!("}"))
        , Content::Group
    )
);

pub fn parse_input(input: &str) -> Content {
    get_content(input.as_bytes()).to_result().unwrap()
}