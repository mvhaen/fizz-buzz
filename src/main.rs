fn main() {
    println!("{}", fizz_buzz(1));
}

fn fizz_buzz(n: i32) -> String {
    if n % 3 == 0 {
        return "Fizz".to_string();
    } else if n % 5 == 0 {
        return "buzz".to_string();
    }
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

    #[test]
    fn it_returns_fizz_for_3() {
        assert_eq!("Fizz", fizz_buzz(3));
    }

    #[test]
    fn it_returns_4_for_4() {
        assert_eq!("4", fizz_buzz(4));
    }

    #[test]
    fn it_returns_fizz_for_5() {
        assert_eq!("buzz", fizz_buzz(5));
    }
}