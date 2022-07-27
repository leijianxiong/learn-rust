/// 测试vec的类型

fn main() {
    let arr = vec![1, 2, 3, 4];
    //会调用arr对应类型的 as_ref 转成数组切片引用
    //let arr2: &[i32] = &arr;
    let arr2: &[i32] = &arr;
    assert_ne!(arr, arr2);
}