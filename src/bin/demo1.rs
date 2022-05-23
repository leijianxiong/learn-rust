fn main() {
    struct Point { x: i32, y: i32 }

    let Point { x, y } = Point{ x: 1, y: 2 };

    assert_eq!(1, x);
    assert_eq!(1, y);
}