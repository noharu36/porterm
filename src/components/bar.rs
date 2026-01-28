use chrono;
use leptos::*;

#[component]
pub fn Bar(
    active_id: ReadSignal<usize>,
    #[prop(into)] on_switch: Callback<usize>,
) -> impl IntoView {
    let buttons = vec![
        (1, "1: about"),
        (2, "2: github"),
        (3, "3: contact"),
        (4, "4: future"),
    ];

    view! {
        <div class="bar">
            {buttons.into_iter().map(|(id, label)| {
                view! {
                    <button
                        class="bar-item"
                        class:active=move || active_id.get() == id
                        on:click=move |_| on_switch.call(id)
                    >
                        {label}
                    </button>
                }
            }).collect_view()}
            <div class="bar-clock">
                {chrono::Local::now().format("%a %d %b %H:%M").to_string()}
            </div>
        </div>
    }
}
