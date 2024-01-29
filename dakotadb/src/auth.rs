use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation}
use serde::{Serialize, Deserialize}
use crate::Pool;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    sub: String,
    exp: usize,
}

fn generate_token(pool: web::Data<Pool>, user_id: i32) -> String {
    let conn = pool.get()?;
    let user = super::get_user(&conn, user_id);

    let key = user.pass;
    let claim = Claims {
        iss: "DakotaDB".to_owned(),
        sub: user.email.to_owned(),
        exp: 10000000000,
    };
    match encode(&Header::default(), &claim, &EncodingKey::from_secret(key)) {
        Ok(t) => t,
        Err(e) => e,
    }
}

fn validate(pool: web::Data<Pool>, token: &'a str) -> i32 {
    let mut validate = Validation::new(Algorithm::HS256);
    
    let conn = pool.get()?;
    let all_users = get_all_users(&conn);

    let mut token_data = "";
    let mut i = 0;
    let mut id = 0;

    for u in all_users {
        i += 1;
        validate.sub = Some(u.email);
        let key = u.pass.into_bytes();
        let token_data = match decode::<Claims>(token, &DecodingKey::from_secret(key), &validate) {
            Ok(c) => c,
            Err(_) => continue,
        };
        id = i;
        break;
    }
    
    id
}
