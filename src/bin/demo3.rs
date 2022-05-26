use std::rc::Rc;

fn main() {
    // let arr = vec![1];
    let arr = 1;
    //转移所有权需要显式使用move
    std::thread::spawn(move || {
        println!("{:?}", arr);
    });
}