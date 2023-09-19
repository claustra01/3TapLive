use async_graphql::Object;
use anyhow::Result;
use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods};

pub struct Query;

use crate::db;
use crate::models;
use crate::objects::gql_objects;
use crate::schema;

#[Object]
impl Query {

  async fn get_channel_list<'ctx>(
    &self,
    ctx: &async_graphql::Context<'ctx>,
  ) -> Result<Vec<gql_objects::Channel>> {

    // get channel data list
    let conn = &mut ctx.data_unchecked::<db::Pool>().get().unwrap();
    let all_channels: Vec<models::Channel> = schema::channels::dsl::channels
      .order(schema::channels::dsl::created_at.desc()) 
      .load::<models::Channel>(conn)?;
    
    // shape channel data
    let mut channels_data: Vec<gql_objects::Channel> = [].to_vec();
    for c in all_channels {
      let channel = gql_objects::Channel {
        name: c.name,
        token: c.token,
        title: c.title,
        owner_name: c.owner_name,
        owner_icon: c.owner_icon,
      };
      channels_data.push(channel);
    }
    Ok(channels_data)
  }

}