///陈天rust 08 思考题
fn main() {
    // println!("hello rust!");
    let mut arr = vec![1, 2, 3];

    //cache last item
    //这里持有的是对数组成员的一个不可变引用！与随后的arr.push中的arr可变引用冲突
    // let last = arr.last();

    //这里分为两步 通过取数组的长度 这个值不存在引用，通过索引访问数组元素的值 这个也不存在引用！所以可行!
    let last = arr[arr.len() - 1];
    arr.push(4);
    println!("last: {:?}", last);
}