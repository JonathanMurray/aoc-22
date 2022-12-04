#[allow(dead_code)]
pub fn count_pairs_where_one_fully_contains_the_other(
    input: impl Iterator<Item = impl Into<String>>,
) -> u32 {
    let mut count = 0;
    for line in input {
        let (a, b) = parse_line(line);
        let full_overlap = (b.0 >= a.0 && b.1 <= a.1) || (a.0 >= b.0 && a.1 <= b.1);
        if full_overlap {
            count += 1;
        }
    }
    count
}

#[allow(dead_code)]
pub fn count_overlapping_pairs(input: impl Iterator<Item = impl Into<String>>) -> u32 {
    let mut count = 0;
    for line in input {
        let (a, b) = parse_line(line);
        let overlap = b.1 >= a.0 && b.0 <= a.1;
        if overlap {
            count += 1;
        }
    }
    count
}

fn parse_line(line: impl Into<String>) -> ((u32, u32), (u32, u32)) {
    let (range_a, range_b) = split_on(line.into(), ',');
    (parse_range(range_a), parse_range(range_b))
}

fn parse_range(range: String) -> (u32, u32) {
    let (start, end) = split_on(range, '-');
    let start = start.parse().expect("Malformed range start");
    let end = end.parse().expect("Malformed range end");
    (start, end)
}

fn split_on(line: String, delimiter: char) -> (String, String) {
    if let [a, b] = line.split(delimiter).collect::<Vec<&str>>()[..] {
        (a.to_owned(), b.to_owned())
    } else {
        panic!("Delimiter not found in line")
    }
}
