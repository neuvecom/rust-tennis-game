use std::time::SystemTime;
use std::thread::sleep;
use std::time::Duration;

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

// ループ処理
fn game_loop() {
    // 時間の初期化
    let mut time: SystemTime = SystemTime::now();
    // ループ
    loop{
        // 描画処理
        draw();
        // 時間処理
        time += Duration::from_nanos(16_666_667);
        if let Ok(dur) = time.duration_since(SystemTime::now()) {
            sleep(dur);
        }
    }
}

fn main() {
    // 画面のクリア
    println!("\x1B[2J");
    // ゲームループ処理
    game_loop();
}
