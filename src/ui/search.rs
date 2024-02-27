use leptos::*;

use super::state::SearchQuery;

#[component]
pub fn SearchBlock() -> impl IntoView {
    let search_query = SearchQuery::use_query();
    let search_query_untracked = SearchQuery::use_query_untracked();
    let (search, search_set) = create_signal(search_query_untracked().q.unwrap_or_default());

    let update_search_query = move |_| {
        let mut search_query = search_query();
        search_query.q = Some(search());
        search_query.set();
    };

    view! {
        <div class="mx-auto max-w-max">
            <input
                class="rounded-lg p-1 border border-solid border-black"
                type="text"
                on:input=move |ev| {
                    search_set(event_target_value(&ev))
                }

                prop:value=search
            />
            <button
                class="bg-slate-400 rounded-xl px-2"
                on:click=update_search_query
            >
                "Пошук"
            </button>
        </div>
    }
}