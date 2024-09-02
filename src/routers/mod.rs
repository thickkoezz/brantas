use crate::middleware::jwt::jwt_middleware;
use salvo::{
  prelude::{CatchPanic, Logger, OpenApi, Scalar},
  Router,
};

use self::{
  demo::hello,
  user::{
    delete_user_account, get_user_accounts, login_page, post_add_user_account, post_login, put_update_user_account,
    user_account_list_page,
  },
};

pub mod demo;
pub mod user;
mod static_routers;

pub fn router() -> Router {
  let mut no_auth_routers = vec![
    Router::with_path("login").get(login_page),
    Router::with_path("/api/login").post(post_login),
  ];

  let mut need_auth_routers = vec![
    Router::with_path("user_accounts")
      .get(user_account_list_page),
    Router::with_path("/api/user_accounts").get(get_user_accounts)
      .post(post_add_user_account)
      .push(
        Router::with_path("<id>")
          .put(put_update_user_account)
          .delete(delete_user_account),
      ),
  ];

  let static_routers = static_routers::create_static_routers();
  no_auth_routers.extend(static_routers);

  let router = Router::new()
    .hoop(Logger::new())
    .hoop(CatchPanic::new())
    .get(hello)
    .append(&mut no_auth_routers)
    .push(
      Router::new()
        .append(&mut need_auth_routers)
        .hoop(jwt_middleware()),
    );

  let doc = OpenApi::new("salvo web api", "0.0.1").merge_router(&router);
  router
    .push(doc.into_router("/api-doc/openapi.json"))
    .push(Scalar::new("/api-doc/openapi.json").into_router("scalar"))
}
