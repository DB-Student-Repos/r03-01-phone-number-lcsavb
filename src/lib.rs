pub fn number(user_number: &str) -> Option<String> {
    let number = user_number.trim();

    let mut sanitized_number = String::new(); 
    
    for c in number.chars() {
        if c.is_digit(10) {
            sanitized_number.push(c);
        }
    } 
    
    if sanitized_number.len() == 10 &&
       sanitized_number.chars().nth(0).unwrap() >= '2' &&
       sanitized_number.chars().nth(3).unwrap() >= '2' {
           Some(sanitized_number)
        } else {
            None
        } 
}
