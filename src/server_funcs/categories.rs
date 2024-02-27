use leptos::{server, ServerFnError};

use crate::data::categories::{Category, CategoryId};

#[server(AddCategory, "/api")]
pub async fn add_category(category_name: String) -> Result<Category, ServerFnError> {
    use leptos_actix::extract;
    use crate::db::{Repository, categories::CategoryDB};

    Ok(extract(|db: Repository| async move {
        db.add_category(&category_name).await
    }).await??)
}

#[server(GetCategories, "/api", "GetJson")]
pub async fn get_categories() -> Result<Vec<Category>, ServerFnError> {
    use leptos_actix::extract;
    use crate::db::{Repository, categories::CategoryDB};

    Ok(extract(|db: Repository| async move {
        db.get_categories().await
    }).await??)
}

#[server(RemoveCategory, "/api")]
pub async fn remove_category(category_id: CategoryId) -> Result<(), ServerFnError> {
    use leptos_actix::extract;
    use crate::db::{Repository, categories::CategoryDB};

    Ok(extract(move |db: Repository| async move {
        db.remove_category(category_id).await
    }).await??)
}