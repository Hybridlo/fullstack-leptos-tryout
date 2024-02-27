use leptos::*;

use super::state::AdminState;

#[component]
pub fn AdminChanger(admin_state_setter: WriteSignal<AdminState>) -> impl IntoView {
    let admin_state = use_context::<ReadSignal<AdminState>>()
        .expect("`AdminState` to be added to the context");

    let admin_view = move || {
        if admin_state().set {
            view! {
                <button
                    class="text-xl p-2 rounded-xl bg-green-700 text-black mx-2"
                    on:click=move |_| admin_state_setter(AdminState { set: false })
                >
                    Admin ON
                </button>
            }
        } else {
            view! {
                <button
                    class="text-xl p-2 rounded-xl bg-red-700 text-white mx-2"
                    on:click=move |_| admin_state_setter(AdminState { set: true })
                >
                    Admin OFF
                </button>
            }
        }
    };

    view! {
        {admin_view}
    }
}