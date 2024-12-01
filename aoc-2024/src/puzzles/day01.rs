pub fn part_a(input: &str) -> i32 {
    let (mut lr, mut rh) = parse_input(input);
    assert!(lr.len() == rh.len(), "List must be same length");

    lr.sort();
    rh.sort();

    lr.iter()
        .copied()
        .zip(rh.iter().copied())
        .fold(0, |acc, e| acc + (e.0 - e.1).abs())
}
pub fn part_b(input: &str) -> i32 {
    let (lr, rh) = parse_input(input);
    assert!(lr.len() == rh.len(), "List must be same length");

    let mut acc = 0;
    for x in lr.iter() {
        acc += x * (rh.iter().copied().filter(|e| e == x).count() as i32);
    }
    acc
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let v = input
        .lines()
        .flat_map(|x| x.split_whitespace().map(|x| x.parse::<i32>().unwrap()))
        .collect::<Vec<i32>>();

    let size = v.len();
    let mut lr: Vec<i32> = vec![0; size / 2];
    let mut rh: Vec<i32> = vec![0; size / 2];

    for i in (0..size - 1).step_by(2) {
        lr.push(v[i]);
        rh.push(v[i + 1]);
    }

    (lr, rh)
}

#[test]
fn test_part_a() {
    let input = r"
3   4
4   3
2   5
1   3
3   9
3   3
    ";
    let actual = part_a(input);
    assert!(actual == 11, "Got {}", actual)
}
#[test]
fn test_part_b() {
    let input = r"
3   4
4   3
2   5
1   3
3   9
3   3
    ";
    let actual = part_a(input);
    assert!(actual == 11, "Got {}", actual)
}
