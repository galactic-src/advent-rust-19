pub struct PasswordProps {
    six_digit: bool,
    adjacent_pair: bool,
    monotonic: bool
}

impl PasswordProps {
    fn create(pw: u32) -> PasswordProps {
        let six_digit = PasswordProps::is_six_digit(&pw);
        let adjacent_pair = PasswordProps::has_adjacent_pair(&pw);
        let monotonic = PasswordProps::is_monotonic(&pw);

        PasswordProps { six_digit, adjacent_pair, monotonic }
    }

    fn create2(pw: u32) -> PasswordProps {
        let six_digit = PasswordProps::is_six_digit(&pw);
        let adjacent_pair = PasswordProps::has_adjacent_pair_2(&pw);
        let monotonic = PasswordProps::is_monotonic(&pw);

        PasswordProps { six_digit, adjacent_pair, monotonic }
    }

    fn is_six_digit(pw: &u32) -> bool {
        PasswordProps::split_chars(pw).iter().count() == 6
    }

    fn has_adjacent_pair(pw: &u32) -> bool {
        let cs = PasswordProps::split_chars(pw);
        let mut cs2 = cs.to_vec();
        cs2.insert(0, '#');
        cs.iter().zip(cs2).any(|(c1,c2)| *c1 == c2)
    }

    fn has_adjacent_pair_2(pw: &u32) -> bool {
        let cs = PasswordProps::split_chars(pw);
        let mut tally = 0u32;
        let mut current_char = '#';

        for c in cs {
            if c == current_char {
                tally += 1;
            } else if tally == 2 {
                return true;
            } else {
                tally = 1;
                current_char = c;
            }
        }

        tally == 2
    }

    fn split(pw: &u32) -> Vec<u32> {
        PasswordProps::split_chars(pw).into_iter().map(|c|c.to_digit(10).expect("could not parse to digit")).collect()
    }

    fn split_chars(pw: &u32) -> Vec<char> {
        pw.to_string().chars().collect()
    }

    fn is_monotonic(pw: &u32) -> bool {
        let mut current = 0;

        for i in PasswordProps::split(&pw) {
            if i < current { return false }
            current = i;
        }

        true
    }

    fn valid(&self) -> bool {
        self.six_digit && self.adjacent_pair && self.monotonic
    }
}

fn main() {
    let low = 123257u32;
    let high = 647015u32;

    let valid_count_1 =
        (low..(high+1))
            .map(|int|PasswordProps::create(int))
            .filter(|pp| pp.valid())
            .count();

    println!("part 1: {}", valid_count_1);

    let valid_count_2 =
        (low..(high+1))
            .map(|int|PasswordProps::create2(int))
            .filter(|pp| pp.valid())
            .count();

    println!("part 2: {}", valid_count_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_chars() {
        assert_eq!(PasswordProps::split_chars(&123), vec!('1','2','3'));
    }

    #[test]
    fn test_split() {
        assert_eq!(PasswordProps::split(&123), vec!(1,2,3));
    }

    #[test]
    fn test_has_adjacent_pair() {
        assert_eq!(PasswordProps::has_adjacent_pair(&1123), true);
    }

    #[test]
    fn test_has_adjacent_pair_2() {
        assert_eq!(PasswordProps::has_adjacent_pair(&1233), true);
    }

    #[test]
    fn test_has_adjacent_pair_3() {
        assert_eq!(PasswordProps::has_adjacent_pair(&1234), false);
    }

    #[test]
    fn test_monotonic() {
        assert_eq!(PasswordProps::is_monotonic(&1234), true);
    }

    #[test]
    fn test_monotonic_2() {
        assert_eq!(PasswordProps::is_monotonic(&1243), false);
    }

}