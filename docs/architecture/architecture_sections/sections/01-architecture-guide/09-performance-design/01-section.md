
- 全フィールドは`Vec<f64>` / `Vec<[f64; 3]>`の連続配列
- メッシュ接続はCSR圧縮（`Vec<u32>` offsets + `Vec<Id>` indices）
- `HashMap`はホットループ外（場の取得は1回/タイムステップ/フィールド）
