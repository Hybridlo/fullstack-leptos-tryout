use futures::{StreamExt, TryStreamExt};

use crate::data::{item::{TagId, Item, Tag, ItemId, ItemObject, ItemIncomplete, ItemObjectId}, categories::Category};

use super::{ResultDb, Repository};

impl ItemIncomplete {
    pub async fn fetch_related(self, repo: &impl ItemsDB) -> ResultDb<Item> {
        let objects = repo.get_item_objects(self.id).await?;
        let tags = repo.get_item_tags(self.id).await?;

        Ok(Item {
            id: self.id,
            name: self.name,
            category: self.category,
            tags,
            objects,
        })
    }
}

#[async_trait::async_trait]
pub trait ItemsDB {
    async fn search_items(&self, query: Option<&str>, tags_filtered: &[String], category: &str) -> ResultDb<Vec<Item>>;
    async fn add_tag(&self, tag_name: &str) -> ResultDb<Tag>;
    async fn add_item(&self, item_name: &str, item_category: &str) -> ResultDb<Item>;
    async fn add_item_object(&self, item_id: ItemId, item_code: &str) -> ResultDb<ItemObject>;
    async fn get_tags(&self) -> ResultDb<Vec<Tag>>;
    async fn remove_tag(&self, tag_id: TagId) -> ResultDb<()>;
    async fn get_item_objects(&self, item_id: ItemId) -> ResultDb<Vec<ItemObject>>;
    async fn get_item_tags(&self, item_id: ItemId) -> ResultDb<Vec<Tag>>;
    async fn remove_item(&self, item_id: ItemId) -> ResultDb<()>;
    async fn remove_item_object(&self, item_object_id: ItemObjectId) -> ResultDb<()>;
    async fn add_item_tag(&self, item_id: ItemId, tag_id: TagId) -> ResultDb<()>;
    async fn remove_item_tag(&self, item_id: ItemId, tag_id: TagId) -> ResultDb<()>;
}

#[async_trait::async_trait]
impl ItemsDB for Repository {
    async fn search_items(&self, query: Option<&str>, tags_filtered: &[String], category: &str) -> ResultDb<Vec<Item>> {
        Ok(futures::stream::iter(
            sqlx::query_as!(
                ItemIncomplete,
                r#"
                    WITH items_ids_with_tags AS (
                        SELECT
                            item_tag.item_id
                        FROM
                            item_tag
                        LEFT JOIN
                            tag ON tag.id = item_tag.tag_id
                        WHERE
                            tag.name in (SELECT unnest($2::text[]))
                    )
                    
                    SELECT
                        item.id, item.name,
                        (category.id,  category.name) as "category!: Category"
                    FROM
                        item

                    INNER JOIN
                        category ON category.id = item.category_id

                    WHERE
                        item.name LIKE '%' || $1 || '%'
                    AND
                        NOT item.id in (SELECT item_id from items_ids_with_tags)
                    AND
                        category.name = $3
                "#,
                query.unwrap_or(""),
                tags_filtered,
                category
            )
            .fetch_all(&self.pool)
            .await?
        )
        .map(|item| item.fetch_related(self))
        .buffer_unordered(10)
        .try_collect()
        .await?)
    }

    async fn add_tag(&self, tag_name: &str) -> ResultDb<Tag> {
        Ok(sqlx::query_as!(
            Tag,
            "
                INSERT INTO tag (name)
                VALUES ($1)
                RETURNING id, name
            ",
            tag_name
        )
        .fetch_one(&self.pool)
        .await?)
    }

    async fn add_item(&self, item_name: &str, item_category: &str) -> ResultDb<Item> {
        Ok(sqlx::query_as!(
            ItemIncomplete,
            r#"
                WITH inserted_items AS (
                    INSERT INTO item (name, category_id)
                    SELECT $1, category.id
                    FROM category
                    WHERE category.name = $2
                    RETURNING item.id, item.name, item.category_id
                )

                SELECT
                    inserted_items.id,
                    inserted_items.name,
                    (
                        category.id,
                        category.name
                    ) as "category!: Category"
                FROM
                    inserted_items
                INNER JOIN
                    category ON category.id = inserted_items.category_id
            "#,
            item_name,
            item_category
        )
        .fetch_one(&self.pool)
        .await?
        .fetch_related(self)
        .await?)
    }

    async fn add_item_object(&self, item_id: ItemId, item_code: &str) -> ResultDb<ItemObject> {
        Ok(sqlx::query_as!(
            ItemObject,
            r#"
                INSERT INTO item_objects (item_code, item_id)
                VALUES ($1, $2)
                RETURNING id, item_code
            "#,
            item_code,
            item_id as _
        )
        .fetch_one(&self.pool)
        .await?)
    }

    async fn get_tags(&self) -> ResultDb<Vec<Tag>> {
        Ok(sqlx::query_as!(
            Tag,
            "
                SELECT id, name
                FROM tag
            "
        )
        .fetch_all(&self.pool)
        .await?)
    }

    async fn remove_tag(&self, tag_id: TagId) -> ResultDb<()> {
        sqlx::query!(
            "
                DELETE FROM tag
                WHERE id = $1
            ",
            tag_id as _
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    async fn get_item_objects(&self, item_id: ItemId) -> ResultDb<Vec<ItemObject>> {
        Ok(sqlx::query_as!(
            ItemObject,
            "
                SELECT item_objects.id, item_objects.item_code
                FROM item_objects
                WHERE item_objects.item_id = $1
            ",
            item_id as _
        )
        .fetch_all(&self.pool)
        .await?)
    }

    async fn get_item_tags(&self, item_id: ItemId) -> ResultDb<Vec<Tag>> {
        Ok(sqlx::query_as!(
            Tag,
            "
                SELECT tag.id, tag.name
                FROM tag
                LEFT JOIN item_tag ON item_tag.tag_id = tag.id
                WHERE item_tag.item_id = $1
            ",
            item_id as _
        )
        .fetch_all(&self.pool)
        .await?)
    }

    async fn remove_item(&self, item_id: ItemId) -> ResultDb<()> {
        sqlx::query!(
            "
                DELETE FROM item
                WHERE id = $1
            ",
            item_id as _
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    async fn remove_item_object(&self, item_object_id: ItemObjectId) -> ResultDb<()> {
        sqlx::query!(
            "
                DELETE FROM item_objects
                WHERE id = $1
            ",
            item_object_id as _
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn add_item_tag(&self, item_id: ItemId, tag_id: TagId) -> ResultDb<()> {
        sqlx::query!(
            "
                INSERT INTO item_tag (item_id, tag_id)
                VALUES ($1, $2)
            ",
            item_id as _,
            tag_id as _
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn remove_item_tag(&self, item_id: ItemId, tag_id: TagId) -> ResultDb<()> {
        sqlx::query!(
            "
                DELETE FROM item_tag
                WHERE item_id = $1 AND tag_id = $2
            ",
            item_id as _,
            tag_id as _,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
