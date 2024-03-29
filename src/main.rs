fn main() {
    fizz_buzz()
}

fn fizz_buzz() {
    for n in 1..100 {
        println!("{}", fizz_buzz_for_one(n));
    }
}

fn fizz_buzz_for_one(n: i32) -> String {
    // throw an error if the number is smaller than 1 or larger than 100
    if n < 1 || n > 100 {
        panic!("n must be between 0 and 100, got {}", n)
    }

    // match n against the tuple (n % 3, n % 5)
    // if n % 3 == 0 && n % 5 == 0 => "Fizz-Buzz"
    // if n % 3 == 0 => "Fizz"
    // if n % 5 == 0 => "Buzz"
    // else => n.to_string()
    return match (n % 3, n % 5) {
        (0, 0) => "Fizz-Buzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => n.to_string(),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    // array with numbers divisible by 3 up to 100 and not divisible by 5
    const FIZZ: [i32; 27] = [
        3, 6, 9, 12, 18, 21, 24, 27, 33, 36, 39, 42, 48, 51, 54, 57, 63, 66, 69, 72, 78, 81, 84, 87,
        93, 96, 99,
    ];

    #[test]
    fn it_returns_fizz_for_3_6_9_12_18_21_24_27_33_36_39_42_48_51_54_57_63_66_69_72_78_81_84_87_93_96_99() {
        for n in FIZZ.iter() {
            assert_eq!("Fizz", fizz_buzz_for_one(*n));
        }
    }

    // array with numbers divisible by 5 up to 100 and not divisible by 3
    const BUZZ: [i32; 14] = [5, 10, 20, 25, 35, 40, 50, 55, 65, 70, 80, 85, 95, 100];
    #[test]
    fn it_returns_buzz_for_5_10_20_25_35_40_50_55_65_70_80_85_95_100() {
        for n in BUZZ.iter() {
            assert_eq!("Buzz", fizz_buzz_for_one(*n));
        }
    }

    // array with numbers divisible by 3 and 5 up to 100
    const FIZZ_BUZZ: [i32; 6] = [15, 30, 45, 60, 75, 90];
    #[test]
    fn it_returns_fizz_buzz_for_15_30_45_60_75_90() {
        for n in FIZZ_BUZZ.iter() {
            assert_eq!("Fizz-Buzz", fizz_buzz_for_one(*n));
        }
    }

    #[test]
    fn it_returns_n_for_n_if_not_divisible_by_3_or_5 () {
        for n in 1..100 {
            if ! n % 3 == 0 && ! n % 5 == 0 {
                assert_eq!(n.to_string(), fizz_buzz_for_one(n));
            }
        }
    }

    #[test]
    fn it_panics_when_passing_smaller_than_1_or_larger_than_100 () {
        // write a test that checks for a panic when passing 0 to fizz_buzz
        assert!(std::panic::catch_unwind(|| fizz_buzz_for_one(0)).is_err());
        assert!(std::panic::catch_unwind(|| fizz_buzz_for_one(101)).is_err());
    }

}