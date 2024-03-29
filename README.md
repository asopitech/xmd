# Executable Markdown

これは Markdown を実行可能にする革新的なツールです

Chat GPT に以下のプロンプトを与えて生成させたコードです。

```
あなたは優秀なプログラマーです。
以下の仕様をもとにRust言語のテストコードを作成してください。
また、仕様を満たしあなたが出力したテストコードを通過するプログラムをRust言語で作成してください。

## 仕様
### 前提条件
- タスクランナーのコマンドラインツール

### 動作
- 本コマンドを xmd とする。
- 同一ディレクトリ内の README.md という Markdown 形式のファイルを読み込む。
- "## タスクリスト" で始まる行以下を対象とする。
- "```" で囲まれた箇所をコードブロックとする。
- "###" で始まり次の "###"の間でコードブロックを含む範囲をタスク定義ブロックとする。 
- タスク定義ブロックの "### "以降の文字列をタスク名とする。 
- タスク定義ブロックの "### "の次の行からコードブロックまでの間のコードブロックを含まない行をヘルプ文とする
- タスク名が渡されたらタスク定義ブロック内のコードブロックをシェルスクリプトとして実行する
- タスク名が渡されなければ、すべての"タスク名 タブ 空白文字を除去したヘルプ文"を出力する
- 出力されるヘルプ文は、２行目以降はインデントをそろえる
- README.md というファイルがなければ "README.md がありません"と出力する
- `-f` オプションで README.md 以外のMarkdownファイルも指定できる

なお出力形式は以下とします。
テストコードは同値分割法(Equivalence Partitioning Testing)と境界値分析(Boundary value analysis)を用いてください。

## 出力形式
- 概要
- 短い実装コード

私たちがこのタスクで最高の結果を出すためにステップバイステップで考えてください。
```

## タスクリスト

### test

Rust test を実行します。
いつものあれだし。

```
cargo test
```

### echo

Echo "hello world!"

```
echo "hello world!"
```