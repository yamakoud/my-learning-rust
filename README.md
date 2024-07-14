## 実行コマンド

`cargo run`

## ディレクトリ

### 問題の場所

instructions 配下に v1.md, v2.md, v3.md ...があります。

### rust のファイル
src/mains の配下に v1.rs, v2.rs, v3.rs ...があります。


## 演習の進め方

まずは main.rs の呼び出し方を変えてください。
v1 の演習をしたい場合は下記のように変更してください。
```rust
mod mains {
    pub mod v1;
}

fn main() {
    mains::v1::main();
}

```

そうすることで src/mains/v1.rs が呼び出されます。
あとは自分で v1.rs の挙動を確認したり、自分でゼロから書いてみたりしましょう。

## 問題の追加

ここの問題の追加は yamakoud が演習を進めるたびに追加されます。
