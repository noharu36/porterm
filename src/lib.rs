use leptos::*;
use leptos_meta::*;

mod components;
use components::bar::Bar;
use components::github::GithubActivity;
use components::placeholder::Placeholder;
use components::profile::Profile;
use components::window::Window;

#[derive(Clone, Copy, Debug, PartialEq)]
enum ContentId {
    Profile = 1,
    Github = 2,
    Contact = 3,
    Future = 4,
}

impl From<usize> for ContentId {
    fn from(id: usize) -> Self {
        match id {
            1 => ContentId::Profile,
            2 => ContentId::Github,
            3 => ContentId::Contact,
            4 => ContentId::Future,
            _ => ContentId::Profile,
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // Slots: 0=Main, 1=SubA, 2=SubB, 3=SubC
    // Values: Content IDs (1..4)
    // Initial State: Main=Profile(1), SubA=Github(2), SubB=Contact(3), SubC=Future(4)
    let (layout, set_layout) = create_signal(vec![1, 2, 3, 4]);
    let (active_main_id, set_active_main_id) = create_signal(1);

    let switch_content = move |target_id_usize: usize| {
        let update_dom = move || {
            let current_layout = layout.get();
            if let Some(current_slot_index) =
                current_layout.iter().position(|&id| id == target_id_usize)
            {
                if current_slot_index == 0 {
                    return;
                }

                let mut new_layout = current_layout.clone();
                let content_in_main = new_layout[0];
                new_layout[0] = target_id_usize;
                new_layout[current_slot_index] = content_in_main;
                set_layout.set(new_layout);
                set_active_main_id.set(target_id_usize);
            }
        };

        use wasm_bindgen::prelude::*;
        let doc = document();

        // Define key for startViewTransition property
        let svt_key = wasm_bindgen::JsValue::from_str("startViewTransition");
        // Check if API exists
        let has_vt = js_sys::Reflect::has(&doc, &svt_key).unwrap_or(false);

        if has_vt {
            // Create closure for the update callback
            let update_callback = Closure::wrap(Box::new(move || {
                update_dom();
            }) as Box<dyn FnMut()>);

            // Get startViewTransition function
            let start_vt_fn = js_sys::Reflect::get(&doc, &svt_key)
                .unwrap()
                .dyn_into::<js_sys::Function>()
                .unwrap();

            // Call it with the closure
            let _ = start_vt_fn.call1(&doc, &update_callback.as_ref().unchecked_ref());

            // Forget closure to prevent drop before execution (in a real app, manage memory better or allow leak for simple app-wide callbacks)
            update_callback.forget();
        } else {
            update_dom();
        }
    };

    // Helper to render content based on ID
    let render_content = |id: usize| match ContentId::from(id) {
        ContentId::Profile => view! { <Profile/> }.into_view(),
        ContentId::Github => view! { <GithubActivity/> }.into_view(),
        ContentId::Contact => view! {
            <div style="padding: 20px">
                <h3>"Contact Me"</h3>
                <p>"Github: " <a href="https://github.com/noharu36">"github.com/noharu36"</a></p>
                <p>"Twitter(X): " <a href="https://x.com/pieceofharuki">"@pieceofharuki"</a></p>
                <p>"Blog: " <a href="https://zenn.dev/haru_blog">"zenn.dev/haru_blog"</a></p>
            </div>
        }
        .into_view(),
        ContentId::Future => view! { <Placeholder/> }.into_view(),
    };

    // Helper to get title based on ID
    let get_title = |id: usize| {
        match ContentId::from(id) {
            ContentId::Profile => "~/about",
            ContentId::Github => "~/github-activity",
            ContentId::Contact => "~/contact",
            ContentId::Future => "~/future",
        }
        .to_string()
    };

    // Helper to get grid class based on slot index
    let get_slot_class = |index: usize| match index {
        0 => "main-window",
        1 => "sub-a",
        2 => "sub-b",
        3 => "sub-c",
        _ => "",
    };

    view! {
        <Html lang="en" />
        <Meta name="description" content="A tiling window manager style portfolio" />

        <div id="app">
            <Bar active_id=active_main_id on_switch=switch_content />

            {move || {
                layout.get().into_iter().enumerate().map(|(index, content_id)| {
                    // Assign unique view-transition-name to the Window component itself
                    // This creates a stable identity for the element across layout changes
                    let style = format!("view-transition-name: window-{}", content_id);
                    view! {
                        <Window
                            title=get_title(content_id)
                            class=get_slot_class(index)
                            style=style
                        >
                            <div class="window-wrapper" style="height: 100%; display: flex; flex-direction: column;">
                                {render_content(content_id)}
                            </div>
                        </Window>
                    }
                }).collect_view()
            }}
        </div>
    }
}
