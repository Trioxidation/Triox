use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use actix_web::{dev, http, Error, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{
    decode, encode, errors, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};

/// JWT claims.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: String,
    pub id: u32,
    pub role: u8,
    pub exp: usize,
}

pub struct JWT(pub String);

/// Helper function for encoding claims to JWT string
pub fn encode_claims(claims: &Claims, secret: &str) -> Result<String, Error> {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| ErrorInternalServerError("JWTs encoding failed"))
}

/// Helper function for extracting claims from JWT string
pub fn extract_claims(jwt: &str, secret: &str) -> Ready<Result<Claims, Error>> {
    match decode::<Claims>(
        jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(token) => ok(token.claims),
        Err(e) => match e.kind() {
            errors::ErrorKind::ExpiredSignature => {
                err(ErrorUnauthorized("Expired token"))
            }
            _ => err(ErrorUnauthorized("Invalid token")),
        },
    }
}

/// Helper function to extract a JWT from the authorization header
fn extract_jwt_from_authorization_header(
    header: Option<&http::HeaderValue>,
) -> Option<Ready<Result<JWT, Error>>> {
    if let Some(jwt) = header {
        // Extract string from header
        // The header should look like this:
        // Bearer ENCODED_JWT
        let auth_str: &str = match jwt.to_str() {
            Ok(str) => str,
            Err(_) => {
                return Some(err(ErrorUnauthorized("Invalid authorization header")))
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

                Some(ok(JWT(jwt_str.to_string())))
            } else {
                Some(err(ErrorUnauthorized("Invalid authorization token")))
            }
        } else {
            Some(err(ErrorUnauthorized("Invalid authorization type")))
        }
    } else {
        None
    }
}

/// Helper function to extract a JWT from the cookie header
fn extract_jwt_from_cookie_header(
    header: Option<&http::HeaderValue>,
) -> Option<Ready<Result<JWT, Error>>> {
    if let Some(cookies) = header {
        // Extract string from token
        let cookie_str: &str = match cookies.to_str() {
            Ok(str) => str,
            Err(_) => return Some(err(ErrorUnauthorized("Invalid cookie header"))),
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

            Some(ok(JWT(jwt_str.to_string())))
        } else {
            Some(err(ErrorUnauthorized(
                "No authorization token inside cookie header",
            )))
        }
    } else {
        None
    }
}

/// Extractor for JWT header as JWT struct (wrapper around String).
///
/// Returns an error if header is missing or can't be converted to string.
impl FromRequest for JWT {
    type Error = Error;
    type Future = Ready<Result<JWT, Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        // Extract token from authorization header
        if let Some(auth_res) =
            extract_jwt_from_authorization_header(req.headers().get("authorization"))
        {
            auth_res
        }
        // Optionally the authorization can use cookies to send the JWT
        else if let Some(cookie_res) =
            extract_jwt_from_cookie_header(req.headers().get("cookie"))
        {
            cookie_res
        }
        // no token found
        else {
            err(ErrorUnauthorized("No authorization token was sent"))
        }
    }
}
