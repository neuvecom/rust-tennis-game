const COAT_SIZE: i32 = 64;

// 描画処理
fn draw() {
    // 描画バッファの作成
    let mut buf = String::from(" ");
    buf += "|";
    for _ in 0..COAT_SIZE {
        buf += " ";
    }
    buf += "|";
    // 描画
    println!("\x1B[1;1H{}", buf);
}

fn main() {
    // 画面のクリア
    println!("\x1B[2J");
    // 描画処理
    draw();
}
