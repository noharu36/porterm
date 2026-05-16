use gloo_net::http::Request;
use leptos::*;
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};

/// 記事メタデータ（index.json の各エントリ）
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArticleMeta {
    pub slug: String,
    pub title: String,
    pub tags: Vec<String>,
    pub publish: bool,
}

/// 記事一覧を index.json から取得
async fn fetch_article_list() -> Result<Vec<ArticleMeta>, String> {
    let resp = Request::get("/assets/articles/index.json")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.ok() {
        return Err(format!("Failed to fetch article list: {}", resp.status()));
    }

    let articles: Vec<ArticleMeta> = resp.json().await.map_err(|e| e.to_string())?;
    Ok(articles.into_iter().filter(|a| a.publish).collect())
}

/// 個別記事のMarkdownを取得
async fn fetch_article_content(slug: String) -> Result<String, String> {
    let url = format!("/assets/articles/{}.md", slug);
    let resp = Request::get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.ok() {
        return Err(format!("Failed to fetch article: {}", resp.status()));
    }

    resp.text().await.map_err(|e| e.to_string())
}

/// Markdown → HTML 変換
fn markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

/// URLハッシュからスラッグを取得
fn get_slug_from_hash() -> Option<String> {
    let hash = window().location().hash().unwrap_or_default();
    if hash.starts_with("#/blog/") {
        let slug = hash.trim_start_matches("#/blog/");
        if !slug.is_empty() {
            return Some(slug.to_string());
        }
    }
    None
}

/// URLハッシュを記事スラッグに設定
fn set_article_hash(slug: &str) {
    let _ = window().location().set_hash(&format!("/blog/{}", slug));
}

/// URLハッシュをクリア（pushState でハッシュなしURLに遷移）
fn clear_article_hash() {
    let pathname = window().location().pathname().unwrap_or_default();
    let _ = js_sys::eval(&format!("history.pushState(null, '', '{}')", pathname));
}

/// 記事のフルURLを生成
fn get_article_url(slug: &str) -> String {
    let loc = window().location();
    let origin = loc.origin().unwrap_or_default();
    let pathname = loc.pathname().unwrap_or_default();
    format!("{}{}#/blog/{}", origin, pathname, slug)
}

/// ブログ一覧コンポーネント
#[component]
pub fn Blog() -> impl IntoView {
    let articles_resource = create_resource(|| (), |_| fetch_article_list());

    // モーダルの状態管理
    let (selected_article, set_selected_article) = create_signal::<Option<ArticleMeta>>(None);
    let (is_maximized, set_is_maximized) = create_signal(false);

    // URL ハッシュから開くべき記事のスラッグ
    let (pending_slug, set_pending_slug) = create_signal::<Option<String>>(get_slug_from_hash());

    // 記事一覧ロード後に pending_slug の記事を自動で開く
    create_effect(move |_| {
        if let Some(slug) = pending_slug.get() {
            if let Some(Ok(articles)) = articles_resource.get() {
                if let Some(article) = articles.iter().find(|a| a.slug == slug) {
                    set_selected_article.set(Some(article.clone()));
                }
                set_pending_slug.set(None);
            }
        }
    });

    // ブラウザの戻る/進むボタンでモーダルの開閉を同期
    {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;

        let on_nav = Closure::wrap(Box::new(move || {
            match get_slug_from_hash() {
                Some(slug) => set_pending_slug.set(Some(slug)),
                None => {
                    set_selected_article.set(None);
                    set_is_maximized.set(false);
                }
            }
        }) as Box<dyn FnMut()>);

        // hashchange: set_hash() による変更を検知
        let _ = window().add_event_listener_with_callback(
            "popstate",
            on_nav.as_ref().unchecked_ref(),
        );
        on_nav.forget();
    }

    let close_modal = move |_| {
        set_selected_article.set(None);
        set_is_maximized.set(false);
        clear_article_hash();
    };

    let toggle_maximize = move |_| {
        set_is_maximized.update(|v| *v = !*v);
    };

    view! {
        <div class="blog-container">
            <Suspense fallback=move || view! { <p class="blog-loading">"Loading articles..."</p> }>
                {move || {
                    articles_resource.get().map(|result| match result {
                        Ok(articles) => {
                            if articles.is_empty() {
                                view! {
                                    <div class="blog-empty">
                                        <p>"No articles yet."</p>
                                    </div>
                                }.into_view()
                            } else {
                                let set_selected = set_selected_article.clone();
                                view! {
                                    <div class="blog-list">
                                        {articles.into_iter().map(|article| {
                                            let article_for_click = article.clone();
                                            let set_selected = set_selected.clone();
                                            view! {
                                                <button
                                                    class="blog-card"
                                                    on:click=move |_| {
                                                        set_article_hash(&article_for_click.slug);
                                                        set_selected.set(Some(article_for_click.clone()));
                                                    }
                                                >
                                                    <div class="blog-card-title">{&article.title}</div>
                                                    <div class="blog-card-tags">
                                                        {article.tags.iter().map(|tag| {
                                                            view! {
                                                                <span class="blog-tag">{tag}</span>
                                                            }
                                                        }).collect_view()}
                                                    </div>
                                                </button>
                                            }
                                        }).collect_view()}
                                    </div>
                                }.into_view()
                            }
                        }
                        Err(e) => view! {
                            <p class="blog-error">{format!("Error: {}", e)}</p>
                        }.into_view(),
                    })
                }}
            </Suspense>

            // モーダル（Portal で body 直下にレンダリング）
            {move || {
                selected_article.get().map(|article| {
                    let article_clone = article.clone();
                    view! {
                        <Portal>
                            <ArticleModal
                                article=article_clone.clone()
                                is_maximized=is_maximized
                                on_close=close_modal
                                on_toggle_maximize=toggle_maximize
                            />
                        </Portal>
                    }
                })
            }}
        </div>
    }
}

/// 記事モーダルコンポーネント
#[component]
fn ArticleModal(
    article: ArticleMeta,
    is_maximized: ReadSignal<bool>,
    on_close: impl Fn(ev::MouseEvent) + 'static + Copy,
    on_toggle_maximize: impl Fn(ev::MouseEvent) + 'static + Copy,
) -> impl IntoView {
    let slug = article.slug.clone();
    let slug_for_x = article.slug.clone();
    let slug_for_copy = article.slug.clone();
    let title_for_x = article.title.clone();

    let content_resource = create_resource(
        move || slug.clone(),
        |slug| fetch_article_content(slug),
    );

    // コピー完了フィードバック
    let (copied, set_copied) = create_signal(false);

    // X (Twitter) シェア
    let share_on_x = move |_: ev::MouseEvent| {
        let url = get_article_url(&slug_for_x);
        let encoded_url = js_sys::encode_uri_component(&url).to_string();
        let encoded_text = js_sys::encode_uri_component(&title_for_x).to_string();
        let share_url = format!(
            "https://twitter.com/intent/tweet?url={}&text={}",
            encoded_url, encoded_text
        );
        let _ = js_sys::eval(&format!("window.open('{}', '_blank')", share_url));
    };

    // URLコピー
    let copy_url = move |_: ev::MouseEvent| {
        let url = get_article_url(&slug_for_copy);
        let _ = js_sys::eval(&format!(
            "navigator.clipboard.writeText(\"{}\")",
            url.replace('"', "\\\"")
        ));
        set_copied.set(true);
        set_timeout(
            move || set_copied.set(false),
            std::time::Duration::from_millis(2000),
        );
    };

    // ESCキーでモーダルを閉じる
    let on_close_for_key = on_close.clone();
    let on_keydown = move |e: ev::KeyboardEvent| {
        if e.key() == "Escape" {
            use wasm_bindgen::JsCast;
            if let Ok(event) = web_sys::MouseEvent::new("click") {
                on_close_for_key(event.unchecked_into());
            }
        }
    };

    view! {
        <div
            class="modal-overlay"
            on:click=on_close
            on:keydown=on_keydown
            tabindex="-1"
        >
            <div
                class="modal-window"
                class:modal-maximized=move || is_maximized.get()
                on:click=move |e| e.stop_propagation()
            >
                <div class="modal-header">
                    <div class="modal-title-area">
                        <span class="modal-title">{&article.title}</span>
                        <div class="modal-tags">
                            {article.tags.iter().map(|tag| {
                                view! {
                                    <span class="blog-tag">{tag}</span>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                    <div class="modal-controls">
                        <button class="modal-btn modal-btn-maximize" on:click=on_toggle_maximize>
                            {move || if is_maximized.get() { "[ ]" } else { "[□]" }}
                        </button>
                        <button class="modal-btn modal-btn-close" on:click=on_close>
                            "[x]"
                        </button>
                    </div>
                </div>
                <div class="modal-body">
                    <Suspense fallback=move || view! { <p>"Loading article..."</p> }>
                        {move || {
                            content_resource.get().map(|result| match result {
                                Ok(markdown) => {
                                    let html_content = markdown_to_html(&markdown);
                                    view! {
                                        <div class="article-content" inner_html=html_content />
                                    }.into_view()
                                }
                                Err(e) => view! {
                                    <p class="blog-error">{format!("Error: {}", e)}</p>
                                }.into_view(),
                            })
                        }}
                    </Suspense>

                    // シェアセクション（記事下部）
                    <div class="share-section">
                        <span class="share-label">"Share"</span>
                        <div class="share-buttons">
                            <button class="share-btn" title="Share on X" on:click=share_on_x>
                                "𝕏 Post"
                            </button>
                            <button
                                class="share-btn"
                                class:copied=move || copied.get()
                                title="Copy link"
                                on:click=copy_url
                            >
                                {move || if copied.get() { "✓ Copied!" } else { "Copy URL" }}
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
