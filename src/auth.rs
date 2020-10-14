use crate::jwt;
use jsonwebtoken::{encode, EncodingKey, Header};

use actix_web::error::BlockingError;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorUnauthorized};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use tokio::fs;

use std::time::{SystemTime, UNIX_EPOCH};

use crate::database::users::DbErrorType;

use crate::{database, AppState};

/// Information required for sign in.
#[derive(serde::Deserialize)]
pub struct SignInForm {
    pub user_name: String,
    pub password: String,
}

/// Information required for sign up.
#[derive(serde::Deserialize)]
pub struct SignUpForm {
    pub user_name: String,
    pub password: String,
    pub email: String, //TODO: captcha: Vec<u8>
}

/// Return user information stored inside the JWT.
pub async fn user_info(claims: jwt::Claims) -> HttpResponse {
    HttpResponse::Ok().json(claims)
}

/// Give user sign in page
pub async fn sign_in_page(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("<h1>SIGN IN PAGE</h1>
		<form>
			<label for='username'>Username:</label><br>
			<input type='text' id='user_name' name='user_name'><br>
			<label for='password'>Password:</label><br>
			<input type='password' id='password' name='password'><br><br>
			<input type='button' value='Submit' onclick='submitform()'><span style='padding-left: 50px;'><input type='button' onclick='location.href=\"/sign_up\"' value='or sign up' /></span>
		</form>

		<script type='text/javascript'>
    	function submitform(){
			data = JSON.stringify({
				'user_name': document.getElementsByTagName('input')[0].value,
				'password': document.getElementsByTagName('input')[1].value
			})
			console.log(data);
	        var xhr = new XMLHttpRequest();
	        xhr.open('post', '/sign_in', true);
	        xhr.setRequestHeader('Content-Type', 'application/json');
			xhr.onreadystatechange = function() {
				if (xhr.readyState === 4 && xhr.status === 200) {
					console.log(xhr.responseText);
				}
			}
        	xhr.send(data);
		}
		</script>
		")
}

/// Give user sign up page
pub async fn sign_up_page(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("<h1>SIGN UP PAGE</h1>
        <form>
            <label for='username'>Username:</label><br>
            <input type='text' id='user_name' name='user_name'><br>
            <label for='password'>Password:</label><br>
            <input type='password' id='password' name='password'><br>
            <label for='email'>Email:</label><br>
            <input type='text' id='email' name='email'><br><br>
            <input type='button' value='Submit' onclick='submitform()'><span style='padding-left: 50px;'><input type='button' onclick='location.href=\"/sign_in\"' value='or sign in' /></span>
        </form>

		<script type='text/javascript'>
        function submitform(){
            data = JSON.stringify({
                'user_name': document.getElementsByTagName('input')[0].value,
                'password': document.getElementsByTagName('input')[1].value,
                'email': document.getElementsByTagName('input')[2].value,
            })
            console.log(data);
            var xhr = new XMLHttpRequest();
            xhr.open('post', '/sign_up', true);
            xhr.setRequestHeader('Content-Type', 'application/json');
            xhr.onreadystatechange = function() {
                if (xhr.readyState === 4 && xhr.status === 200) {
					console.log(xhr.responseText);
                }
            }
            xhr.send(data);
        }
        </script>

        ")
}


/// Sign in user and return JWT on success.
pub async fn sign_in(
    app_state: web::Data<AppState>,
    form: web::Json<SignInForm>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || database::users::authenticate_user(&form, app_state.clone()))
        .await
        .map_err(|outer_err| match outer_err {
            BlockingError::Error(err) => match err.err_type {
                DbErrorType::InternalServerError => ErrorInternalServerError(err.cause),
                DbErrorType::BadRequest => ErrorBadRequest(err.cause),
                DbErrorType::Unauthorized => ErrorUnauthorized(err.cause),
            },
            BlockingError::Canceled => ErrorInternalServerError("database request canceled"),
        })?;

    // Get unix time stamp
    let time_now = SystemTime::now();
    let timestamp = time_now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize;

    // create claims
    let claims = jwt::Claims {
        sub: user.name,
        id: user.id,
        role: user.role,
        exp: timestamp + 7200, // now + two hours
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    );

    match token {
        Ok(token) => Ok(HttpResponse::Ok().body(token)),
        Err(_) => Err(ErrorInternalServerError("JWTs generation failed")),
    }
}

/// Sign up user by creating files and database entries.
pub async fn sign_up(
    app_state: web::Data<AppState>,
    form: web::Json<SignUpForm>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || database::users::add_user(&form, app_state.clone()))
        .await
        .map_err(|outer_err| match outer_err {
            BlockingError::Error(err) => match err.err_type {
                DbErrorType::InternalServerError => ErrorInternalServerError(err.cause),
                DbErrorType::BadRequest => ErrorBadRequest(err.cause),
                DbErrorType::Unauthorized => ErrorUnauthorized(err.cause),
            },
            BlockingError::Canceled => ErrorInternalServerError("database request canceled"),
        })?;

    // generate storage path for user
    let path: std::path::PathBuf = [".", "data", "users", &user.id.to_string(), "files"]
        .iter()
        .collect();

    fs::create_dir_all(path).await?;

    Ok(HttpResponse::Ok().body("user created"))
}
