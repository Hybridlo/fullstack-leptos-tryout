use leptos::{Memo, create_memo, SignalGetUntracked, SignalGet};
use leptos_router::{NavigateOptions, State};
use serde::{Serialize, Deserialize};

#[derive(Clone, Default)]
pub struct AdminState {
    pub set: bool
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub category: Option<String>,
    #[serde(default, skip_serializing_if="Vec::is_empty")]
    pub filter_tags: Vec<String>
}

impl SearchQuery {
    pub fn use_query() -> impl Fn() -> SearchQuery + Copy {
        let location = leptos_router::use_location();
        move || serde_qs::from_str(&(location.search)()).expect("Any querystring to be parsable")
    }

    pub fn use_query_untracked() -> impl Fn() -> SearchQuery + Copy {
        let location = leptos_router::use_location();
        move || serde_qs::from_str(&(location.search).get_untracked()).expect("Any querystring to be parsable")
    }

    pub fn set(&self) {
        let path = leptos_router::use_location().pathname.get_untracked();
        let navigate = leptos_router::use_navigate();
        navigate(
            &(path + "?" + &serde_qs::to_string(self).expect("SearchQuery to be serializable")),
            NavigateOptions { resolve: true, replace: true, scroll: true, state: State(None) }
        );
    }
}