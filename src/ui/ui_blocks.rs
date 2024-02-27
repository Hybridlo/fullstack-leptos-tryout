use leptos::{component, WriteSignal, IntoView, view, create_resource, Resource, ServerFnError};

use crate::{ui::{tags::TagsBlock, items::Items}, server_funcs::items::get_tags, data::item::Tag};

use super::{state::AdminState, admin_changer::AdminChanger};

#[component]
pub fn TopBlock(admin_state_setter: WriteSignal<AdminState>) -> impl IntoView {
    view! {
        <div class="grid gap-4 grid-cols-3">
            <div></div>

            <div class="mx-auto max-w-max bg-slate-200 rounded-xl shadow-lg my-3">
                <h1 class="text-center text-4xl py-3">База даних магазину</h1>
            </div>

            <div class="m-auto">
                <AdminChanger admin_state_setter />
            </div>
        </div>
    }
}

#[component]
pub fn MainBlock() -> impl IntoView {
    let tags = create_resource(|| (), |_| get_tags());
    view! {
        <div class="flex flex-row">
            <LeftBlock tags />
            <Items tags />
        </div>
    }
}

#[component]
pub fn LeftBlock(tags: Resource<(), Result<Vec<Tag>, ServerFnError>>) -> impl IntoView {
    view! {
        <TagsBlock tags />
    }
}