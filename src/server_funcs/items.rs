use leptos::{server, ServerFnError};

use crate::data::item::{TagId, Item, Tag, ItemId, ItemObject, ItemObjectId};

#[server(SearchItems, "/api", "GetJson")]
pub async fn search_items(query: Option<String>, #[server(default)] tags_filtered: Vec<String>, category: String) -> Result<Vec<Item>, ServerFnError> {
    use leptos_actix::extract;
    use crate::db::{Repository, item::ItemsDB};

    Ok(extract(move |db: Repository| async move {
        db.search_items(query.as_deref(), &tags_filtered, &category).await
    }).await??)
}

#[server(AddTag, "/api")]
pub async fn add_tag(tag_name: String) -> Result<Tag, ServerFnError> {
    use leptos_actix::extract;
    use crate::db::{Repository, item::ItemsDB};

    Ok(extract(move |db: Repository| async move {
        db.add_tag(&tag_name).await
    }).await??)
}

#[server(AddItem, "/api")]
pub async fn add_item(item_name: String, item_category: String) -> Result<Item, ServerFnError> {
    use leptos_actix::extract;
    use crate::db::{Repository, item::ItemsDB};

    Ok(extract(move |db: Repository| async move {
        db.add_item(&item_name, &item_category).await
    }).await??)
}

#[server(AddItemObject, "/api")]
pub async fn add_item_object(item_id: ItemId, item_code: String) -> Result<ItemObject, ServerFnError> {
    use leptos_actix::extract;
    use crate::db::{Repository, item::ItemsDB};

    Ok(extract(move |db: Repository| async move {
        db.add_item_object(item_id, &item_code).await
    }).await??)
}

#[server(GetTags, "/api", "GetJson")]
pub async fn get_tags() -> Result<Vec<Tag>, ServerFnError> {
    use leptos_actix::extract;
    use crate::db::{Repository, item::ItemsDB};

    Ok(extract(move |db: Repository| async move {
        db.get_tags().await
    }).await??)
}

#[server(RemoveTag, "/api")]
pub async fn remove_tag(tag_id: TagId) -> Result<(), ServerFnError> {
    use leptos_actix::extract;
    use crate::db::{Repository, item::ItemsDB};

    Ok(extract(move |db: Repository| async move {
        db.remove_tag(tag_id).await
    }).await??)
}

#[server(RemoveItem, "/api")]
pub async fn remove_item(item_id: ItemId) -> Result<(), ServerFnError> {
    use leptos_actix::extract;
    use crate::db::{Repository, item::ItemsDB};

    Ok(extract(move |db: Repository| async move {
        db.remove_item(item_id).await
    }).await??)
}

#[server(RemoveItemObject, "/api")]
pub async fn remove_item_object(item_object_id: ItemObjectId) -> Result<(), ServerFnError> {
    use leptos_actix::extract;
    use crate::db::{Repository, item::ItemsDB};

    Ok(extract(move |db: Repository| async move {
        db.remove_item_object(item_object_id).await
    }).await??)
}

#[server(AddItemTag, "/api")]
pub async fn add_item_tag(item_id: ItemId, tag_id: TagId) -> Result<(), ServerFnError> {
    use leptos_actix::extract;
    use crate::db::{Repository, item::ItemsDB};

    Ok(extract(move |db: Repository| async move {
        db.add_item_tag(item_id, tag_id).await
    }).await??)
}

#[server(RemoveItemTag, "/api")]
pub async fn remove_item_tag(item_id: ItemId, tag_id: TagId) -> Result<(), ServerFnError> {
    use leptos_actix::extract;
    use crate::db::{Repository, item::ItemsDB};

    Ok(extract(move |db: Repository| async move {
        db.remove_item_tag(item_id, tag_id).await
    }).await??)
}