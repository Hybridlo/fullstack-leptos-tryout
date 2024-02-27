use serde::{Serialize, Deserialize};
use uuid::Uuid;
use derive_more::{From, FromStr, Into, Display};

use super::categories::Category;

#[derive(Clone, Copy, Display, Debug, From, FromStr, Into, Deserialize, Serialize, PartialEq)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type), sqlx(transparent))]
pub struct TagId(pub Uuid);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
pub struct Tag {
    pub id: TagId,
    pub name: String,
}

#[derive(Clone, Copy, Debug, From, Into, Deserialize, Serialize, PartialEq)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type), sqlx(transparent))]
pub struct ItemObjectId(pub Uuid);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
pub struct ItemObject {
    pub id: ItemObjectId,
    pub item_code: Option<String>,
}

#[derive(Clone, Copy, Debug, From, FromStr, Into, Deserialize, Serialize, PartialEq)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type), sqlx(transparent))]
pub struct ItemId(pub Uuid);

pub struct ItemIncomplete {
    pub id: ItemId,
    pub name: String,
    pub category: Category
}

//#[derive(sqlx::Type)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Item {
    pub id: ItemId,
    pub name: String,
    pub category: Category,
    pub tags: Vec<Tag>,
    pub objects: Vec<ItemObject>,
}

/// A workaround module for the [`sqlx::Type`]/[`sqlx::Decode`] derive, which breaks because of compiler bug.
/// 
/// Related issue: https://github.com/launchbadge/sqlx/issues/1031
/// 
/// Also an impl of [`sqlx::postgres::PgHasArrayType`] for Tag and ItemObject.
#[cfg(feature = "ssr")]
mod derive_workaround {
    use sqlx::postgres::PgHasArrayType;

    use crate::data::categories::Category;

    use super::{Item, ItemId, ItemObject, Tag};

    impl PgHasArrayType for Tag {
        fn array_type_info() -> sqlx::postgres::PgTypeInfo {
            sqlx::postgres::PgTypeInfo::with_name("_tag")
        }
    }

    impl PgHasArrayType for ItemObject {
        fn array_type_info() -> sqlx::postgres::PgTypeInfo {
            sqlx::postgres::PgTypeInfo::with_name("_item_object")
        }
    }

    impl ::sqlx::encode::Encode<'_, ::sqlx::Postgres> for Item
    where
        ItemId: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
        ItemId: ::sqlx::types::Type<::sqlx::Postgres>,
        String: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
        String: ::sqlx::types::Type<::sqlx::Postgres>,
        Category: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
        Category: ::sqlx::types::Type<::sqlx::Postgres>,
        Vec<Tag>: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
        Vec<Tag>: ::sqlx::types::Type<::sqlx::Postgres>,
        Vec<ItemObject>: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
        Vec<ItemObject>: ::sqlx::types::Type<::sqlx::Postgres>,
    {
        fn encode_by_ref(
            &self,
            buf: &mut ::sqlx::postgres::PgArgumentBuffer,
        ) -> ::sqlx::encode::IsNull {
            let mut encoder = ::sqlx::postgres::types::PgRecordEncoder::new(buf);
            encoder.encode(&self.id);
            encoder.encode(&self.name);
            encoder.encode(&self.category);
            encoder.encode(&self.tags);
            encoder.encode(&self.objects);
            encoder.finish();
            ::sqlx::encode::IsNull::No
        }
        fn size_hint(&self) -> ::std::primitive::usize {
            5usize * (4 + 4)
                + <ItemId as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.id)
                + <String as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.name)
                + <Category as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.category)
                + <Vec<Tag> as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.tags)
                + <Vec<ItemObject> as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(
                    &self.objects,
                )
        }
    }
    
    impl<'r> ::sqlx::decode::Decode<'r, ::sqlx::Postgres> for Item
    where
        ItemId: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
        ItemId: ::sqlx::types::Type<::sqlx::Postgres>,
        //String: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
        String: ::sqlx::types::Type<::sqlx::Postgres>,
        Category: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
        Category: ::sqlx::types::Type<::sqlx::Postgres>,
        Vec<Tag>: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
        Vec<Tag>: ::sqlx::types::Type<::sqlx::Postgres>,
        Vec<ItemObject>: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
        Vec<ItemObject>: ::sqlx::types::Type<::sqlx::Postgres>,
    {
        fn decode(
            value: ::sqlx::postgres::PgValueRef<'r>,
        ) -> ::std::result::Result<
            Self,
            ::std::boxed::Box<
                dyn ::std::error::Error + 'static + ::std::marker::Send + ::std::marker::Sync,
            >,
        > {
            let mut decoder = ::sqlx::postgres::types::PgRecordDecoder::new(value)?;
            let id = decoder.try_decode::<ItemId>()?;
            let name = decoder.try_decode::<String>()?;
            let category = decoder.try_decode::<Category>()?;
            let tags = decoder.try_decode::<Vec<Tag>>()?;
            let objects = decoder.try_decode::<Vec<ItemObject>>()?;
            ::std::result::Result::Ok(Item {
                id,
                name,
                category,
                tags,
                objects,
            })
        }
    }
    
    impl ::sqlx::Type<::sqlx::Postgres> for Item {
        fn type_info() -> ::sqlx::postgres::PgTypeInfo {
            ::sqlx::postgres::PgTypeInfo::with_name("Item")
        }
    }
}