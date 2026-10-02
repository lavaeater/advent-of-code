pub fn is_invalid(product_id: &str) -> bool {
    let chars = product_id.chars().collect::<Vec<char>>();
    let l = chars.len();
    if l <= 1 {
        return false;
    }

    let max_window = l / 2;
    for window_size in (1..=max_window) {
        let repeating = chars
            .chunks(window_size)
            .fold((true, None), |acc, elem| {
                if let Some(prev) = acc.1 {
                    (acc.0 && (prev == elem), Some(elem))
                } else {
                    (true, Some(elem))
                }
            })
            .0;
        if repeating {
            return true;
        }
    }
    false
}

pub fn invalid_ids_from_range(range: &str) -> Vec<i64> {
    generate_range(range)
        .iter()
        .filter(|s| is_invalid(s))
        .map(|s| s.parse::<i64>().map_or_default(|i| i))
        .collect()
}

pub fn generate_range(input: &str) -> Vec<String> {
    //Range is in format nnnn-mmmm
    let ranges = input.split("-").collect::<Vec<&str>>();
    let Some(range_start) = ranges.first() else {
        return Vec::new();
    };
    let Some(range_end) = ranges.last() else {
        return Vec::new();
    };

    println!("Input {input}");

    if let Ok(start) = range_start.parse::<i64>()
        && let Ok(end) = range_end.parse::<i64>()
    {
        (start..=end).map(|i| i.to_string()).collect()
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("11-22", &[11, 22])]
    #[test_case("95-115", &[99, 111])]
    #[test_case("998-1012", &[999, 1010])]
    #[test_case("1188511880-1188511890", &[1188511885])]
    #[test_case("222220-222224", &[222222])]
    #[test_case("1698522-1698528", &[])]
    #[test_case("446443-446449", &[446446])]
    #[test_case("38593856-38593862", &[38593859])]
    #[test_case("565653-565659", &[565656])]
    #[test_case("824824821-824824827", &[824824824])]
    #[test_case("2121212118-2121212124", &[2121212121])]
    fn correct_results(range: &str, expected: &[i64]) {
        let actual = invalid_ids_from_range(range);
        assert_eq!(actual, expected);
    }

    #[test_case("22")]
    #[test_case("11")]
    #[test_case("2121")]
    #[test_case("211211")]
    #[test_case("202202")]
    fn repeating_ids_returns_true(product_id: &str) {
        assert!(is_invalid(product_id));
    }

    #[test_case("1")]
    #[test_case("21")]
    #[test_case("2322")]
    #[test_case("212020")]
    fn non_repeating_ids_returns_false(product_id: &str) {
        assert!(!is_invalid(product_id));
    }
}
