use prost_demo::pb::User;
fn main() {
    println!("Hello, world!");
    let user = User::new("tao.chen".to_string(), 32);
    println!("{:?}",user);
}
