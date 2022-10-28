use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize, Default)]
struct Claims {
    user_id: i64,
    exp: usize,
}

static SECRET: &str = "secret";

pub fn get_token() -> String {
    let claims = Claims { user_id: 1, exp: 0 };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET.as_ref()),
    )
    .unwrap();
    return token;
}

pub fn get_user_id(token: String) {
    println!("token: {}", token);
    let mut v = Validation::default();
    v.validate_exp = false; //设置不校验过期时间

    let token = decode::<Claims>(&token, &DecodingKey::from_secret(SECRET.as_ref()), &v);
    match token {
        Ok(ok) => {
            println!("正确: {:?}", ok.claims)
        }
        Err(err) => {
            println!("错误: {}", err)
        }
    };
}

pub fn test() {
    let token = get_token();
    get_user_id(token);
}
