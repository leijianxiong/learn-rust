fn main() {
    // println!("Hello, world!");
    let list1 = vec![1, 2, 3, 4, 5];
    let res = find_largest(&list1);
    println!("list1 find largest res is {}", res);

    let list2 = vec![6, 7, 8, 9, 10];
    let res = find_largest1(&list2);
    println!("list2 find largest1 res is {}", res);
}

fn find_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn find_largest1<T: PartialOrd + Clone>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > &largest {
            largest = item;
        }
    }

    largest 
}