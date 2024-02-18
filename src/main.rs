use std::time::SystemTime;
use std::thread::sleep;
use std::time::Duration;

const COAT_SIZE: i32 = 64;

// ゲームの状態(構造体)
struct Game {
    pub ball: f64,
}

// ゲームの状態(構造体)の実装
impl Game {
    // ゲームの状態の初期化
    pub fn new() -> Self {
        Self {  
            ball: 0.0,
        }
    }
}

// 描画処理
fn draw(ball: f64) {
    // ボールの位置の計算
    let ball: i32 = (COAT_SIZE as f64 * ball).round() as i32;
    // 描画バッファの作成
    let mut buf = String::from(" ");
    buf += "|";
    for i in 0..COAT_SIZE {
        buf += if i == ball { "@" } else { " " };
    }
    buf += "|";
    // 描画
    println!("\x1B[1;1H{}", buf);
}

// ループ処理
fn game_loop(game: &mut Game) {
    // 時間の初期化
    let mut time: SystemTime = SystemTime::now();
    // ループ
    loop{
        // 描画処理
        draw(game.ball);
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
    let mut game: Game = Game::new();
    game_loop(&mut game);
}
