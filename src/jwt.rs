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

pub struct JWT(pub String);

/// Helper function for extracting claims from JWT string
pub fn extract_claims(jwt: &str, secret: &[u8]) -> Ready<Result<Claims, Error>> {
    match decode::<Claims>(
        jwt,
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(token) => ok(token.claims),
        Err(e) => match e.kind() {
            errors::ErrorKind::ExpiredSignature => err(ErrorUnauthorized("expired token!")),
            _ => err(ErrorUnauthorized("invalid token!")),
        },
    }
}

/// Extractor for JWT header as String.
///
/// Returns an error if header is missing can't be converted to string.
impl FromRequest for JWT {
    type Error = Error;
    type Future = Ready<Result<JWT, Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        // Extract token from header
        let jwt_header = req.headers().get("Triox-JWT");

        if let Some(jwt) = jwt_header {
            // Extract string from token
            let token: &str = match jwt.to_str() {
                Ok(str) => str,
                Err(_) => return err(ErrorUnauthorized("invalid token!")),
            };

            let token = token.trim();

            ok(JWT(token.to_string()))
        } else {
            err(ErrorUnauthorized("no token!"))
        }
    }
}
