use leptos::*;

use crate::{data::item::{Item, ItemId, ItemObjectId, ItemObject, Tag, TagId}, server_funcs::items::{search_items, add_item, add_item_object, remove_item, remove_item_object, add_item_tag, remove_item_tag}, ui::state::AdminState};

use super::state::SearchQuery;

#[component]
pub fn ItemObject<RemObjF>(object: ItemObject, remove_object_cb: RemObjF) -> impl IntoView
where
    RemObjF: Fn(&ItemObjectId) + Copy + 'static,
{
    let admin_state = use_context::<ReadSignal<AdminState>>()
        .expect("`AdminState` to be added to the context");

    let remove_object_action = create_action(move |_| {
        async move {
            remove_item_object(object.id).await?;
            remove_object_cb(&object.id);
            Ok::<_, ServerFnError>(())
        }
    });

    view! {
        <div class="flex flex-col gap-1">
            <div
                class="border-solid border-slate-200 border-2 rounded-lg"
            >
                {object.item_code.unwrap_or("Код відсутній".into())}
            </div>

            {
                move || admin_state().set.then(|| view! {
                    <button
                        on:click=move |_| {
                            remove_object_action.dispatch(())
                        }
                        class="bg-red-700 disabled:text-slate-400 rounded-xl"
                        disabled=remove_object_action.pending()
                    >
                        Видалити
                    </button>
                })
            }
        </div>
    }
}

#[component]
pub fn AddObject(add_object_action: Action<String, ()>) -> impl IntoView {
    let (new_object_name, new_object_set) = create_signal(String::new());

    view! {
        <div class="flex flex-col items-center border-solid border-black border">
            <input
                class="rounded-lg p-1 border-solid border-slate-400 border"
                type="text"
                on:input=move |ev| {
                    new_object_set(event_target_value(&ev))
                }

                prop:value=new_object_name
            />
            <button
                class="bg-green-700 rounded-xl px-2"
                on:click=move |_| {
                    add_object_action.dispatch(new_object_name())
                }
            >
                "Додати"
            </button>
        </div>
    }
}

#[component]
pub fn ItemTag<RemTagF>(item_id: ItemId, tag: Tag, remove_tag_cb: RemTagF) -> impl IntoView
where
    RemTagF: Fn(&TagId) + Copy + 'static
{
    let admin_state = use_context::<ReadSignal<AdminState>>()
        .expect("`AdminState` to be added to the context");

    let remove_tag_action = create_action(move |_| {
        async move {
            remove_item_tag(item_id, tag.id).await?;
            remove_tag_cb(&tag.id);
            Ok::<_, ServerFnError>(())
        }
    });

    view! {
        <div class="flex flex-col gap-1">
            <div
                class="border-solid border-slate-400 border-2 rounded-lg"
            >
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
pub fn AddItemTag(tags: Resource<(), Result<Vec<Tag>, ServerFnError>>, item_tags: Vec<Tag>, item_id: ItemId, add_tag_action: Action<(ItemId, Tag), ()>) -> impl IntoView {
    let item_tags = store_value(item_tags);
    let tags_filtered = move || {
        tags().map(move |tags| tags.map(move |tags|
            tags.into_iter()
                .filter(move |tag| !item_tags().iter().any(|item_tag| item_tag.id == tag.id))
        ))
    };
    let (new_tag_id, new_tag_set) = create_signal(tags_filtered().unwrap().unwrap().next().map(|tag| tag.id.to_string()).unwrap_or("".to_string()));

    let tag_options = move || {
        tags_filtered().map(|tags| {
            match tags {
                Ok(tags) => tags
                    .map(|tag| {
                        view! {
                            <option value=tag.id.to_string()>{tag.name}</option>
                        }
                    }).collect_view(),
                Err(_) => view! { Помилка завантаження тегів }.into_view(),
            }
        })
    };

    let loading = move || view! {
        Завантаження тегів...
    };

    view! {
        <Suspense
            fallback=loading
        >
        <div class="flex flex-col items-center border-solid border-black border">
            <select
                on:change=move |ev| {
                    new_tag_set(event_target_value(&ev));
                }
            >
                {tag_options}
            </select>
            <button
                class="bg-green-700 rounded-xl px-2"
                on:click=move |_| {
                    add_tag_action.dispatch((
                        item_id,
                        tags().unwrap().unwrap().into_iter().find(|tag| tag.id == new_tag_id().parse().unwrap()).unwrap()
                    ))
                }
            >
                "Додати"
            </button>
        </div>
        </Suspense>
    }
}

#[component]
pub fn ItemCard<RemItemF, RemObjF, RemTagF>(
    item: Item,
    tags: Resource<(), Result<Vec<Tag>, ServerFnError>>,
    remove_item_cb: RemItemF,
    add_object_action: Action<String, ()>,
    remove_object_cb: RemObjF,
    add_tag_action: Action<(ItemId, Tag), ()>,
    remove_tag_cb: RemTagF,
) -> impl IntoView
where
    RemItemF: Fn(&ItemId) + Copy + 'static,
    RemObjF: Fn(&ItemObjectId) + Copy + 'static,
    RemTagF: Fn(&TagId) + Copy + 'static,
{
    let admin_state = use_context::<ReadSignal<AdminState>>()
        .expect("`AdminState` to be added to the context");

    let tags_view = item.tags.clone().into_iter().map(|tag| {
        view! {
            <ItemTag item_id=item.id tag remove_tag_cb />
        }
    }).collect_view();

    let objects_view = item.objects.clone().into_iter().map(|object| {
        view! {
            <ItemObject object remove_object_cb />
        }
    }).collect_view();

    let remove_item_action = create_action(move |_| {
        async move {
            remove_item(item.id).await?;
            remove_item_cb(&item.id);
            Ok::<_, ServerFnError>(())
        }
    });

    view! {
        <div class="flex flex-col gap-1">
            <div class="border-2 border-solid border-blue-700 rounded-xl">
                <div class="flex flex-row gap-2">
                    <div>"Назва:"</div>
                    <div>{item.name}</div>
                </div>
                <div class="flex flex-row gap-2">
                    <div>Теги:</div>
                    <div class="flex flex-row gap-1">{tags_view}</div>
                    {
                        move || admin_state().set.then(||
                            view! {
                                <AddItemTag tags item_tags=item.tags.clone() item_id=item.id add_tag_action />
                            }
                        )
                    }
                </div>
                <div class="flex flex-row gap-2">
                    <div>Наявні предмети:</div>
                    <div class="flex flex-row gap-1">{objects_view}</div>
                    {
                        move || admin_state().set.then(||
                            view! {
                                <AddObject add_object_action />
                            }
                        )
                    }
                </div>
            </div>

            {
                move || admin_state().set.then(|| view! {
                    <button
                        on:click=move |_| {
                            remove_item_action.dispatch(())
                        }
                        class="bg-red-700 disabled:text-slate-400 rounded-xl"
                        disabled=remove_item_action.pending()
                    >
                        Видалити
                    </button>
                })
            }
        </div>
    }
}

#[component]
pub fn AddItem(add_item_action: Action<String, ()>) -> impl IntoView {
    let (new_item_name, new_item_set) = create_signal(String::new());

    view! {
        <div class="flex flex-col items-center border-solid border-black border">
            <input
                class="rounded-lg p-1 border-solid border-slate-400 border"
                type="text"
                on:input=move |ev| {
                    new_item_set(event_target_value(&ev))
                }

                prop:value=new_item_name
            />
            <button
                class="bg-green-700 rounded-xl px-2"
                on:click=move |_| {
                    add_item_action.dispatch(new_item_name())
                }
            >
                "Додати"
            </button>
        </div>
    }
}

#[component]
pub fn Items(tags: Resource<(), Result<Vec<Tag>, ServerFnError>>) -> impl IntoView {
    let search_query = SearchQuery::use_query();
    let items_resource = create_resource(
        search_query,
        |search_query| {
            async move {
                // If category isn't set - we don't load anything yet
                let Some(category) = search_query.category else { return Ok(vec![]) };
                search_items(
                    search_query.q,
                    search_query.filter_tags,
                    category
                ).await
            }
        }
    );
    let items_loading = items_resource.loading();

    let loading = move || view! {
        Завантаження товарів...
    };

    let remove_item_cb = move |item_id: &ItemId| {
        items_resource.update(|items| {
            // PANIC: unwraps are fine, because this action is passed to a component, that is
            //        rendered only after items have loaded.
            let items = items.as_mut().unwrap().as_mut().unwrap();
            // PANIC: items are rendered from the vec, from which we're removing an item.
            let idx = items
                .iter()
                .position(|item| &item.id == item_id)
                .unwrap();
            items.remove(idx);
        })
    };

    let loaded_items = move || {
        items_resource().map(|items| {
            match items {
                Ok(items) => items.into_iter().map(|item| {
                    let add_object_action = create_action(move |input: &String| {
                        let input = input.clone();

                        async move {
                            if let Ok(new_object) = add_item_object(item.id, input).await {
                                items_resource.update(|items| {
                                    // PANIC: unwraps are fine, because this action is passed to a component, that is
                                    //        rendered only after items have loaded.
                                    items.as_mut().unwrap().as_mut().unwrap()
                                        .iter_mut().skip_while(|search_item| search_item.id != item.id)
                                        .next().unwrap().objects.push(new_object);
                                })
                            }
                        }
                    });

                    let remove_object_cb = move |item_object_id: &ItemObjectId| {
                        items_resource.update(|items| {
                            // PANIC: unwraps are fine, because this action is passed to a component, that is
                            //        rendered only after items have loaded.
                            let item = items.as_mut().unwrap().as_mut().unwrap()
                                .iter_mut().skip_while(|search_item| search_item.id != item.id)
                                .next().unwrap();
                            // PANIC: items are rendered from the vec, from which we're removing an item.
                            let idx = item.objects
                                .iter()
                                .position(|item_object| &item_object.id == item_object_id)
                                .unwrap();
                            item.objects.remove(idx);
                        })
                    };

                    let add_tag_action = create_action(move |input: &(ItemId, Tag)| {
                        let (item_id, tag) = input.clone();

                        async move {
                            if let Ok(_) = add_item_tag(item_id, tag.id).await {
                                items_resource.update(|items| {
                                    // PANIC: unwraps are fine, because this action is passed to a component, that is
                                    //        rendered only after items have loaded.
                                    items.as_mut().unwrap().as_mut().unwrap()
                                        .iter_mut().skip_while(|search_item| search_item.id != item.id)
                                        .next().unwrap().tags.push(tag);
                                })
                            }
                        }
                    });

                    let remove_tag_cb = move |tag_id: &TagId| {
                        items_resource.update(|items| {
                            // PANIC: unwraps are fine, because this action is passed to a component, that is
                            //        rendered only after items have loaded.
                            let item = items.as_mut().unwrap().as_mut().unwrap()
                                .iter_mut().skip_while(|search_item| search_item.id != item.id)
                                .next().unwrap();
                            // PANIC: items are rendered from the vec, from which we're removing an item.
                            let idx = item.tags
                                .iter()
                                .position(|tag| &tag.id == tag_id)
                                .unwrap();
                            item.tags.remove(idx);
                        })
                    };

                    view! {
                        <ItemCard item tags remove_item_cb add_object_action remove_object_cb add_tag_action remove_tag_cb />
                    }
                }).collect_view(),
                Err(_) => view! { Помилка завантаження продуктів }.into_view(),
            }
        })
    };

    let add_item_action = create_action(move |input: &String| {
        let input = input.clone();
        // Category must be chosen in AddItem component
        let category = search_query().category.unwrap();
        async move {
            if let Ok(new_item) = add_item(input, category).await {
                items_resource.update(|items| {
                    // PANIC: unwraps are fine, because this action is passed to a component, that is
                    //        rendered only after items have loaded.
                    items.as_mut().unwrap().as_mut().unwrap().push(new_item)
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
            <div class="grid gap-2 grid-cols-2 w-full h-max">
                {loaded_items}
                {
                    move || (!items_loading() && admin_state().set).then(||
                        view! {
                            <AddItem add_item_action />
                        }
                    )
                }
            </div>
        </Suspense>
    }
}