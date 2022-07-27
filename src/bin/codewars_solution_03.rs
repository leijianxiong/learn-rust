fn main() {}

fn rgb(r: i32, g: i32, b: i32) -> String {
    let r = std::cmp::max(0, std::cmp::min(r, 255));
    let g = std::cmp::max(0, std::cmp::min(g, 255));
    let b = std::cmp::max(0, std::cmp::min(b, 255));
    format!("{:02X}{:02X}{:02X}", r, g, b)
}

macro_rules! compare {
    ( $got : expr, $expected : expr ) => {
        if $got != $expected {
            panic!("Got: {}\nExpected: {}\n", $got, $expected);
        }
    };
}

#[cfg(test)]
mod sample_tests {
    use self::super::*;

    #[test]
    fn tests() {
        compare!(rgb(0, 0, 0), "000000");
        compare!(rgb(1, 2, 3), "010203");
        compare!(rgb(255, 255, 255), "FFFFFF");
        compare!(rgb(254, 253, 252), "FEFDFC");
        compare!(rgb(-20, 275, 125), "00FF7D");
    }
}