/// 我有两个类型 都需要序列化，我希望序列化之前执行一个修改类型的字段的方法 然后再执行序列化
///

use core::fmt::Debug;

fn main() {
    let u = User {
        name: "user1".to_string(),
        ..User::default()
    };
    let a = Article {
        blog: "article1".to_string(),
        ..Article::default()
    };

    println!("u = {:?}", u);
    println!("a = {:?}", a);

    //执行打印
    print_json(u);
    print_json(a);
}

#[derive(Default, Debug)]
struct User {
    name: String,
    remark: String,
}

impl UpdateBeforePrintJson for User {
    fn update_fields(&mut self) {
        self.remark += " - update before"
    }
}

#[derive(Default, Debug)]
struct Article {
    blog: String,
    remark: String,
}

impl UpdateBeforePrintJson for Article {
    fn update_fields(&mut self) {
        self.remark += " - update before"
    }
}

trait UpdateBeforePrintJson {
    fn update_fields(&mut self);
}


fn print_json<T>(mut t: T)
where
    T: UpdateBeforePrintJson + Debug, {
    t.update_fields();
    println!("print json after update hook {:?}", t);
}
