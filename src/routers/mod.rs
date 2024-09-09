use crate::middleware::jwt::jwt_middleware;
use salvo::{
  prelude::{CatchPanic, Logger, OpenApi, Scalar},
  Router,
};

use self::{
  demo::hello,
  user_account::{login_page, post_login, user_account_routes},
};

pub mod demo;
mod static_routers;
pub mod user_account;

pub fn router() -> Router {
  let mut no_auth_routers = vec![
    Router::with_path("login").get(login_page),
    Router::with_path("/api/login").post(post_login),
  ];

  let mut need_auth_routers: Vec<Router> = Vec::new();
  need_auth_routers.append(&mut crate::routers::user_account::user_account_routes());

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
