fn main() {
    println!("{}", fizz_buzz(1));
}

fn fizz_buzz(n: i32) -> String {
    n.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn it_returns_1_for_1() {
        assert_eq!("1", fizz_buzz(1));
    }

    #[test]
    fn it_returns_2_for_2() {
        assert_eq!("2", fizz_buzz(2));
    }
}