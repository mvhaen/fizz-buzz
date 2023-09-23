fn main() {
    println!("{}", fizz_buzz(1));
}

fn fizz_buzz(n: i32) -> String {
    if n % 15 == 0 {
        return "Fizz-Buzz".to_string();
    }
    if n % 3 == 0 {
        return "Fizz".to_string();
    }
    if n % 5 == 0 {
        return "Buzz".to_string();
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
    fn it_returns_buzz_for_5() {
        assert_eq!("Buzz", fizz_buzz(5));
    }

    #[test]
    fn it_returns_fizz_for_6() {
        assert_eq!("Fizz", fizz_buzz(6));
    }

    #[test]
    fn it_returns_7_for_7() {
        assert_eq!("7", fizz_buzz(7));
    }

    // array with numbers divisible by 3 and 5 up to 100
    const FIZZ_BUZZ: [i32; 6] = [15, 30, 45, 60, 75, 90];

    // parameterized test
    #[test]
    fn it_returns_fizz_buzz_for_15_30_45_60_75_90() {
        for n in FIZZ_BUZZ.iter() {
            assert_eq!("Fizz-Buzz", fizz_buzz(*n));
        }
    }

}