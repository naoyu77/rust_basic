# Rust 基本学習メモ

## プロジェクト概要
コマンドライン引数を受け付ける基本的なRustプログラム

## 実行方法

### cargo経由で実行
```bash
cargo run -- 引数1 引数2 引数3
```
- `--` の前はcargoのオプション
- `--` の後はプログラムへの引数

### ビルドして直接実行
```bash
# デバッグビルド
cargo build
./target/debug/basic 引数1 引数2

# リリースビルド（最適化あり）
cargo build --release
./target/release/basic 引数1 引数2
```

## ビルド成果物の場所
- デバッグビルド: `target/debug/basic`
- リリースビルド: `target/release/basic`

## 学んだRustの概念

### 1. 所有権（Ownership）
```rust
let s1 = String::from("hello");
let s2 = s1;  // s1の所有権がs2に移動
// println!("{}", s1);  // エラー！s1は無効
```

### 2. 参照と借用（References & Borrowing）
```rust
// 参照を使うと所有権は移動しない
let s1 = String::from("hello");
let s2 = &s1;  // s1を借用
println!("{}", s1);  // OK！s1はまだ有効
```

#### 参照の種類
- `&T` - イミュータブルな参照（読み取り専用、複数持てる）
- `&mut T` - ミュータブルな参照（変更可能、1つだけ）

### 3. イテレータの種類
```rust
let vec = vec![String::from("a"), String::from("b")];

// .iter() - 参照を返す（&T）
for item in vec.iter() {
    // item: &String
}

// .into_iter() - 所有権を移動（T）
for item in vec.into_iter() {
    // item: String
    // vecは使えなくなる
}

// .iter_mut() - ミュータブルな参照（&mut T）
for item in vec.iter_mut() {
    // item: &mut String
    // 変更できる
}
```

### 4. コマンドライン引数の取得
```rust
use std::env;

let args: Vec<String> = env::args().collect();
// args[0] = プログラム名
// args[1] = 1番目の引数
// args[2] = 2番目の引数...
```

## Rustの特徴

### パフォーマンス
- C/C++と同等の実行速度
- ガベージコレクションなし
- ゼロコスト抽象化
- LLVMによる強力な最適化

### 速度比較（目安）
```
C/C++ ≈ Rust > Go > Java (JIT後) >> Python
```

### ビルドの種類
- **デバッグビルド**: 最適化なし、デバッグ情報あり
- **リリースビルド**: 最適化あり、数倍〜数十倍高速

## メモ
- デフォルトで変数はイミュータブル（不変）
- ミュータブルにするには `let mut` を使う
- 関数の最後の式（セミコロンなし）が戻り値になる
- メモリ安全性をコンパイル時に保証
