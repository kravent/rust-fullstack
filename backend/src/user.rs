use anyhow::Result;
use common::{User};
use std::{sync::Arc, time::Duration};
use tokio::{sync::Mutex, time::sleep};

#[derive(Clone)]
pub struct UserRepository {
    users: Arc<Mutex<Vec<User>>>,
}

impl Default for UserRepository {
    fn default() -> Self {
        UserRepository {
            users: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl UserRepository {
    pub async fn get_users(&self) -> Result<Vec<User>> {
        let users = self.users.lock().await;
        sleep(Duration::from_secs(2)).await;
        Ok(users.iter().cloned().collect())
    }
}
