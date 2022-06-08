use std::path::Path;

/// 测试14 AsRef

#[allow(dead_code)]
enum Language {
    Rust,
    TypeScript,
    Elixir,
    Haskell,
}

impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        match self {
            Language::Rust => "Rust",
            Language::TypeScript => "TypeScript",
            Language::Elixir => "Elixir",
            Language::Haskell => "Haskell",
        }
    }
}

fn print_ref(v: impl AsRef<str>) {
    println!("{}", v.as_ref());
}

fn main() {
    let lang = Language::Rust;
    // let s1 = "rust";
    // let s2: &str = s1.as_ref();
    // &str 实现了 AsRef<str>
    print_ref("Hello world!");
    // String 实现了 AsRef<str>
    print_ref("Hello world!".to_string());
    // 我们自己定义的 enum 也实现了 AsRef<str>
    print_ref(lang);

    let s3 = "s3: Hello world!".to_string();
    print_ref(&s3);

    let s4: &str = s3.as_ref();
    print_ref(s4);
    assert_eq!(&s3, s4);

    let s4ref: &str = s4.as_ref();
    print_ref(s4ref);

    let s5 = "s5 str type";
    println!("{}", s5);
    // print_ref(s5);

}