use uuid::Uuid;
use serde::{Deserialize, Serialize};
use derive_more::{From, Into, FromStr};

#[derive(Clone, Copy, Debug, From, FromStr, Into, Deserialize, Serialize, PartialEq)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type), sqlx(transparent))]
pub struct CategoryId(pub Uuid);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
pub struct Category {
    pub id: CategoryId,
    pub name: String,
}