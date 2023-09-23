fn main() {
    println!("{}", fizz_buzz(1));
}

fn fizz_buzz(n: i32) -> String {
        if n == 1 {
            return 1.to_string();
        } else if n == 2 {
            return 2.to_string();
        }
        "".to_string()
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