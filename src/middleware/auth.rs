use salvo::jwt_auth::{ConstDecoder, HeaderFinder, JwtAuth};

use crate::config::Claims;

pub fn jwt_auth() -> JwtAuth<Claims, ConstDecoder> {
    let secret_key =
        std::env::var("JWT_SECRET").expect("SECRET_KEY must be set in environment variables");

    return JwtAuth::new(ConstDecoder::from_secret(secret_key.as_bytes()))
        .finders(vec![Box::new(HeaderFinder::new())])
        .force_passed(true);
}
