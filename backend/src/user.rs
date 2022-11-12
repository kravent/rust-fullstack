use anyhow::Result;
use async_trait::async_trait;
use common::{User, UserInput};
use shaku::{Component, Interface};
use std::{sync::Arc, time::Duration};
use tokio::{sync::Mutex, time::sleep};

#[async_trait]
pub trait UserRepository: Interface {
    async fn get_users(&self) -> Result<Vec<User>>;
    async fn create_user(&self, input: UserInput) -> Result<()>;
}

#[derive(Clone, Component)]
#[shaku(interface = UserRepository)]
pub struct UserRepositoryImpl {
    #[shaku(default)]
    users: Arc<Mutex<Vec<User>>>,
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn get_users(&self) -> Result<Vec<User>> {
        let users = self.users.lock().await;
        sleep(Duration::from_secs(2)).await;
        Ok(users.iter().cloned().collect())
    }

    async fn create_user(&self, input: UserInput) -> Result<()> {
        let mut users = self.users.lock().await;
        sleep(Duration::from_secs(1)).await;
        let id = users.len() as u32;
        users.push(User {
            id,
            name: input.name,
            password: input.password,
        });
        Ok(())
    }
}
