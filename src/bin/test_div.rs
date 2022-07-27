//指定数的除数集合

fn divs(n: i32) -> Result<Vec<i32>, String> {
    let mut res = vec![];
    for i in 2..(n-1) {
        if n % i == 0 {
            res.push(i);
        }
    }
    if res.len() == 0 {
        return Err(format!("{} is prime", n));
    }
    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_divs_should_work() {
        assert_eq!(divs(12), Ok(vec![2,3,4,6]));
        assert_eq!(divs(25), Ok(vec![5]));
        assert_eq!(divs(13), Err("13 is prime".to_string()));
    }
}