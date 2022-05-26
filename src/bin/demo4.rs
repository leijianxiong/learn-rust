/**
陈天 rust第一课 12思考题
*/
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
struct MyWriter<W> {
    writer: W,
}

impl<W: Write> MyWriter<W> {
    //使用依赖注入
    pub fn new_by_writer(w: W) -> Self {
        Self { writer: w }
    }

    pub fn write(&mut self, buf: &str) -> std::io::Result<()> {
        self.writer.write_all(buf.as_bytes())
    }
}

//为具体的类型实现new
impl MyWriter<File> {
    pub fn new(path: &str) -> Self {
        //稳定版的rust使用不了不稳定版的功能 后续看如何处理
        // let file = File::with_options().read(true).write(true).open(path).unwrap();
        let file = File::create(path).unwrap();
        Self::new_by_writer(file)
    }
}

fn main() {
    // let stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    // let mut writer = MyWriter::new(BufWriter::new(stream));

    // let file = File::create("/tmp/test1").unwrap();
    let mut writer = MyWriter::<File>::new("/tmp/test1");

    let res = writer.write("hello world2!\n");
    println!("write res: {:?}", res);
}
