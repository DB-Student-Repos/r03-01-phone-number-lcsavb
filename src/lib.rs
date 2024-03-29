
pub fn number(user_number: &str) -> Option<String> {
    let sanitized_number: String = user_number.chars().filter(|c| c.is_digit(10)).collect();

    match sanitized_number.len() {
        11 if sanitized_number.starts_with("1") => {
            Some(sanitized_number[1..].to_string())
            .filter(|s| check_nth_digits(s))
        }
        10 => {
            Some(sanitized_number)
            .filter(|s| check_nth_digits(s))
        }
        _ => None,
    }
}

fn check_nth_digits(slice: &str) -> bool {
    slice.chars().nth(0).unwrap() >= '2' && slice.chars().nth(3).unwrap() >= '2'
}


