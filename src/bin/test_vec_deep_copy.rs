fn main() {
    let a = vec![
        vec![1],
    ];

    let mut b = a.clone();
    b[0][0] = 2;
    println!("a: {:?}", a);
    println!("a addr {:p}, a[0] addr {:p}", &a, &a[0]);

    println!("b: {:?}", b);
    println!("b addr {:p}, b[0] addr {:p}", &b, &b[0]);
}

mod test {
    #[test]
    fn test_for() {
        let a = vec![
            vec![7,8,4, 1,5,9, 3,2,6],
            vec![5,3,9, 6,7,2, 8,4,1],
            vec![6,1,2, 4,3,8, 7,5,9],

            vec![9,2,8, 7,1,5, 4,6,3],
            vec![3,5,7, 8,4,6, 1,9,2],
            vec![4,6,1, 9,2,3, 5,8,7],
            
            vec![8,7,6, 3,9,4, 2,1,5],
            vec![2,4,3, 5,6,1, 9,7,8],
            vec![1,9,5, 2,8,7, 6,3,4]
        ];

        let mut lxline = vec![];
        let mut rxline = vec![];

        let w = 9;
        for wi in 0..w {
            lxline.push(a[wi][wi]);
            let hm= w - 1 - wi;
            rxline.push(a[wi][hm]);
        }

        println!("lxline :{:?}", lxline);
        println!("rxline :{:?}", rxline);

    }
}