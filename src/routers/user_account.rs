use crate::services::PaginatorOption;
use crate::{
  app_writer::{AppResult, AppWriter, ErrorResponseBuilder},
  dtos::user_account::{
    UserAccountAddRequest, UserAccountLoginRequest, UserAccountLoginResponse, UserAccountResponse,
    UserAccountUpdateRequest,
  },
  services,
  services::user_account,
};
use salvo::Writer;
use salvo::{
  handler,
  http::cookie::Cookie,
  oapi::{
    endpoint,
    extract::{JsonBody, PathParam},
  },
  Request, Response, Router,
};
use uuid::Uuid;

#[handler]
async fn hello() -> &'static str {
  "hello world!"
}

#[endpoint(tags("user_accounts"))]
pub async fn post_login(req: JsonBody<UserAccountLoginRequest>, res: &mut Response) {
  let result: AppResult<UserAccountLoginResponse> = user_account::login(req.0).await;
  match result {
    Ok(data) => {
      let jwt_token = data.token.clone();
      let cookie = Cookie::build(("jwt_token", jwt_token))
        .path("/")
        .http_only(true)
        .build();
      res.add_cookie(cookie);
    },
    Err(e) => ErrorResponseBuilder::with_err(e).into_response(res),
  }
}

pub fn create_routers() -> Vec<Router> {
  vec![Router::with_path("/api/user_account")
    .get(get_user_accounts)
    .post(post_add_user_account)
    .push(
      Router::with_path("<id>")
        .put(put_update_user_account)
        .delete(delete_user_account),
    )]
}

#[endpoint(tags("user_accounts"))]
pub async fn post_add_user_account(
  new_user: JsonBody<UserAccountAddRequest>,
) -> AppWriter<UserAccountResponse> {
  let result = user_account::add_user_account(new_user.0).await;
  AppWriter(result)
}

#[endpoint(tags("user_accounts"),
  parameters(
    ("id", description = "user id"),
  ))]
pub async fn put_update_user_account(
  req: &mut Request,
) -> AppResult<AppWriter<UserAccountResponse>> {
  let req: UserAccountUpdateRequest = req.extract().await?;
  let result = user_account::update_user_account(req).await;
  Ok(AppWriter(result))
}

#[endpoint(tags("user_accounts"))]
pub async fn delete_user_account(id: PathParam<Uuid>) -> AppWriter<()> {
  let result = user_account::delete_user_account(services::DeletionMode::Soft, id.0).await;
  AppWriter(result)
}

#[endpoint(tags("user_accounts"))]
pub async fn get_user_accounts() -> AppWriter<Vec<UserAccountResponse>> {
  let option = Option::from(PaginatorOption {
    page_size: 500,
    page: 1,
  });
  let result = user_account::get_user_accounts(option).await;
  AppWriter(result)
}
