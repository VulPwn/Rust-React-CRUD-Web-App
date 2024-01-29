use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation}
use serde::{Serialize, Deserialize}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    aud: String,
    sub: String,
    exp: usize,
}

fn main() {
    let key = b"my secret key";
    let my_claim = Claims {
        iss: "DakotaDB".to_owned(),
        aud: "me".to_owned(),
        sub: "example@example.com".to_owned(),
        exp: 10000000000,
    };
    let token = match encode(&Header::default(), &my_claim, &EncodingKey::from_secret(key)) {
        Ok(t) => t,
        Err(_) => panic!(),
    };

    let mut validation = Validation::new(Algorithm::HS256);
    validation.sub = Some("example@example.com".to_string());
    validation.set_audience(&["me"]);
    let token_data = match decode::<Claims>(&token, &DecodingKey::from_secret(key), &validation) {
        Ok(c) => c,
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => panic!("Token is invalid"),
            ErrorKind::InvalidIssuer => panic!("Issuer is invalid"),
            _ => panic!("Some other errors"),
        },
    };
    println!("{:?}", token_data.claims);
    println!("{:?}", token_data.header);
}
