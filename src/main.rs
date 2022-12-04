fn main() {
    println!("Hello, world!");
    dbg!(is_valid(String::from("({[]})"))); // should be true
    dbg!(is_valid(String::from("([)]"))); // should be false
    dbg!(is_valid(String::from("([)]"))); // should be false
}

mod test {
    use super::is_valid;
    #[test]
    fn test_is_valid() {
        assert_eq!(is_valid(String::from("")), true);
        assert_eq!(is_valid(String::from("({[]})")), true);
        assert_eq!(is_valid(String::from("([)]")), false);
        assert_eq!(is_valid(String::from("([{")), false);
        assert_eq!(is_valid(String::from("(")), false);
    }
}

pub fn is_valid(string: String) -> bool {
    let mut stack: Vec<char> = vec![];
    for char in string.chars() {
        let is_open = is_open(char);
        if is_open {
            stack.push(char);
        } else {
            let is_valid = does_match(stack.pop(), char);
            if !is_valid {
                return false;
            }
        }
    }
    stack.is_empty()
}

fn is_open(char: char) -> bool {
    match char {
        '{' | '[' | '(' => true,
        _ => false,
    }
}

fn does_match(opening: Option<char>, closing: char) -> bool {
    match (opening, closing) {
        (Some('('), ')') => true,
        (Some('['), ']') => true,
        (Some('{'), '}') => true,
        _ => false,
    }
}