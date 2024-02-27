use leptos::*;

use crate::{server_funcs::items::{add_tag, remove_tag}, data::item::{Tag, TagId}, ui::state::AdminState};

use super::state::SearchQuery;

#[component]
pub fn TagToggle<F>(tag: Tag, remove_tag_cb: F) -> impl IntoView
where
    F: Fn(&TagId) + Copy + 'static
{
    let admin_state = use_context::<ReadSignal<AdminState>>()
        .expect("`AdminState` to be added to the context");

    let remove_tag_action = create_action(move |_| {
        async move {
            remove_tag(tag.id).await?;
            remove_tag_cb(&tag.id);
            Ok::<_, ServerFnError>(())
        }
    });

    let search_params = SearchQuery::use_query();

    let tag_active = {
        let tag_name = tag.name.clone();
        move || !search_params().filter_tags.contains(&tag_name)
    };

    let dot_color = {
        let tag_active = tag_active.clone();
        move || if tag_active() { "green" } else { "red" }
    };

    let toggle_tag = {
        let tag_name = tag.name.clone();
        move |_| {
            let mut search_params = search_params();
            if tag_active() {
                search_params.filter_tags.push(tag_name.clone());
            } else {
                // PANIC: tag must exist
                let idx = search_params.filter_tags.iter().position(|tag_param| tag_param == &tag_name)
                    .unwrap();
                search_params.filter_tags.remove(idx);
            }
            search_params.set();
        }
    };

    view! {
        <div class="flex flex-col gap-1">
            <div
                class="border-solid border-slate-400 border-2 rounded-lg flex flex-row items-center"
                on:click=toggle_tag
            >
                <svg viewBox="0 0 120 120" version="1.1" xmlns="http://www.w3.org/2000/svg" width="10" height="10">
                    <circle cx="60" cy="60" r="50" fill={dot_color}/>
                </svg>
                {tag.name}
            </div>

            {
                move || admin_state().set.then(|| view! {
                    <button
                        on:click=move |_| {
                            remove_tag_action.dispatch(())
                        }
                        class="bg-red-700 disabled:text-slate-400 rounded-xl"
                        disabled=remove_tag_action.pending()
                    >
                        Видалити
                    </button>
                })
            }
        </div>
    }
}

#[component]
pub fn AddTag(add_item_action: Action<String, ()>) -> impl IntoView {
    let (new_tag_name, new_tag_set) = create_signal(String::new());

    view! {
        <div class="flex flex-col items-center border-solid border-black border">
            <input
                class="rounded-lg p-1 border-solid border-slate-400 border"
                type="text"
                required
                on:input=move |ev| {
                    new_tag_set(event_target_value(&ev))
                }

                prop:value=new_tag_name
            />
            <button
                class="bg-green-700 rounded-xl px-2"
                on:click=move |_| {
                    add_item_action.dispatch(new_tag_name())
                }
            >
                "Додати"
            </button>
        </div>
    }
}

#[component]
pub fn TagsBlock(tags: Resource<(), Result<Vec<Tag>, ServerFnError>>) -> impl IntoView {
    let tags_loading = tags.loading();

    let loading = move || view! {
        Завантаження тегів...
    };

    let remove_tag_cb = move |tag_id: &TagId| {
        tags.update(|tags| {
            // PANIC: unwraps are fine, because this action is passed to a component, that is
            //        rendered only after tags have loaded.
            let tags = tags.as_mut().unwrap().as_mut().unwrap();
            // PANIC: tags are rendered from the vec, from which we're removing an item.
            let idx = tags
                .iter()
                .position(|tag| &tag.id == tag_id)
                .unwrap();
            tags.remove(idx);
        })
    };

    let loaded_tags_toggles = move || {
        tags().map(|tags| {
            match tags {
                Ok(tags) => tags.into_iter().map(|tag| {
                    view! {
                        <TagToggle tag remove_tag_cb />
                    }
                }).collect_view(),
                Err(_) => view! { Помилка завантаження тегів }.into_view(),
            }
        })
    };

    let add_tag_action = create_action(move |input: &String| {
        let input = input.clone();
        async move {
            if let Ok(new_tag) = add_tag(input).await {
                tags.update(|tags| {
                    // PANIC: unwraps are fine, because this action is passed to a component, that is
                    //        rendered only after tags have loaded.
                    tags.as_mut().unwrap().as_mut().unwrap().push(new_tag)
                })
            }
        }
    });
    
    let admin_state = use_context::<ReadSignal<AdminState>>()
        .expect("`AdminState` to be added to the context");

    view! {
        <Suspense
            fallback=loading
        >
            <div class="flex flex-col gap-1 p-2">
                {loaded_tags_toggles}
                {
                    move || (!tags_loading() && admin_state().set).then(||
                        view! {
                            <AddTag add_item_action=add_tag_action />
                        }
                    )
                }
            </div>
        </Suspense>
    }
}