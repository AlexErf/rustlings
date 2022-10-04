// strings3.rs
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a hint.


fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let mut counter = 0;
    let startIndex = loop {
        if input.as_bytes()[counter] != b' ' || counter == input.len() {
            break counter;
        }
        counter += 1;
    };
    counter = input.len() - 1;
    let endIndex = loop {
        if input.as_bytes()[counter] != b' ' || counter == 0 {
            break counter;
        }
        counter -= 1;
    };
    input[startIndex..endIndex + 1].to_string()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    format!("{} world!", input)
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let idx = input.find("cars");
    match idx {
        None => input.to_string(),
        Some(i) => format!("{}balloons{}", &input.to_string()[0..i], &input.to_string()[i + 4..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
