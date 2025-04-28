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
    /// Access token of the device (If authenticated)
    pub access_token: Option<String>,
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
    pub access_token: Option<String>,
}

impl DeviceModel {
    /// Create a new device
    pub async fn create(db: &DbPool, create: CreateDevice) -> DbResult<DeviceModel> {
        let model = DeviceModel {
            id: Uuid::new_v4(),
            host: create.host,
            port: create.port,
            name: create.name,
            access_token: create.access_token,
            order: create.order,
            created_at: Utc::now(),
        };

        sqlx::query(
            "
            INSERT INTO \"devices\" (\"id\", \"name\", \"host\", \"port\", \"access_token\", \"order\", \"created_at\")
            VALUES (?, ?, ?, ?, ?, ?, ?)
        ",
        )
        .bind(model.id)
        .bind(&model.name)
        .bind(&model.host)
        .bind(model.port)
        .bind(&model.access_token)
        .bind(model.order)
        .bind(model.created_at)
        .execute(db)
        .await?;

        Ok(model)
    }

    pub async fn set_access_token(
        mut self,
        db: &DbPool,
        access_token: Option<String>,
    ) -> DbResult<DeviceModel> {
        sqlx::query("UPDATE \"profiles\" SET \"access_token\" = ? WHERE \"id\" = ?")
            .bind(&access_token)
            .bind(self.id)
            .execute(db)
            .await?;

        self.access_token = access_token;

        Ok(self)
    }

    /// Delete a device by ID
    pub async fn delete(db: &DbPool, device_id: DeviceId) -> DbResult<()> {
        sqlx::query("DELETE FROM \"devices\" WHERE \"id\" = ?")
            .bind(device_id)
            .execute(db)
            .await?;
        Ok(())
    }

    /// Get a specific device by ID
    pub async fn get_by_id(db: &DbPool, id: DeviceId) -> DbResult<Option<DeviceModel>> {
        sqlx::query_as("SELECT * FROM \"devices\" WHERE \"id\" = ?")
            .bind(id)
            .fetch_optional(db)
            .await
    }

    /// Get all devices
    pub async fn all(db: &DbPool) -> DbResult<Vec<DeviceModel>> {
        sqlx::query_as("SELECT * FROM \"devices\" ORDER BY \"order\" ASC")
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
        let access_token = Some("Test".to_string());

        let db = mock_database().await;
        let model = DeviceModel::create(
            &db,
            CreateDevice {
                name: name.clone(),
                host: host.clone(),
                port,
                order,
                access_token: access_token.clone(),
            },
        )
        .await
        .unwrap();

        // Ensure the return model is correct
        assert_eq!(model.name, name);
        assert_eq!(model.host, host);
        assert_eq!(model.port, port);
        assert_eq!(model.order, order);
        assert_eq!(model.access_token, access_token);

        // Ensure the copy in the database is correct
        let model = DeviceModel::get_by_id(&db, model.id)
            .await
            .unwrap()
            .expect("model should exist");
        assert_eq!(model.name, name);
        assert_eq!(model.host, host);
        assert_eq!(model.port, port);
        assert_eq!(model.order, order);
        assert_eq!(model.access_token, access_token);
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
                access_token: Some("Test".to_string()),
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
    async fn test_devices_set_access_token() {
        let name = "Test".to_string();
        let host = "127.0.0.1".to_string();
        let port = 8080;
        let order = 0;
        let access_token = Some("Test".to_string());
        let db = mock_database().await;

        // Create a model to work with
        let model = DeviceModel::create(
            &db,
            CreateDevice {
                name: "Test 2".to_string(),
                host: "127.0.0.2".to_string(),
                port: 8081,
                order: 1,
                access_token: Some("Test 2".to_string()),
            },
        )
        .await
        .unwrap();

        // Update the model
        let model = model
            .set_access_token(&db, access_token.clone())
            .await
            .unwrap();

        // Ensure the returned model is correct
        assert_eq!(model.name, name);
        assert_eq!(model.host, host);
        assert_eq!(model.port, port);
        assert_eq!(model.order, order);
        assert_eq!(model.access_token, access_token);

        // Ensure the copy in the database is correct
        let model = DeviceModel::get_by_id(&db, model.id)
            .await
            .unwrap()
            .expect("model should exist");
        assert_eq!(model.name, name);
        assert_eq!(model.host, host);
        assert_eq!(model.port, port);
        assert_eq!(model.order, order);
        assert_eq!(model.access_token, access_token);
    }

    /// Tests that getting a device by ID works
    #[tokio::test]
    async fn test_get_device_by_id() {
        let name = "Test".to_string();
        let host = "127.0.0.1".to_string();
        let port = 8080;
        let order = 0;
        let access_token = Some("Test".to_string());

        let db = mock_database().await;
        let model = DeviceModel::create(
            &db,
            CreateDevice {
                name: name.clone(),
                host: host.clone(),
                port,
                order,
                access_token: access_token.clone(),
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
            ("Test".to_string(), "127.0.0.1".to_string(), 8480, 0, Some("Test 2".to_string())),
            ("Test 1".to_string(), "127.0.0.3".to_string(), 8280, 2, Some("Test 1".to_string())),
            ("Test 2".to_string(), "127.0.0.2".to_string(), 8082, 1, Some("Test 4".to_string())),
            ("Test 3".to_string(), "127.0.0.4".to_string(), 8180, 5, Some("Test 2".to_string())),
            ("Test 4".to_string(), "127.0.0.2".to_string(), 8081, 10, Some("Test 3".to_string())),
        ];

        // Prep an ordered copy of the results
        let mut ordered_data = devices_data.to_vec();
        ordered_data.sort_by(|a, b| a.3.cmp(&b.3));

        let db = mock_database().await;

        // Create the devices
        for (name, host, port, order, access_token) in devices_data {
            DeviceModel::create(
                &db,
                CreateDevice {
                    name: name.clone(),
                    host: host.clone(),
                    port,
                    order,
                    access_token: access_token.clone(),
                },
            )
            .await
            .unwrap();
        }

        let devices = DeviceModel::all(&db).await.unwrap();

        for (index, (name, host, port, order, access_token)) in ordered_data.into_iter().enumerate()
        {
            let model = &devices[index];
            assert_eq!(model.name, name);
            assert_eq!(model.host, host);
            assert_eq!(model.port, port);
            assert_eq!(model.order, order);
            assert_eq!(model.access_token, access_token);
        }
    }
}
