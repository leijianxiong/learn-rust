///web 有很多api，每个api的入口的action都有参数校验 逻辑处理 参数返回
/// 我希望把action提出来成一个共用的 每个具体的api都只需要处理自己的handle即可
/// 参考迭代器，自定义类型只需要实现一个方法nex即可

use std::fmt::Debug;

fn main() {
    route_register("/api/get-user".to_string(), GetUser::default())
}

//模拟路由注册
fn route_register<A>(path: String, action: A) 
where A: Action, {
    println!("register path {} use action {:?}", path, action);
    let resp = action.handle();
    println!("got resp {:?}", resp)
}

trait Action: Debug {
    fn action(&self){
        println!("call self handle");
        self.handle();
    }

    type Resp: Debug;
    fn handle(&self) -> Self::Resp;
}

#[derive(Default, Debug)]
struct GetUser {

}

#[derive(Default, Debug)]
struct GetUserResp{
    Id: i32,
    Name: String,
}

impl Action for GetUser {
    type Resp = GetUserResp;
    fn handle(&self) -> Self::Resp {
        println!("call get-user handle");
        GetUserResp { Id: 1, Name: "user".to_string()}
    }
}