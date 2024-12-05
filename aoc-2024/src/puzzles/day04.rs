use regex::Regex;

pub fn part_a(input: &str) -> i32 {
    0
}
pub fn part_b(input: &str) -> i32 {
    0
}

#[test]
fn test_part_a() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let actual = part_a(input);

    let expected = 161;

    assert!(actual == expected, "expected {}; got {}", expected, actual)
}
#[test]
fn test_part_b() {
    let input = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let actual = part_b(input);

    let expected = 48;

    assert!(actual == expected, "expected {}; got {}", expected, actual)
}
