// Cloudflare Workers エントリポイント
// Workers Static Assets が有効な場合、静的ファイルは自動的に配信されます。
// このファイルは、動的ルートのフォールバックや追加ロジックが必要な場合に拡張できます。

export default {
  async fetch(request, env, ctx) {
    // 静的アセットは wrangler.toml の [assets] 設定により自動配信
    // ここではフォールバックとして index.html を返す (SPA用)
    const url = new URL(request.url);
    
    // アセットが見つからない場合は index.html にフォールバック (SPA対応)
    // Workers Static Assets は自動的にアセットを優先するため、
    // このロジックは通常トリガーされません
    return env.ASSETS.fetch(request);
  },
};
