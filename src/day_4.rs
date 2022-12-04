#[allow(dead_code)]
pub fn count_pairs_where_one_fully_contains_the_other(
    input: impl Iterator<Item = impl Into<String>>,
) -> u32 {
    let mut count = 0;
    for line in input {
        let line = line.into();
        let (range_a, range_b) = split_on(line, ',');
        let (start_a, end_a) = parse_range(range_a);
        let (start_b, end_b) = parse_range(range_b);
        let full_overlap =
            (start_b >= start_a && end_b <= end_a) || (start_a >= start_b && end_a <= end_b);
        if full_overlap {
            count += 1;
        }
    }
    count
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
