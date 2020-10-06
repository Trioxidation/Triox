use actix_web::error::ErrorUnauthorized;
use actix_web::{dev, Error, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, errors, Algorithm, DecodingKey, Validation};

/// JWT claims.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: String,
    pub id: u32,
    pub role: u8,
    pub exp: usize,
}

/// Extractor for JWT claims.
///
/// Can be used by accessing claims in service parameters.
///
/// Automatically returns an error if claims are missing or invalid.
impl FromRequest for Claims {
    type Error = Error;
    type Future = Ready<Result<Claims, Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let jwt_header = req.headers().get("Triox-JWT");

        if let Some(jwt) = jwt_header {
            let token: &str = match jwt.to_str() {
                Ok(str) => str,
                Err(_) => return err(ErrorUnauthorized("invalid token!")),
            };

            let token = token.trim();

            match decode::<Claims>(
                token,
                &DecodingKey::from_secret("secret".as_ref()),
                &Validation::new(Algorithm::HS256),
            ) {
                Ok(token) => ok(token.claims),
                Err(e) => match e.kind() {
                    errors::ErrorKind::ExpiredSignature => err(ErrorUnauthorized("expired token!")),
                    _ => err(ErrorUnauthorized("invalid token!")),
                },
            }
        } else {
            err(ErrorUnauthorized("no token!"))
        }
    }
}
