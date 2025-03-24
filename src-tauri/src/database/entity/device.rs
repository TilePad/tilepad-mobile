use chrono::{DateTime, Utc};
use sea_query::{Expr, IdenStatic, Query};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::database::{
    helpers::{sql_exec, sql_query_all, sql_query_maybe_one, UpdateStatementExt},
    DbPool, DbResult,
};

pub type DeviceId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DeviceModel {
    pub id: DeviceId,
    pub name: String,
    pub host: String,
    pub port: i32,
    pub access_token: Option<String>,
    pub order: u32,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateDevice {
    pub name: String,
    pub host: String,
    pub port: i32,
    pub order: u32,
    pub access_token: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateDevice {
    pub name: Option<String>,
    pub host: Option<String>,
    pub port: Option<i32>,
    pub order: Option<u32>,
    pub access_token: Option<Option<String>>,
}

impl DeviceModel {
    pub async fn create(db: &DbPool, create: CreateDevice) -> anyhow::Result<DeviceModel> {
        let model = DeviceModel {
            id: Uuid::new_v4(),
            host: create.host,
            port: create.port,
            name: create.name,
            access_token: create.access_token,
            order: 0,
            created_at: Utc::now(),
        };

        sql_exec(
            db,
            Query::insert()
                .into_table(DevicesTable)
                .columns([
                    DevicesColumn::Id,
                    DevicesColumn::Name,
                    DevicesColumn::Host,
                    DevicesColumn::Port,
                    DevicesColumn::AccessToken,
                    DevicesColumn::Order,
                    DevicesColumn::CreatedAt,
                ])
                .values_panic([
                    model.id.into(),
                    model.name.clone().into(),
                    model.host.clone().into(),
                    model.port.into(),
                    model.access_token.clone().into(),
                    model.order.into(),
                    model.created_at.into(),
                ]),
        )
        .await?;

        Ok(model)
    }

    pub async fn get_by_id(db: &DbPool, id: DeviceId) -> DbResult<Option<DeviceModel>> {
        sql_query_maybe_one(
            db,
            Query::select()
                .from(DevicesTable)
                .columns([
                    DevicesColumn::Id,
                    DevicesColumn::Name,
                    DevicesColumn::Host,
                    DevicesColumn::Port,
                    DevicesColumn::AccessToken,
                    DevicesColumn::Order,
                    DevicesColumn::CreatedAt,
                ])
                .and_where(Expr::col(DevicesColumn::Id).eq(id)),
        )
        .await
    }

    pub async fn update(
        mut self,
        db: &DbPool,

        update: UpdateDevice,
    ) -> anyhow::Result<DeviceModel> {
        sql_exec(
            db,
            Query::update()
                .table(DevicesTable)
                .and_where(Expr::col(DevicesColumn::Id).eq(self.id))
                .cond_value(DevicesColumn::Name, update.name.as_ref())
                .cond_value(DevicesColumn::Host, update.host.as_ref())
                .cond_value(DevicesColumn::Port, update.port.as_ref().copied())
                .cond_value(DevicesColumn::Order, update.order.as_ref().copied())
                .cond_value(
                    DevicesColumn::AccessToken,
                    update.access_token.as_ref().cloned(),
                ),
        )
        .await?;

        self.name = update.name.unwrap_or(self.name);
        self.host = update.host.unwrap_or(self.host);
        self.port = update.port.unwrap_or(self.port);
        self.order = update.order.unwrap_or(self.order);
        self.access_token = update.access_token.unwrap_or(self.access_token);

        Ok(self)
    }

    pub async fn all(db: &DbPool) -> DbResult<Vec<DeviceModel>> {
        sql_query_all(
            db,
            Query::select().from(DevicesTable).columns([
                DevicesColumn::Id,
                DevicesColumn::Name,
                DevicesColumn::Host,
                DevicesColumn::Port,
                DevicesColumn::AccessToken,
                DevicesColumn::Order,
                DevicesColumn::CreatedAt,
            ]),
        )
        .await
    }

    pub async fn delete(db: &DbPool, device_id: DeviceId) -> DbResult<()> {
        sql_exec(
            db,
            Query::delete()
                .from_table(DevicesTable)
                .and_where(Expr::col(DevicesColumn::Id).eq(device_id)),
        )
        .await
    }
}

#[derive(IdenStatic, Copy, Clone)]
#[iden(rename = "devices")]
pub struct DevicesTable;

#[derive(IdenStatic, Copy, Clone)]
pub enum DevicesColumn {
    /// Unique ID for the device
    Id,
    /// Host for connecting to the device
    Host,
    /// Port for connecting to the device
    Port,
    /// Name of the device
    Name,
    /// Access token for the device
    AccessToken,
    /// Order of the device in the UI
    Order,
    /// Timestamp when the device was added
    CreatedAt,
}
