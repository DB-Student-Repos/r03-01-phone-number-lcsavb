

pub fn number(user_number: &str) -> Option<String> {
    let sanitized_number: String = user_number.chars().filter(|c| c.is_digit(10)).collect();

    match sanitized_number {

        n if n.len() == 11 && n.starts_with('1') && 
        n.chars().nth(1).unwrap() >= '2' && 
        n.chars().nth(4).unwrap() >= '2' 
        => Some(n[1..].to_string()),

        n if n.len() == 10 && 
        n.chars().nth(0).unwrap() >= '2' &&
        n.chars().nth(3).unwrap() >= '2' => Some(n),

        _ => None
    }
}

