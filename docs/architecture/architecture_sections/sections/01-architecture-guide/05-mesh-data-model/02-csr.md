
セル→面、セル→ノード、面→ノードの接続情報はCSR形式（offsets配列 + indices配列）で格納する。`Vec<Vec<FaceId>>`のようなヒープ散在を避け、キャッシュ効率とSIMD対応を確保する。
