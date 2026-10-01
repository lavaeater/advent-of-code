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

pub fn is_invalid_old(product_id: &str) -> bool {
    let l = product_id.len();
    println!("Is {product_id} repeating? Let's see!");

    if l % 2 != 0 {
        false
    } else {
        let max_sublength = l / 2;
        for sl in 1..=max_sublength {
            let number_of_parts = l / sl; //str of l 2 has 2 1 char parts
            if let Some(first_part) = product_id.get(0..sl) {
                let mut is_invalid = true;
                for pi in 1..number_of_parts {
                    if let Some(next_part) = product_id.get(pi * sl..pi * sl + sl) {
                        println!("Comparing {first_part} with {next_part}");
                        is_invalid = is_invalid && first_part.eq(next_part);
                    }
                }
                if is_invalid {
                    return is_invalid;
                }
            }
        }
        false
    }
}

fn generate_range(input: &str)-> Result<Vec<String>, &str> {
    //Range is in format nnnn-mmmm
    let ranges = input.split("-").collect::<Vec<&str>>();
    let Some(range_start) = ranges.first() else { return Err("Should have a first!"); };
    let Some(range_end) = ranges.last() else { return Err("Should have a last!"); };
    
    if let Ok(start) = range_start.parse::<i32>() && let Ok(end) = range_end.parse::<i32>() {
        Ok((start..=end).map(|i|i.to_string()).collect())
    } else {
        Err("Could not parse to i32")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("22")]
    #[test_case("2222")]
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
