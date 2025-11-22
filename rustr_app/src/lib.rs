/// Library core for rustr_app

/// Return greeting message for optional name
pub fn greet(name: Option<&str>) -> String {
    match name {
        Some(n) => format!("Hello, {}!", n),
        None => "Hello, world!".to_string(),
    }
}

/// Add two integers and return result
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_none() {
        assert_eq!(greet(None), "Hello, world!");
    }

    #[test]
    fn test_greet_some() {
        assert_eq!(greet(Some("Alice")), "Hello, Alice!");
    }

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
