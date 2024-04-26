
use serde::{Deserialize, Serialize};
use my_macro::GettersForFieldNames;

fn main() {
    let user = User{
        name: None,
        age: Some(12),
        email: Some("200@qq.com".to_string()),
    };

    println!("{:?}", user);
    println!("{}", user.get_name());
    println!("{}", user.get_age());
    println!("{}", user.get_email());
}


#[derive(GettersForFieldNames,Serialize, Deserialize, Debug)]
pub struct User{
    pub name: Option<String>,
    pub age: Option<u32>,
    pub email: Option<String>,
}


