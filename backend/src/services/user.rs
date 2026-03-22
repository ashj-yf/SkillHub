use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user::User;
use crate::repos::user::UserRepo;

pub struct UserService {
    user_repo: UserRepo,
}

impl UserService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            user_repo: UserRepo::new(pool),
        }
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<User>> {
        self.user_repo.find_by_id(id).await
    }

    pub async fn get_by_email(&self, email: &str) -> Result<Option<User>> {
        self.user_repo.find_by_email(email).await
    }

    pub async fn get_by_username(&self, username: &str) -> Result<Option<User>> {
        self.user_repo.find_by_username(username).await
    }

    pub async fn deactivate(&self, id: Uuid) -> Result<()> {
        self.user_repo.set_active(id, false).await
    }

    pub async fn activate(&self, id: Uuid) -> Result<()> {
        self.user_repo.set_active(id, true).await
    }
}