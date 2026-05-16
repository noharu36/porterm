# ポートフォリオサイトをRust (WASM) で作った話

## はじめに

こんにちは！この記事では、ポートフォリオサイトを **Rust** と **WebAssembly** で構築した話を紹介します。

ターミナルのタイリングウィンドウマネージャーをモチーフにしたデザインで、[Leptos](https://leptos.dev/) フレームワークを使って実装しました。

## 技術スタック

使用した技術は以下の通りです：

- **Leptos** — Rust製のリアクティブWebフレームワーク
- **Trunk** — WASMアプリケーションのビルドツール
- **Cloudflare Workers** — デプロイ先
- **pulldown-cmark** — Markdownパーサー

## なぜRust + WASMなのか

正直に言うと、「やってみたかったから」というのが一番の理由です。

> 好きな技術で何かを作ることが、一番のモチベーションになる。

普段からRustが好きで書いていたので、ポートフォリオもRustで作ってみようと思い立ちました。

### メリット

1. **型安全性** — コンパイル時に多くのバグを防げる
2. **パフォーマンス** — WASMは高速に動作する
3. **楽しい** — Rustを書くこと自体が楽しい

### 大変だったこと

- WASMのデバッグは通常のWebアプリより難しい
- Leptosのエコシステムはまだ成熟途上
- CSSは結局手書き

## コード例

Leptosのコンポーネントはこんな感じで書けます：

```rust
#[component]
fn HelloWorld() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button on:click=move |_| set_count.update(|n| *n += 1)>
            "Click me: " {count}
        </button>
    }
}
```

## まとめ

Rust + WASMでのWeb開発は、まだまだ挑戦的な部分がありますが、非常に面白い体験でした。
興味がある方はぜひ試してみてください！
