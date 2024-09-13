pub enum DeletionMode {
  Hard,
  Soft,
}

pub struct PaginatorOption {
  pub page_size: u64,
  pub page: u64,
}

impl PaginatorOption {
  pub fn new(&mut self) -> &mut PaginatorOption {
    self.page_size = 500;
    self.page = 1;
    self
  }

  pub fn init(&mut self, page_size: u64, page: u64) {
    self.page_size = page_size;
    self.page = page;
  }
}

pub mod balance_history;
pub mod block;
pub mod chat_group;
pub mod chat_group_member;
pub mod department;
pub mod direct_chat;
pub mod direct_chat_reaction;
pub mod email;
pub mod follow;
pub mod friend;
pub mod friend_group;
pub mod group_chat;
pub mod group_chat_reaction;
pub mod job;
pub mod job_project;
pub mod job_skill;
pub mod link;
pub mod link_code;
pub mod message;
pub mod note;
pub mod note_edit_history;
pub mod note_share;
pub mod organization;
pub mod organization_address;
pub mod organization_administrator;
pub mod organization_role;
pub mod person;
pub mod phone;
pub mod photo;
pub mod post;
pub mod post_bookmark;
pub mod post_comment;
pub mod post_reaction;
pub mod post_share;
pub mod project;
pub mod project_skill;
pub mod skill;
pub mod socmed_url;
pub mod tweet;
pub mod tweet_bookmark;
pub mod tweet_reaction;
pub mod user_account;
pub mod verification_token;
pub mod video;
