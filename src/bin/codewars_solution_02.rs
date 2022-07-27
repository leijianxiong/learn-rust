fn main() { }

struct Sudoku {
    data: Vec<Vec<u32>>,
}

impl Sudoku {
    //数独
    //横竖个数相同
    //横竖宫线的数字都不同 
    fn is_valid(&self) -> bool {
        // YOUR SOLUTION
        let h = self.data.len();
        if h == 0 {
            return false;
        }
        //以第一个横着的个数为准
        let w = self.data[0].len();
        //检查横竖个数相等
        if w != h {
            return false;
        }

        //检查每个横的个数都与第一个横着的个数相同
        for v in self.data.iter() {
            if v.len() != w {
                return false;
            }
        }

        //检查横竖宫线的都不同 n的话数组有 2n*2(横+竖+宫)
        //取所有线
        //取横线
        let mut allline = self.data.clone();

        //取竖线
        let mut allvline = Vec::new();
        for i in 0..h {
            let mut vline = vec![];    
            for v in self.data.iter() {
                vline.push(v[i]);
            }
            allvline.push(vline);
        }

        //取宫线
        let mut lxline = vec![];
        let mut rxline = vec![];
        for wi in 0..w {
            lxline.push(self.data[wi][wi]);
            let hm= w - 1 - wi;
            rxline.push(self.data[wi][hm]);
        }

        //合并所有线
        allline.append(&mut allvline);
        allline.push(lxline);
        allline.push(rxline);

        //排序
        allline = allline.into_iter().map(|mut line| {
            line.sort();
            line
        }).collect();

        let refline = &allline[0];
        //检查看元素都与第一个相同
        for line in allline.iter() {
            if line != refline {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fake_bin(s: &str) -> String {
        s.chars()
            .map(|c| {
                let c = c.to_string().parse::<i32>().unwrap();
                match c >= 5 {
                    true => "1",
                    false => "0",
                }
            })
            .flat_map(|c| c.chars())
            .collect::<String>()
    }
    #[test]
    fn basic_tests() {
        assert_eq!(fake_bin("45385593107843568"), "01011110001100111");
        assert_eq!(fake_bin("509321967506747"), "101000111101101");
        assert_eq!(
            fake_bin("366058562030849490134388085"),
            "011011110000101010000011011"
        );
        assert_eq!(fake_bin("15889923"), "01111100");
        assert_eq!(fake_bin("800857237867"), "100111001111");
    }

    fn is_divide_by(number: i32, a: i32, b: i32) -> bool {
        number % a == 0 && number % b == 0
    }
    #[test]
    fn basic_tests2() {
        assert_eq!(is_divide_by(8, 2, 4), true);
        assert_eq!(is_divide_by(12, -3, 4), true);
        assert_eq!(is_divide_by(8, 3, 4), false);
        assert_eq!(is_divide_by(48, 2, -5), false);
        assert_eq!(is_divide_by(-100, -25, 10), true);
        assert_eq!(is_divide_by(10000, 5, -3), false);
        assert_eq!(is_divide_by(4, 4, 2), true);
        assert_eq!(is_divide_by(5, 2, 3), false);
        assert_eq!(is_divide_by(-96, 25, 17), false);
        assert_eq!(is_divide_by(33, 1, 33), true);
    }

    fn greet(language: &str) -> &str {
        let databases = &[
            ("english", "Welcome"),
            ("czech", "Vitejte"),
            ("danish", "Velkomst"),
            ("dutch", "Welkom"),
            ("estonian", "Tere tulemast"),
            ("finnish", "Tervetuloa"),
            ("flemish", "Welgekomen"),
            ("french", "Bienvenue"),
            ("german", "Willkommen"),
            ("irish", "Failte"),
            ("italian", "Benvenuto"),
            ("latvian", "Gaidits"),
            ("lithuanian", "Laukiamas"),
            ("polish", "Witamy"),
            ("spanish", "Bienvenido"),
            ("swedish", "Valkommen"),
            ("welsh", "Croeso"),
        ][..];
        for record in databases.iter() {
            if record.0 == language {
                return record.1;
            }
        }
        databases[0].1
    }
    #[test]
    fn test_fixed() {
        assert_eq!(greet("english"), "Welcome");
        assert_eq!(greet("dutch"), "Welkom");
        assert_eq!(greet("IP_ADDRESS_INVALID"), "Welcome");
        assert_eq!(greet(""), "Welcome");
        assert_eq!(greet("swelsh"), "Welcome");
    }

    fn index(nums: &[u64], n: usize) -> Option<u64> {
        for (i, num) in nums.iter().enumerate() {
            if n == i {
                return Some(num.pow(i as u32));
            }
        }
        None
    }
    #[test]
    fn sample_tests() {
        assert_eq!(
            index(&[1, 2, 3, 4], 2),
            Some(9),
            "Failed on the first sample test"
        );
        assert_eq!(
            index(&[1, 3, 10, 100], 3),
            Some(1000000),
            "Failed on the second sample test"
        );
        assert_eq!(
            index(&[1, 2, 3, 4], 69),
            None,
            "Failed on the third sample test"
        );
    }

    fn expressions_matter(a: u64, b: u64, c: u64) -> u64 {
        let methods = &[
            a + b + c,
            a * b * c,
            a + b * c,
            a * b + c,
            (a + b) * c,
            a * (b + c),
        ][..];
        methods.iter().max().map(|u| *u).unwrap()
    }
    #[test]
    fn basic_tests3() {
        assert_eq!(expressions_matter(2, 1, 2), 6);
        assert_eq!(expressions_matter(1, 1, 1), 3);
        assert_eq!(expressions_matter(2, 1, 1), 4);
        assert_eq!(expressions_matter(1, 2, 3), 9);
        assert_eq!(expressions_matter(1, 3, 1), 5);
        assert_eq!(expressions_matter(2, 2, 2), 8);

        assert_eq!(expressions_matter(5, 1, 3), 20);
        assert_eq!(expressions_matter(3, 5, 7), 105);
        assert_eq!(expressions_matter(5, 6, 1), 35);
        assert_eq!(expressions_matter(1, 6, 1), 8);
        assert_eq!(expressions_matter(2, 6, 1), 14);
        assert_eq!(expressions_matter(6, 7, 1), 48);

        assert_eq!(expressions_matter(2, 10, 3), 60);
        assert_eq!(expressions_matter(1, 8, 3), 27);
        assert_eq!(expressions_matter(9, 7, 2), 126);
        assert_eq!(expressions_matter(1, 1, 10), 20);
        assert_eq!(expressions_matter(9, 1, 1), 18);
        assert_eq!(expressions_matter(10, 5, 6), 300);
        assert_eq!(expressions_matter(1, 10, 1), 12);
    }

    fn gimme(input_array: [i32; 3]) -> usize {
        let mut enum_arr = input_array.iter().enumerate().collect::<Vec<_>>();
        enum_arr.sort_by(|e1, e2| e1.1.cmp(e2.1));
        // println!("enum arr sorted {:?}", enum_arr);
        enum_arr[1].0
    }
    #[test]
    fn test_gimme() {
        assert_eq!(gimme([2, 3, 1]), 0);
        assert_eq!(gimme([-2, -3, -1]), 0);
        assert_eq!(gimme([5, 10, 14]), 1);
    }

    fn nb_dig(n: i32, d: i32) -> i32 {
        let d = d.to_string();
        let res = (0..=n)
            .map(|v| v * v)
            .flat_map(|v| {
                v.to_string()
                    .chars()
                    .filter(|c| c.to_string() == d)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        res.len() as i32
    }
    fn dotest(n: i32, d: i32, exp: i32) -> () {
        println!("n: {:?}", n);
        println!("d: {:?}", d);
        let ans = nb_dig(n, d);
        println!("actual:\n{:?}", ans);
        println!("expect:\n{:?}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basic_tests4() {
        dotest(550, 5, 213);
        dotest(5750, 0, 4700);
    }

    fn how_much_i_love_you(nb_petals: u16) -> &'static str {
        let flowers = &[
            "I love you",
            "a little",
            "a lot",
            "passionately",
            "madly",
            "not at all",
        ][..];

        let pos = (nb_petals - 1) % (flowers.len() as u16);
        let res = flowers.get(pos as usize).unwrap();
        return res;
    }
    #[test]
    fn fixed_tests() {
        assert_eq!(how_much_i_love_you(7), "I love you");
        assert_eq!(how_much_i_love_you(3), "a lot");
        assert_eq!(how_much_i_love_you(6), "not at all");
    }

    fn get_sum(a: i64, b: i64) -> i64 {
        let c;
        if a < b {
            c = a..=b;
        } else {
            c = b..=a;
        }
        c.into_iter().sum()
    }
    #[test]
    fn sample_tests_for_get_sum() {
        assert_eq!(get_sum(0, 1), 1);
        assert_eq!(get_sum(1, 2), 3);
        assert_eq!(get_sum(5, -1), 14);
        assert_eq!(get_sum(505, 4), 127759);
    }

    fn int32_to_ip(int: u32) -> String {
        let mut int_binary = format!("{int:b}");
        if int_binary.len() < 32 {
            int_binary = "0".repeat(32 - int_binary.len()) + int_binary.as_str();
        }

        println!("int_binary {}", int_binary);
        let vec = int_binary
            .chars()
            .collect::<Vec<_>>()
            .chunks(8)
            .map(|chars| chars.iter().collect::<String>())
            .map(|ev_int_binary| {
                isize::from_str_radix(ev_int_binary.as_str(), 2)
                    .unwrap()
                    .to_string()
            })
            .collect::<Vec<_>>();

        vec.join(".")
    }
    #[test]
    fn basic() {
        assert_eq!(int32_to_ip(2154959208), "128.114.17.104");
        assert_eq!(int32_to_ip(2149583361), "128.32.10.1");
        assert_eq!(int32_to_ip(0), "0.0.0.0");
        assert_eq!(int32_to_ip(3), "0.0.0.3");
    }

    #[test]
    fn good_sudoku() {
        let good_sudoku_1 = Sudoku {
            data: vec![
                vec![7, 8, 4, 1, 5, 9, 3, 2, 6],
                vec![5, 3, 9, 6, 7, 2, 8, 4, 1],
                vec![6, 1, 2, 4, 3, 8, 7, 5, 9],
                vec![9, 2, 8, 7, 1, 5, 4, 6, 3],
                vec![3, 5, 7, 8, 4, 6, 1, 9, 2],
                vec![4, 6, 1, 9, 2, 3, 5, 8, 7],
                vec![8, 7, 6, 3, 9, 4, 2, 1, 5],
                vec![2, 4, 3, 5, 6, 1, 9, 7, 8],
                vec![1, 9, 5, 2, 8, 7, 6, 3, 4],
            ],
        };

        let good_sudoku_2 = Sudoku {
            data: vec![
                vec![1, 4, 2, 3],
                vec![3, 2, 4, 1],
                vec![4, 1, 3, 2],
                vec![2, 3, 1, 4],
            ],
        };
        assert!(good_sudoku_1.is_valid());
        assert!(good_sudoku_2.is_valid());
    }

    #[test]
    fn bad_sudoku() {
        let bad_sudoku_1 = Sudoku {
            data: vec![
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            ],
        };

        let bad_sudoku_2 = Sudoku {
            data: vec![
                vec![1, 2, 3, 4, 5],
                vec![1, 2, 3, 4],
                vec![1, 2, 3, 4],
                vec![1],
            ],
        };
        assert!(!bad_sudoku_1.is_valid());
        assert!(!bad_sudoku_2.is_valid());
    }
}
