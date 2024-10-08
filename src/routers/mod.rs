use self::{demo::hello, user_account::post_login};
use crate::middleware::cors::cors_middleware;
use crate::middleware::jwt::jwt_middleware;
use salvo::{
  prelude::{CatchPanic, Logger, OpenApi, Scalar},
  Router,
};

pub mod demo;
mod static_routers;
mod user_account;

pub fn router() -> Router {
  let mut no_auth_routers = vec![Router::with_path("/api/login").post(post_login)];
  no_auth_routers.extend(static_routers::create_static_routers());

  let mut need_auth_routers: Vec<Router> = Vec::new();
  need_auth_routers.extend(user_account::create_routers());

  let _cors_handler = cors_middleware();

  let router = Router::new()
    .hoop(_cors_handler)
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
