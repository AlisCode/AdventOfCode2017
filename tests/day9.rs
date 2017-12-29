extern crate day9_parser;
extern crate advent_of_code_2017;

use day9_parser::{Content, parse_input};
use advent_of_code_2017::day9::calc_total_score;

#[test]
fn test_day9_garbage_sample1() {
    assert_eq!(parse_input("<>"), Content::Garbage(0));
}

#[test]
fn test_day9_garbage_sample2() {
    assert_eq!(parse_input("<random characters>"), Content::Garbage(17));
}

#[test]
fn test_day9_garbage_sample3() {
    assert_eq!(parse_input("<<<<>"), Content::Garbage(3));
}

#[test]
fn test_day9_garbage_sample4() {
    assert_eq!(parse_input("<{!>}>"), Content::Garbage(2));
}

#[test]
fn test_day9_garbage_sample5() {
    assert_eq!(parse_input("<!!>"), Content::Garbage(0));
}

#[test]
fn test_day9_garbage_sample6() {
    assert_eq!(parse_input("<!!!>>"), Content::Garbage(0));
}

#[test]
fn test_day9_garbage_sample7() {
    assert_eq!(parse_input("<{o\"i!a,<{i<a>"), Content::Garbage(10));
}

#[test]
fn test_day9_group_sample1() {
    assert_eq!(parse_input("{}"), Content::Group(vec![]));
}

#[test]
fn test_day9_group_sample2() {
    assert_eq!(
        parse_input("{{{}}}"),
        Content::Group(vec![Content::Group(vec![Content::Group(vec![])])])
    );
}

#[test]
fn test_day9_group_sample3() {
    assert_eq!(
        parse_input("{{},{}}"),
        Content::Group(vec![Content::Group(vec![]), Content::Group(vec![])])
    );
}

#[test]
fn test_day9_group_sample4() {
    assert_eq!(
        parse_input("{{{},{},{{}}}}"),
        Content::Group(vec![
            Content::Group(vec![
                Content::Group(vec![]),
                Content::Group(vec![]),
                Content::Group(vec![Content::Group(vec![])]),
            ]),
        ])
    );
}

#[test]
fn test_day9_group_sample5() {
    assert_eq!(
        parse_input("{<a>,<a>,<a>,<a>}"),
        Content::Group(vec![
            Content::Garbage(1),
            Content::Garbage(1),
            Content::Garbage(1),
            Content::Garbage(1),
        ])
    );
}

#[test]
fn test_day9_group_sample6() {
    assert_eq!(
        parse_input("{{<ab>},{<ab>},{<ab>},{<ab>}}"),
        Content::Group(vec![
            Content::Group(vec![Content::Garbage(2)]),
            Content::Group(vec![Content::Garbage(2)]),
            Content::Group(vec![Content::Garbage(2)]),
            Content::Group(vec![Content::Garbage(2)]),
        ])
    );
}

#[test]
fn test_day9_group_sample7() {
    assert_eq!(
        parse_input("{{<!!>},{<!!>},{<!!>},{<!!>}}"),
        Content::Group(vec![
            Content::Group(vec![Content::Garbage(0)]),
            Content::Group(vec![Content::Garbage(0)]),
            Content::Group(vec![Content::Garbage(0)]),
            Content::Group(vec![Content::Garbage(0)]),
        ])
    );
}

#[test]
fn test_day9_group_sample8() {
    assert_eq!(
        parse_input("{{<a!>},{<a!>},{<a!>},{<ab>}}"),
        Content::Group(vec![Content::Group(vec![Content::Garbage(17)])])
    );
}


#[test]
fn test_day9_score_sample1() {
    let group = parse_input("{}");
    assert_eq!(calc_total_score(group, 1), 1);
}

#[test]
fn test_day9_score_sample2() {
    let group = parse_input("{{{}}}");
    assert_eq!(calc_total_score(group, 1), 6);
}

#[test]
fn test_day9_score_sample3() {
    let group = parse_input("{{},{}}");
    assert_eq!(calc_total_score(group, 1), 5);
}

#[test]
fn test_day9_score_sample4() {
    let group = parse_input("{{{},{},{{}}}}");
    assert_eq!(calc_total_score(group, 1), 16);
}

#[test]
fn test_day9_score_sample5() {
    let group = parse_input("{<a>,<a>,<a>,<a>}");
    assert_eq!(calc_total_score(group, 1), 1);
}

#[test]
fn test_day9_score_sample6() {
    let group = parse_input("{{<ab>},{<ab>},{<ab>},{<ab>}}");
    assert_eq!(calc_total_score(group, 1), 9);
}

#[test]
fn test_day9_score_sample7() {
    let group = parse_input("{{<!!>},{<!!>},{<!!>},{<!!>}}");
    assert_eq!(calc_total_score(group, 1), 9);
}

#[test]
fn test_day9_score_sample8() {
    let group = parse_input("{{<a!>},{<a!>},{<a!>},{<ab>}}");
    assert_eq!(calc_total_score(group, 1), 3);
}