use crate::data::categories::{Category, CategoryId};

use super::{ResultDb, Repository};

#[async_trait::async_trait]
pub trait CategoryDB {
    async fn get_categories(&self) -> ResultDb<Vec<Category>>;
    async fn add_category(&self, category_name: &str) -> ResultDb<Category>;
    async fn remove_category(&self, category_id: CategoryId) -> ResultDb<()>;
}

#[async_trait::async_trait]
impl CategoryDB for Repository {
    async fn get_categories(&self) -> ResultDb<Vec<Category>> {
        Ok(sqlx::query_as!(
            Category,
            "
                SELECT id, name
                FROM category
            "
        )
        .fetch_all(&self.pool)
        .await?)
    }

    async fn add_category(&self, category_name: &str) -> ResultDb<Category> {
        Ok(sqlx::query_as!(
            Category,
            "
                INSERT INTO category (name)
                VALUES ($1)
                RETURNING id, name
            ",
            category_name
        )
        .fetch_one(&self.pool)
        .await?)
    }

    async fn remove_category(&self, category_id: CategoryId) -> ResultDb<()> {
        sqlx::query!(
            "
                DELETE FROM category
                WHERE id = $1
            ",
            category_id as _
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
