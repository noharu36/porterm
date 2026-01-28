use leptos::*;

#[component]
pub fn Placeholder() -> impl IntoView {
    view! {
        <div class="placeholder-content" style="display: flex; align-items: center; justify-content: center; height: 100%; color: var(--gray);">
            <div style="text-align: center;">
                <h3 style="margin: 0;">"Future Work"</h3>
                <p style="font-size: 0.8em;">"Reserved for future projects"</p>
            </div>
        </div>
    }
}
