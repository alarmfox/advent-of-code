pub fn part_a(input: &str) -> i32 {
    #[derive(Debug)]
    enum Status {
        DECR,
        INCR,
        INVALID,
        UNDEF,
    }
    let reports = parse_input(input);

    assert!(reports.len() > 0, "No reports provided");

    let mut acc = 0;
    for report in reports.iter() {
        assert!(report.len() > 1, "A report must have at least 2 levels");
        let mut status = Status::UNDEF;

        for i in 0..report.len() - 1 {
            let diff = report[i] - report[i + 1];
            match status {
                Status::UNDEF => {
                    if diff.abs() <= 3 && diff > 0 {
                        status = Status::DECR;
                    } else if diff.abs() <= 3 && diff.abs() > 0 && diff < 0 {
                        status = Status::INCR;
                    } else {
                        status = Status::INVALID;
                        break;
                    }
                }
                Status::INCR => {
                    if diff.abs() > 3 || diff >= 0 {
                        status = Status::INVALID;
                    }
                }
                Status::DECR => {
                    if diff.abs() > 3 || diff <= 0 {
                        status = Status::INVALID;
                    }
                }
                Status::INVALID => break,
            };
            // println!(
            //     "report={:?},i={};diff={};status={:?}",
            //     report, i, diff, status
            // );
        }
        println!("report={:?},status={:?}", report, status);

        match status {
            Status::INVALID | Status::UNDEF => acc += 0,
            _ => acc += 1,
        }
    }
    acc
}
pub fn part_b(input: &str) -> i32 {
    let reports = parse_input(input);

    assert!(reports.len() > 0, "No reports provided");

    let num_safe_with_dampener = reports
        .iter()
        .map(|report| report)
        .fold(0, |acc, report| acc + is_safe_with_dampener(report));

    num_safe_with_dampener
}
fn is_safe_with_dampener(report: &Vec<i32>) -> i32 {
    for i in 0..report.len() {
        let inc = report
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(j, _)| i != *j)
            .map(|(_, val)| val)
            .collect::<Vec<i32>>()
            .windows(2)
            .all(is_safe_increasing);
        let dec = report
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(j, _)| i != *j)
            .map(|(_, val)| val)
            .collect::<Vec<i32>>()
            .windows(2)
            .all(is_safe_decreasing);

        if inc || dec {
            return 1;
        }
    }
    return 0;
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}
fn is_safe_increasing(pair: &[i32]) -> bool {
    pair[1] > pair[0] && (pair[1] - pair[0] > 0) && (pair[1] - pair[0]) < 4
}

fn is_safe_decreasing(pair: &[i32]) -> bool {
    pair[0] > pair[1] && (pair[0] - pair[1] > 0) && (pair[0] - pair[1]) < 4
}

#[test]
fn test_part_a() {
    let input = r"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    let actual = part_a(input);

    let expected = 2;

    assert!(actual == expected, "expected {}; got {}", expected, actual)
}
#[test]
fn test_part_b() {
    let input = r"
5 4 8 3 2  
1 3 2 4 5
69 72 67 65 64 61 58
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
8 6 4 4 1
1 3 6 7 9";
    let actual = part_b(input);

    let expected = 6;

    assert!(actual == expected, "expected {}; got {}", expected, actual)
}
