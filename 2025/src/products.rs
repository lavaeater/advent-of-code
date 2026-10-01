pub fn is_invalid(product_id: &str) -> bool {
    let chars = product_id.chars().collect::<Vec<char>>();
    let l = chars.len();
    if l <= 1 {
        return false;
    }
    if l.is_multiple_of(2) {
        let max_window = l / 2;
        for window_size in 1..=max_window {
            let is_invalid = chars
                .chunks(window_size)
                .fold((true, None), |acc, elem| {
                    if let Some(prev) = acc.1 {
                        (acc.0 && (prev == elem), Some(elem))
                    } else {
                        (true, Some(elem))
                    }
                })
                .0;
            if is_invalid {
                return is_invalid;
            }
        }
        false
    } else {
        chars
            .iter()
            .fold((true, None), |acc, elem| {
                if let Some(prev) = acc.1 {
                    (acc.0 && (prev == elem), Some(elem))
                } else {
                    (true, Some(elem))
                }
            })
            .0
    }
}

pub fn invalid_ids_from_range(range: &str) -> Vec<i64> {
    generate_range(range)
    .iter()
    .filter(|s| is_invalid(s))
    .map(|s|s.parse::<i64>().map_or_default(|i|i))
    .collect()
}

pub fn generate_range(input: &str)-> Vec<String> {
    //Range is in format nnnn-mmmm
    let ranges = input.split("-").collect::<Vec<&str>>();
    let Some(range_start) = ranges.first() else { return Vec::new(); };
    let Some(range_end) = ranges.last() else { return Vec::new(); };
    
    println!("Input {input}");
    
    if let Ok(start) = range_start.parse::<i64>() && let Ok(end) = range_end.parse::<i64>() {
        (start..=end).map(|i|i.to_string()).collect()
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("11-22", &[11i64,22i64])]
    #[test_case("95-115", &[99i64])]
    #[test_case("1188511880-1188511890", &[1188511885i64])]
    #[test_case("222220-222224", &[222222i64])]
    #[test_case("1698522-1698528", &[])]
    fn correct_results(range: &str, expected: &[i64]) {
        let actual = invalid_ids_from_range(range);
        assert_eq!(actual, expected);
    }
    
    #[test_case("22")]
    #[test_case("11")]
    #[test_case("2121")]
    #[test_case("211211")]
    #[test_case("202020")]
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
