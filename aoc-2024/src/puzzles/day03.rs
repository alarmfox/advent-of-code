use regex::Regex;

pub fn part_a(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    input
        .lines()
        .map(|s| {
            re.captures_iter(s)
                .map(|c| c.extract())
                .map(|(_, [x, y])| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
                .map(|(x, y)| x * y)
                .sum::<i32>()
        })
        .sum::<i32>()
}
pub fn part_b(input: &str) -> i32 {
    let mut buffer = String::new();
    let mut enabled = true;
    let mut start: usize = 0;
    let mut i: usize = 0;
    let start_guard = "do()";
    let stop_guard = "don't()";
    let start_size = start_guard.len();
    let stop_size = stop_guard.len();
    while i < input.len() {
        if enabled && i < (input.len() - stop_size) && &input[i..i + stop_size] == stop_guard {
            buffer.push_str(&input[start..i]);
            enabled = false;
            i += stop_size - 1;
        } else if !enabled
            && i < (input.len() - start_size)
            && &input[i..i + start_size] == start_guard
        {
            enabled = true;
            start = i;
            i += start_size - 1
        } else {
            i += 1
        }
    }
    if enabled {
        buffer.push_str(&input[start..]);
    }

    part_a(buffer.as_str())
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
