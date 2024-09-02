use crate::{
  app_writer::{AppWriter, AppResult, ErrorResponseBuilder},
  dtos::user::{
    UserAddRequest, UserLoginRequest, UserLoginResponse, UserResponse, UserUpdateRequest,
  },
  middleware::jwt::decode_token,
  services::user_account,
};
use askama::Template;
use salvo::{oapi::endpoint, http::cookie::Cookie, oapi::extract::{JsonBody, PathParam}, writing::{Redirect, Text}, Request, Response, handler};
use salvo::Writer;
use uuid::Uuid;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {}

#[endpoint(tags("comm"))]
pub async fn login_page(res: &mut Response) -> AppResult<()> {
  let cookies = res.cookies();
  let cookie = cookies.get("jwt_token");
  if let Some(cookie) = cookie {
    let token = cookie.value().to_string();
    if decode_token(&token) {
      res.render(Redirect::other("/user_accounts"));
      return Ok(());
    } else {}
  }
  let hello_tmpl = LoginTemplate {};
  res.render(Text::Html(hello_tmpl.render().unwrap()));
  Ok(())
}

#[derive(Template)]
#[template(path = "user_list_page.html")]
pub struct UserListPageTemplate {}

#[derive(Template)]
#[template(path = "user_list.html")]
pub struct UserListTemplate {}

#[endpoint]
pub async fn user_account_list_page(req: &mut Request, res: &mut Response) -> AppResult<()> {
  let is_fragment = req.headers().get("X-Fragment-Header");
  match is_fragment {
    Some(_) => {
      let hello_tmpl = UserListTemplate {};
      res.render(Text::Html(hello_tmpl.render().unwrap()));
    }
    None => {
      let hello_tmpl = UserListPageTemplate {};
      res.render(Text::Html(hello_tmpl.render().unwrap()));
    }
  }
  Ok(())
}

#[endpoint(tags("comm"))]
pub async fn post_login(req: JsonBody<UserLoginRequest>, res: &mut Response) {
  let result: AppResult<UserLoginResponse> = user_account::login(req.0).await;
  match result {
    Ok(data) => {
      let jwt_token = data.token.clone();
      let cookie = Cookie::build(("jwt_token", jwt_token))
        .path("/")
        .http_only(true)
        .build();
      res.add_cookie(cookie);
    }
    Err(e) => ErrorResponseBuilder::with_err(e).into_response(res),
  }
}

#[endpoint(tags("user_accounts"))]
pub async fn post_add_user_account(new_user: JsonBody<UserAddRequest>) -> AppWriter<UserResponse> {
  let result = user_account::add_user_account(new_user.0).await;
  AppWriter(result)
}

#[endpoint(tags("user_accounts"),
  parameters(
    ("id", description = "user id"),
  ))]
pub async fn put_update_user_account(req: &mut Request) -> AppResult<AppWriter<UserResponse>> {
  let req: UserUpdateRequest = req.extract().await?;
  let result = user_account::update_user_account(req).await;
  Ok(AppWriter(result))
}

#[endpoint(tags("user_accounts"))]
pub async fn delete_user_account(id: PathParam<Uuid>) -> AppWriter<()> {
  let result = user_account::delete_user_account(id.0).await;
  AppWriter(result)
}

#[endpoint(tags("user_accounts"))]
pub async fn get_user_accounts() -> AppWriter<Vec<UserResponse>> {
  let result = user_account::get_user_accounts().await;
  AppWriter(result)
}

