use crate::database::{DbPool, DbResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

pub type DeviceId = Uuid;

/// Model representing a device
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DeviceModel {
    /// Unique ID of the device
    pub id: DeviceId,
    /// Name of the device
    pub name: String,
    /// Host address of the device
    pub host: String,
    /// Port of the device
    pub port: i32,
    /// Private key used with this server
    pub client_private_key: Vec<u8>,
    /// Last known and approved public key used by the server
    pub server_public_key: Option<Vec<u8>>,
    /// Order in the UI the device is displayed
    pub order: u32,
    /// Date the device was created
    pub created_at: DateTime<Utc>,
}

/// Values required to create a device
#[derive(Deserialize)]
pub struct CreateDevice {
    pub name: String,
    pub host: String,
    pub port: i32,
    pub order: u32,
    pub client_private_key: Vec<u8>,
}

impl DeviceModel {
    /// Create a new device
    pub async fn create(db: &DbPool, create: CreateDevice) -> DbResult<DeviceModel> {
        let model = DeviceModel {
            id: Uuid::new_v4(),
            host: create.host,
            port: create.port,
            name: create.name,
            client_private_key: create.client_private_key,
            server_public_key: None,
            order: create.order,
            created_at: Utc::now(),
        };

        sqlx::query(
            r#"
            INSERT INTO "devices" ("id", "name", "host", "port", "client_private_key", "server_public_key", "order", "created_at")
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        )
        .bind(model.id)
        .bind(&model.name)
        .bind(&model.host)
        .bind(model.port)
        .bind(&model.client_private_key)
        .bind(&model.server_public_key)
        .bind(model.order)
        .bind(model.created_at)
        .execute(db)
        .await?;

        Ok(model)
    }

    pub async fn set_server_public_key(
        mut self,
        db: &DbPool,
        server_public_key: Option<Vec<u8>>,
    ) -> DbResult<DeviceModel> {
        sqlx::query(r#"UPDATE "devices" SET "server_public_key" = ? WHERE "id" = ?"#)
            .bind(&server_public_key)
            .bind(self.id)
            .execute(db)
            .await?;

        self.server_public_key = server_public_key;

        Ok(self)
    }

    /// Delete a device by ID
    pub async fn delete(db: &DbPool, device_id: DeviceId) -> DbResult<()> {
        sqlx::query(r#"DELETE FROM "devices" WHERE "id" = ?"#)
            .bind(device_id)
            .execute(db)
            .await?;
        Ok(())
    }

    /// Get a specific device by ID
    pub async fn get_by_id(db: &DbPool, id: DeviceId) -> DbResult<Option<DeviceModel>> {
        sqlx::query_as(r#"SELECT * FROM "devices" WHERE "id" = ?"#)
            .bind(id)
            .fetch_optional(db)
            .await
    }

    /// Get all devices
    pub async fn all(db: &DbPool) -> DbResult<Vec<DeviceModel>> {
        sqlx::query_as(r#"SELECT * FROM "devices" ORDER BY "order" ASC"#)
            .fetch_all(db)
            .await
    }
}

#[cfg(test)]
mod test {
    use uuid::Uuid;

    use crate::database::{
        entity::device::{CreateDevice, DeviceModel},
        mock_database,
    };

    /// Tests creating a device, ensures the returned model is correct
    /// and that the inserted value matches
    #[tokio::test]
    async fn test_create_device() {
        let name = "Test".to_string();
        let host = "127.0.0.1".to_string();
        let port = 8080;
        let order = 0;
        let client_private_key = vec![0, 1];

        let db = mock_database().await;
        let model = DeviceModel::create(
            &db,
            CreateDevice {
                name: name.clone(),
                host: host.clone(),
                port,
                order,
                client_private_key: client_private_key.clone(),
            },
        )
        .await
        .unwrap();

        // Ensure the return model is correct
        assert_eq!(model.name, name);
        assert_eq!(model.host, host);
        assert_eq!(model.port, port);
        assert_eq!(model.order, order);
        assert_eq!(model.client_private_key, client_private_key);

        // Ensure the copy in the database is correct
        let model = DeviceModel::get_by_id(&db, model.id)
            .await
            .unwrap()
            .expect("model should exist");
        assert_eq!(model.name, name);
        assert_eq!(model.host, host);
        assert_eq!(model.port, port);
        assert_eq!(model.order, order);
        assert_eq!(model.client_private_key, client_private_key);
    }

    /// Tests that deleting a device works as intended
    #[tokio::test]
    async fn test_delete_device() {
        let db = mock_database().await;

        // Create a model to work with
        let model = DeviceModel::create(
            &db,
            CreateDevice {
                name: "Test".to_string(),
                host: "127.0.0.1".to_string(),
                port: 8080,
                order: 0,
                client_private_key: vec![0, 1],
            },
        )
        .await
        .unwrap();

        // Delete the model
        DeviceModel::delete(&db, model.id).await.unwrap();

        // Model should no longer exist
        let model = DeviceModel::get_by_id(&db, model.id).await.unwrap();
        assert!(model.is_none());
    }

    /// Tests updating a device correctly updates the fields in the
    /// returned model and in the database
    #[tokio::test]
    async fn test_devices_set_server_public_key() {
        let name = "Test 2".to_string();
        let host = "127.0.0.2".to_string();
        let port = 8081;
        let order = 1;
        let server_public_key = Some(vec![1, 1]);
        let db = mock_database().await;

        // Create a model to work with
        let model = DeviceModel::create(
            &db,
            CreateDevice {
                name: "Test 2".to_string(),
                host: "127.0.0.2".to_string(),
                port: 8081,
                order: 1,
                client_private_key: vec![0, 1],
            },
        )
        .await
        .unwrap();

        // Update the model
        let model = model
            .set_server_public_key(&db, server_public_key.clone())
            .await
            .unwrap();

        // Ensure the returned model is correct
        assert_eq!(model.name, name);
        assert_eq!(model.host, host);
        assert_eq!(model.port, port);
        assert_eq!(model.order, order);
        assert_eq!(model.server_public_key, server_public_key);

        // Ensure the copy in the database is correct
        let model = DeviceModel::get_by_id(&db, model.id)
            .await
            .unwrap()
            .expect("model should exist");
        assert_eq!(model.name, name);
        assert_eq!(model.host, host);
        assert_eq!(model.port, port);
        assert_eq!(model.order, order);
        assert_eq!(model.server_public_key, server_public_key);
    }

    /// Tests that getting a device by ID works
    #[tokio::test]
    async fn test_get_device_by_id() {
        let name = "Test".to_string();
        let host = "127.0.0.1".to_string();
        let port = 8080;
        let order = 0;

        let db = mock_database().await;
        let model = DeviceModel::create(
            &db,
            CreateDevice {
                name: name.clone(),
                host: host.clone(),
                port,
                order,
                client_private_key: vec![0, 1],
            },
        )
        .await
        .unwrap();

        // Ensure the created device exists
        DeviceModel::get_by_id(&db, model.id)
            .await
            .unwrap()
            .expect("model should exist");

        // Ensure an unknown model doesn't exist
        let model = DeviceModel::get_by_id(&db, Uuid::new_v4()).await.unwrap();
        assert!(model.is_none());
    }

    /// Tests that getting a device by ID works
    #[tokio::test]
    async fn test_get_all_devices() {
        #[rustfmt::skip]
        let devices_data = [
            ("Test".to_string(), "127.0.0.1".to_string(), 8480, 0, vec![0, 1]),
            ("Test 1".to_string(), "127.0.0.3".to_string(), 8280, 2, vec![0, 2]),
            ("Test 2".to_string(), "127.0.0.2".to_string(), 8082, 1, vec![0, 3]),
            ("Test 3".to_string(), "127.0.0.4".to_string(), 8180, 5, vec![0, 4]),
            ("Test 4".to_string(), "127.0.0.2".to_string(), 8081, 10,vec![0, 5]),
        ];

        // Prep an ordered copy of the results
        let mut ordered_data = devices_data.to_vec();
        ordered_data.sort_by(|a, b| a.3.cmp(&b.3));

        let db = mock_database().await;

        // Create the devices
        for (name, host, port, order, client_private_key) in devices_data {
            DeviceModel::create(
                &db,
                CreateDevice {
                    name: name.clone(),
                    host: host.clone(),
                    port,
                    order,
                    client_private_key: client_private_key.clone(),
                },
            )
            .await
            .unwrap();
        }

        let devices = DeviceModel::all(&db).await.unwrap();

        for (index, (name, host, port, order, client_private_key)) in
            ordered_data.into_iter().enumerate()
        {
            let model = &devices[index];
            assert_eq!(model.name, name);
            assert_eq!(model.host, host);
            assert_eq!(model.port, port);
            assert_eq!(model.order, order);
            assert_eq!(model.client_private_key, client_private_key);
        }
    }
}
