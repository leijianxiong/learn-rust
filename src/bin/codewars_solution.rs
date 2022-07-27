fn main() {}

#[cfg(test)]
mod tests {
    fn string_to_array(s: &str) -> Vec<String> {
        s.split(" ").into_iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn string_to_array_should_work() {
        assert_eq!(string_to_array("hello world!"), vec!["hello", "world!"]);
    }

    fn number(bus_stops: &[(i32, i32)]) -> i32 {
        bus_stops.iter().map(|users| users.0 - users.1).sum()
    }

    #[test]
    fn returns_expected() {
        assert_eq!(number(&[(10, 0), (3, 5), (5, 8)]), 5);
        assert_eq!(
            number(&[(3, 0), (9, 1), (4, 10), (12, 2), (6, 1), (7, 10)]),
            17
        );
        assert_eq!(
            number(&[(3, 0), (9, 1), (4, 8), (12, 2), (6, 1), (7, 8)]),
            21
        );
    }

    fn longest(a1: &str, a2: &str) -> String {
        let a3 = a1.to_string() + a2;
        let mut uni_chars = Vec::new();
        for c in a3.chars().into_iter() {
            if !uni_chars.contains(&c.to_string()) {
                uni_chars.push(c.to_string());
            }
        }
        uni_chars.sort();
        uni_chars.join("").to_string()
    }

    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        println!("s1:{:?} s2:{:?}", s1, s2);
        println!("{:?} {:?}", longest(s1, s2), exp);
        println!("{}", longest(s1, s2) == exp);
        assert_eq!(&longest(s1, s2), exp)
    }

    #[test]
    fn basic_tests() {
        testing("aretheyhere", "yestheyarehere", "aehrsty");
        testing(
            "loopingisfunbutdangerous",
            "lessdangerousthancoding",
            "abcdefghilnoprstu",
        );
    }

    fn get_count(string: &str) -> usize {
        let vowels = "aeiou".chars().collect::<Vec<_>>();
        string.chars().filter(|c| vowels.contains(c)).count()
    }

    #[test]
    fn my_tests() {
        assert_eq!(get_count("abracadabra"), 5);
    }

    fn parse(code: &str) -> Vec<i32> {
        let mut current: i32 = 0;
        let mut output = Vec::new();
        for c in code.chars() {
            match c {
                'i' => current += 1,
                'd' => current -= 1,
                's' => current *= current,
                'o' => output.push(current),
                _ => continue,
            }
        }
        output
    }

    #[test]
    fn sample_tests() {
        assert_eq!(parse("iiisdoso"), vec![8, 64]);
        assert_eq!(parse("iiisdosodddddiso"), vec![8, 64, 3600]);
    }

    fn remove_every_other(arr: &[u8]) -> Vec<u8> {
        arr.iter()
            .enumerate()
            .filter(|(i, _n)| i % 2 == 0)
            .map(|(_i, n)| *n)
            .collect::<Vec<u8>>()
    }
    #[test]
    fn sample_test() {
        assert_eq!(
            remove_every_other(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
            &[1, 3, 5, 7, 9]
        );
    }
}
