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
            errors::ErrorKind::ExpiredSignature => {
                err(ErrorUnauthorized("expired token!"))
            }
            _ => err(ErrorUnauthorized("invalid token!")),
        },
    }
}

/// Extractor for JWT header as String.
///
/// Returns an error if header is missing or can't be converted to string.
impl FromRequest for JWT {
    type Error = Error;
    type Future = Ready<Result<JWT, Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        // Extract token from header

        if let Some(jwt) = req.headers().get("authorization") {
            // Extract string from header
            // The header should look like this:
            // Bearer ENOCODED_JWT
            let auth_str: &str = match jwt.to_str() {
                Ok(str) => str,
                Err(_) => {
                    return err(ErrorUnauthorized("invalid authorization header!"))
                }
            };

            // We expect the header to authorization type "Bearer"
            let opt_bearer_pos = auth_str.find("Bearer");

            if let Some(bearer_pos) = opt_bearer_pos {
                // Remove the "Bearer" string and extract the JWT itself
                // This returns an Option because we don't know whether the string is long enough
                let opt_jwt_str = &auth_str.get("Bearer".len() + bearer_pos..);
                if let Some(jwt_str) = opt_jwt_str {
                    let jwt_str = jwt_str.trim();

                    ok(JWT(jwt_str.to_string()))
                } else {
                    err(ErrorUnauthorized("invalid token!"))
                }
            } else {
                err(ErrorUnauthorized("invalid authorization type!"))
            }
        }
        // Optionally the authorization can use cookies to send the JWT
        else if let Some(cookies) = req.headers().get("cookie") {
            // Extract string from token
            let cookie_str: &str = match cookies.to_str() {
                Ok(str) => str,
                Err(_) => return err(ErrorUnauthorized("invalid cookie header!")),
            };

            // Cookies are divided by spaces, let's split them up!
            let cookies: Vec<&str> = cookie_str.split(' ').collect();

            // Can we find a "triox_jwt" cookie?
            let mut opt_jwt_cookie: Option<&str> = None;
            for cookie in cookies.iter() {
                if cookie.contains("triox_jwt=") {
                    opt_jwt_cookie = Some(cookie);
                    break;
                }
            }

            if let Some(jwt_cookie) = opt_jwt_cookie {
                let jwt_cookie = jwt_cookie.trim();

                // Remove the "triox_jwt=" part from the cookie to extract the JWT
                let jwt_str = &jwt_cookie["triox_jwt=".len()..];

                ok(JWT(jwt_str.to_string()))
            } else {
                err(ErrorUnauthorized("no token in cookie header!"))
            }
        } else {
            err(ErrorUnauthorized("no token!"))
        }
    }
}
