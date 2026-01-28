use leptos::*;

#[component]
pub fn Window(
    #[prop(into)] title: String,
    children: Children,
    #[prop(optional)] class: &'static str,
    #[prop(optional, into)] style: String,
) -> impl IntoView {
    view! {
        <div class=format!("window {}", class) style=style>
            <div class="window-header">
                <span class="title">{title}</span>
                <span class="controls">"[ ] [x]"</span>
            </div>
            <div class="window-content">
                {children()}
            </div>
        </div>
    }
}
