use leptos::{*, html::P};

use crate::{server_funcs::categories::{add_category, get_categories, remove_category}, data::categories::{Category, CategoryId}, ui::state::AdminState};

use super::state::SearchQuery;

#[component]
pub fn CategoryButton<F>(category: Category, remove_category_cb: F) -> impl IntoView
where
    F: Fn(&CategoryId) + Copy + 'static
{
    let admin_state = use_context::<ReadSignal<AdminState>>()
        .expect("`AdminState` to be added to the context");

    let search_query = SearchQuery::use_query();
    let update_category = {
        let category_name = category.name.clone();
        move |_| {
            let mut search_query = search_query();
            search_query.category = Some(category_name.clone());
            search_query.set();
        }
    };
    let is_category_chosen = {
        let category_name = category.name.clone();
        move || search_query().category == Some(category_name.clone())
    };
    let is_category_not_chosen = {
        let is_category_chosen = is_category_chosen.clone();
        move || !is_category_chosen()
    };

    let remove_category_action = create_action(move |_| {
        async move {
            remove_category(category.id).await?;
            remove_category_cb(&category.id);
            Ok::<_, ServerFnError>(())
        }
    });

    view! {
        <div class="flex flex-col gap-1">
            <button
                class="text-xl p-2 rounded-xl border-solid border-blue-700 border-4"
                class=("bg-slate-200", is_category_not_chosen)
                class=("bg-blue-200", is_category_chosen.clone())
                on:click=update_category
                disabled=is_category_chosen.clone()
            >
                {category.name}
            </button>

            {
                move || admin_state().set.then(|| view! {
                    <button
                        on:click=move |_| {
                            remove_category_action.dispatch(())
                        }
                        class="bg-red-700 disabled:text-slate-400 rounded-xl"
                        disabled=remove_category_action.pending()
                    >
                        Видалити
                    </button>
                })
            }
        </div>
    }
}

#[component]
pub fn AddCategory(add_item_action: Action<String, ()>) -> impl IntoView {
    let (new_category_name, new_category_set) = create_signal(String::new());

    view! {
        <div class="flex flex-col items-center">
            <input
                class="rounded-lg p-1"
                type="text"
                on:input=move |ev| {
                    new_category_set(event_target_value(&ev))
                }

                prop:value=new_category_name
            />
            <button
                class="bg-green-700 rounded-xl px-2"
                on:click=move |_| {
                    add_item_action.dispatch(new_category_name())
                }
            >
                "Додати"
            </button>
        </div>
    }
}

#[component]
pub fn CategoriesBlock() -> impl IntoView {
    let categories = create_resource(|| (), |_| get_categories());
    let categories_loading = categories.loading();

    let loading = move || view! {
        Завантаження категорій...
    };

    let remove_category_cb = move |category_id: &CategoryId| {
        categories.update(|categories| {
            // PANIC: unwraps are fine, because this action is passed to a component, that is
            //        rendered only after categories have loaded.
            let categories = categories.as_mut().unwrap().as_mut().unwrap();
            // PANIC: categories are rendered from the vec, from which we're removing an item.
            let idx = categories
                .iter()
                .position(|category| &category.id == category_id)
                .unwrap();
            categories.remove(idx);
        })
    };

    let loaded_category_buttons = move || {
        categories().map(|categories| {
            match categories {
                Ok(categories) => categories.into_iter().map(|category| {
                    view! {
                        <CategoryButton category remove_category_cb/>
                    }
                }).collect_view(),
                Err(_) => view! { Помилка завантаження категорій }.into_view(),
            }
        })
    };

    let add_category_action = create_action(move |input: &String| {
        let input = input.clone();
        async move {
            if let Ok(new_category) = add_category(input).await {
                categories.update(|categories| {
                    // PANIC: unwraps are fine, because this action is passed to a component, that is
                    //        rendered only after categories have loaded.
                    categories.as_mut().unwrap().as_mut().unwrap().push(new_category)
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
            <div class="flex flex-row items-center gap-2 bg-blue-400 shadow-lg shadow-blue-300/50 p-2">
                {loaded_category_buttons}
                {
                    move || (!categories_loading() && admin_state().set).then(||
                        view! {
                            <AddCategory add_item_action=add_category_action />
                        }
                    )
                }
            </div>
        </Suspense>
    }
}