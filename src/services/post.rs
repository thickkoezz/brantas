use super::{DeletionMode, PaginatorOption};
use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::post::{PostAddRequest, PostResponse};
use crate::entities::{post, prelude::Post};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

pub async fn add_post(req: PostAddRequest) -> AppResult<PostResponse> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  let model = post::ActiveModel {
    owner_id: Set(req.owner_id),
    created_at: Set(DateTimeWithTimeZone::from(chrono::Local::now())),
    updated_at: Set(None),
    deleted_at: Set(None),
    title: Set(req.title),
    content: Set(req.content),
    is_published: Set(req.is_published),
    hashtag: Set(req.hashtag),
    view_count: Set(0),
    comment_count: Set(0),
    reaction_count: Set(0),
    is_public: Set(req.is_public),
    group_name: Set(req.group_name),
    can_comment: Set(req.can_comment),
  };
  let post = Post::insert(model.clone()).exec(db).await?;
  Ok(PostResponse {
    owner_id: post.last_insert_id.0,
    created_at: post.last_insert_id.1,
    ..PostResponse::from(model)
  })
}

pub async fn delete_post(
  deletion_mode: DeletionMode,
  owner_id: Uuid,
  created_at: DateTimeWithTimeZone,
) -> AppResult<()> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match deletion_mode {
    DeletionMode::Hard => {
      let result = Post::delete_by_id((owner_id, created_at)).exec(db).await?;
      match result.rows_affected {
        0 => Err(anyhow::anyhow!(t!("x_not_deleted", x = t!("post"))).into()),
        _ => Ok(()),
      }
    },
    DeletionMode::Soft => {
      let post = Post::find_by_id((owner_id, created_at)).one(db).await?;
      if post.is_none() {
        return Err(anyhow::anyhow!(t!("x_not_exist", x = t!("post"))).into());
      }

      let mut post: post::ActiveModel = post.unwrap().into();
      post.deleted_at = Set(Option::from(DateTimeWithTimeZone::from(
        chrono::Local::now(),
      )));
      post.update(db).await?;
      Ok(())
    },
  }
}

pub async fn get_post(paginator_option: Option<PaginatorOption>) -> AppResult<Vec<PostResponse>> {
  let db = DB
    .get()
    .ok_or(anyhow::anyhow!(t!("database_connection_failed")))?;
  match paginator_option {
    Some(paginator_option) => {
      let posts = Post::find()
        .paginate(db, paginator_option.page_size)
        .fetch_page(paginator_option.page)
        .await?;
      let res = posts
        .into_iter()
        .map(|post: post::Model| PostResponse::from(post))
        .collect::<Vec<_>>();
      Ok(res)
    },
    None => {
      let posts = Post::find().all(db).await?;
      let res = posts
        .into_iter()
        .map(|post: post::Model| PostResponse::from(post))
        .collect::<Vec<_>>();
      Ok(res)
    },
  }
}
