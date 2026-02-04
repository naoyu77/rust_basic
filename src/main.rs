use std::env;

fn main() {
    // コマンドライン引数を取得
    let args: Vec<String> = env::args().collect();

    println!("プログラム名: {}", args[0]);
    println!("引数の数: {}", args.len() - 1);

    // 引数がない場合
    if args.len() == 1 {
        println!("使い方: {} <引数1> <引数2> ...", args[0]);
        return;
    }

    // すべての引数を表示
    println!("\n受け取った引数:");
    for (i, arg) in args.iter().enumerate().skip(1) {
        println!("  引数{}: {}", i, arg);
    }

    // 引数を使った簡単な処理例
    if args.len() > 1 {
        println!("\n最初の引数: {}", args[1]);
    }

    if args.len() > 2 {
        println!("2番目の引数: {}", args[2]);
    }
}
