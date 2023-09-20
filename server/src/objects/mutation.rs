use async_graphql::Object;
use anyhow::Result;
use diesel::RunQueryDsl;
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

use crate::db;
use crate::models;
use crate::token;
use crate::objects::gql_objects;
use crate::schema;

pub struct Mutation;

#[Object]
impl Mutation {

  async fn create_channel<'ctx>(
    &self,
    ctx: &async_graphql::Context<'ctx>,
    title: String,
    owner_name: String,
    owner_icon: String
  ) -> Result<gql_objects::Channel> {

    // generate channel name and token
    let name = Uuid::new_v4().to_string();
    let token = token::get_token(name.clone()).await?;

    // create channel recode
    let conn = &mut ctx.data_unchecked::<db::Pool>().get().unwrap();
    let new_channel = models::NewChannel {
      name: &name,
      token: &token,
      title: &title,
      owner_name: &owner_name,
      owner_icon: &owner_icon,
    };
    diesel::insert_into(schema::channels::dsl::channels)
      .values(&new_channel)
      .execute(conn)?;

    // return new channel data
    let channel_data = gql_objects::Channel {
      name,
      token,
      title,
      owner_name,
      owner_icon,
    };    
    Ok(channel_data)
  }

  async fn create_comment<'ctx>(
    &self,
    ctx: &async_graphql::Context<'ctx>,
    body: String,
    channel: String,
    owner: String,
  ) -> Result<gql_objects::Comment> {

    // create comment recode
    let conn = &mut ctx.data_unchecked::<db::Pool>().get().unwrap();
    let new_comment = models::NewComment {
      body: &body,
      channel: &channel,
      owner: &owner,
    };
    diesel::insert_into(schema::comments::dsl::comments)
      .values(&new_comment)
      .execute(conn)?;
    
    // create new comment data
    let comment_data = gql_objects::Comment {
      body,
      channel,
      owner,
    };

    // push to subscription
    let cmt = ctx.data_unchecked::<UnboundedSender<gql_objects::Comment>>();
    cmt.send(comment_data.clone()).unwrap();

    // return new comment data
    Ok(comment_data)
  }

}