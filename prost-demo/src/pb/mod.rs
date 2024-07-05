mod user;
pub use user::*;
impl User {
    pub fn new(username: String,age:i32)->Self{
        Self{
            age,
            name: username
        }
    }
}