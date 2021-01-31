use actix::prelude::*;
use chrono::{DateTime, Utc};
use std::io::Error;
use uuid::Uuid;

use crate::{
    core::wql::{
        create_entity, insert_entity_content, update_content_entity_content,
        update_set_entity_content,
    },
    model::DataRegister,
};

pub struct Executor;

impl Actor for Executor {
    type Context = Context<Self>;
}

impl Executor {
    pub fn new() -> Self {
        Executor {}
    }
}

pub struct CreateEntity {
    pub name: String,
}

impl Message for CreateEntity {
    type Result = Result<usize, Error>;
}

impl Handler<CreateEntity> for Executor {
    type Result = Result<usize, Error>;

    fn handle(&mut self, msg: CreateEntity, _: &mut Self::Context) -> Self::Result {
        use crate::io::write::write_to_log;
        let entity = create_entity(&msg.name);
        write_to_log(&entity)
    }
}

pub struct InsertEntityContent {
    pub name: String,
    pub content: String,
}

impl Message for InsertEntityContent {
    type Result = Result<(DateTime<Utc>, Uuid, usize), Error>;
}

impl Handler<InsertEntityContent> for Executor {
    type Result = Result<(DateTime<Utc>, Uuid, usize), Error>;

    fn handle(&mut self, msg: InsertEntityContent, _: &mut Self::Context) -> Self::Result {
        use crate::io::write::write_to_log;
        let (date, uuid, content) = insert_entity_content(msg);
        Ok((date, uuid, write_to_log(&content)?))
    }
}

pub struct UpdateSetEntityContent {
    pub name: String,
    pub current_state: String,
    pub content_log: String,
    pub id: Uuid,
    pub previous_registry: DataRegister,
}

impl Message for UpdateSetEntityContent {
    type Result = Result<(DateTime<Utc>, usize), Error>;
}

impl Handler<UpdateSetEntityContent> for Executor {
    type Result = Result<(DateTime<Utc>, usize), Error>;

    fn handle(&mut self, msg: UpdateSetEntityContent, _: &mut Self::Context) -> Self::Result {
        use crate::io::write::write_to_log;
        let (date, content) = update_set_entity_content(msg);
        Ok((date, write_to_log(&content)?))
    }
}

// I know it is duplicated
pub struct UpdateContentEntityContent {
    pub name: String,
    pub current_state: String,
    pub content_log: String,
    pub id: Uuid,
    pub previous_registry: DataRegister,
}

impl Message for UpdateContentEntityContent {
    type Result = Result<(DateTime<Utc>, usize), Error>;
}

impl Handler<UpdateContentEntityContent> for Executor {
    type Result = Result<(DateTime<Utc>, usize), Error>;

    fn handle(&mut self, msg: UpdateContentEntityContent, _: &mut Self::Context) -> Self::Result {
        use crate::io::write::write_to_log;
        let (date, content) = update_content_entity_content(msg);
        Ok((date, write_to_log(&content)?))
    }
}
