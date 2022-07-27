#![allow(unused)]
fn main() {
    let c = C {};
    c.func_in_a();
    c.func_in_b();
}

trait B {
    fn func_in_b(&self);
}

// Trait A继承Trait B
trait A: B {
    fn func_in_a(&self);
}

struct C {}
// C实现Trait A
impl A for C {
    fn func_in_a(&self) {
        println!("impl: func_in_a");
        println!("call self.func_in_b");
        self.func_in_b()
    }
}
// C还要实现Trait B
impl B for C {
    fn func_in_b(&self) {
        println!("impl: func_in_b");
    }
}
