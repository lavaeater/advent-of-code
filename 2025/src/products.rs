

pub fn is_repeating(product_id: &str)->bool {
    let l = product_id.len();
    
    if l % 2 != 0 {false} else {
        let mut is_repeating = false;
        let max_sublength = l / 2;
        for sl in 1..=max_sublength {
            let number_of_parts = l / sl; //str of l 2 has 2 1 char parts
            if let Some(first_part) = product_id.get(0..sl) {
                for pi in 1..number_of_parts {
                    if let Some(next_part) = product_id.get(pi*sl..pi*sl+sl) {
                        
                    }
                }  
            }
            if is_repeating {
                return is_repeating;
            }
        }
        false
    }
}