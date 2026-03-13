# PalmScript を学ぶ

PalmScript の公開ドキュメントは次の二つを中心に構成されています。

- 戦略を書くための言語
- スクリプトの書き方と使い方を示す例

## PalmScript で行うこと

典型的な流れ:

1. `.ps` スクリプトを書く
2. ベースの `interval` を宣言する
3. 1 つ以上の `source` バインディングを宣言する
4. ブラウザ IDE で検証する
5. アプリ内で履歴データに対して実行する

## 長時間の最適化

長い CLI チューニングジョブでは:

- すぐに前面で結果が欲しいなら `palmscript run optimize ...` を使う
- CLI から直接最適化したいなら `palmscript run optimize ...` を使う
- 有望な candidate は `--preset-out best.json` で保存し、`run backtest` や `run walk-forward` で再評価する
- 明示的に無効化したい場合を除き、既定の untouched holdout を有効のままにする

## 次に読むもの

- 最初の実行フロー: [クイックスタート](quickstart.md)
- 最初の完全な戦略 walkthrough: [最初の戦略](first-strategy.md)
- 言語全体の見取り図: [言語概要](language-overview.md)
- 正確なルールとセマンティクス: [リファレンス概要](../reference/overview.md)

## ドキュメントの役割

- `学ぶ` は PalmScript を効果的に使う方法を説明します。
- `リファレンス` は PalmScript の意味を定義します。
