use std::collections::HashMap;

fn main() {
    let strings = vec!["abc", "abbc", "", "  "];
    for s in strings {
        println!("Is '{}' unique? {}", s, is_unique(s));
    }
}


fn is_unique(s: &str) -> bool {
    let mut characters: HashMap<String, i32> = HashMap::new();

    for c in s.chars() {
        let count = characters.entry(c.to_string()).or_insert(0);
        *count += 1;
        
        if *count > 1 {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_a_unique_string() {
        let expected = is_unique("abc");
        assert_eq!(true, expected);
    }

    #[test]
    fn is_not_a_unique_string() {
        let expected = is_unique("abbc");
        assert_eq!(false, expected);
    }

    #[test]
    fn empty_string_is_unique() {
        let expected = is_unique("");
        assert_eq!(true, expected);
    }

    #[test]
    fn string_spaces_are_not_unique() {
        let expected = is_unique("  ");
        assert_eq!(false, expected);
    }
}
