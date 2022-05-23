fn main() {
    let mut a = vec![1, 2, 3];
    let ref mut b = a;
    let ref b3 = b;

    println!("b3.0 is {}", b3[0]);

    // b2[0] = 4;
    //b3[0] = 3;
    // let b2 = &mut a;
    // a[0] = 2;


    println!("a {:?}", a);

    // assert_eq!(a, vec![0, 2, 3]);
}