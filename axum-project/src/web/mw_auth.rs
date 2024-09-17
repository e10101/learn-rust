use axum::body::Body;
use tower_cookies::Cookies;
use axum::response::Response;
use axum::http::Request;
use axum::middleware::Next;

use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

pub async fn mw_require_auth(
    cookies: Cookies,
    req: Request<Body>, 
    next: Next
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");
    
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    
    auth_token.ok_or(Error::AuthFailNoAuthTokenCookie)?;
    
    Ok(next.run(req).await)
}