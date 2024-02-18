---
link:
  - rel: 'stylesheet'
    href: 'css/default.css'
---

# ゲームを作りながら Rust 言語の基本を解説
- [前編](https://www.youtube.com/watch?v=LW9hT0nY51Y)
- [後編](https://www.youtube.com/watch?v=a01nZTFVj6Q)

## Rustについて

### Rustとは
- C++の代替を目的にMozilaが開発開始
- 所有権でメモリーリークを解決（静的言語仕様）
- 既存のプロジェクトの一部を置き換えることが可能
- クラスがなく、構造体（クラスから継承を除いたもの）にトレイト（≒インターフェイス）の実装を付与してオブジェクトを組み立てる
- 最初から開発ツール一式が揃っている
- 関数宣言はfn
- 型は後ろ置き
- 型に&で読み取り権、&mutで読み書き権を借りる
- 引数にmutをつけることで変更可
- 例外はなく、戻り値でエラーを表す
- システムレベル
  - お決まりのクラッシュやセキュリティホールのリスクの排除
  - スピードとメモリ使用
  - 信頼性の高いコードへと自然に導く
- Cargo: 付属の依存関係管理ツール兼ビルドツール
- Rustfmt: コーディングスタイルを保証
- Rust言語サーバー: コード補完やインラインエラーメッセージ
- [GitHub](https://github.com/rust-lang/book/tree/master/src)

### Rust以前にそもそもプログラミングをする環境構築
- ターミナルを使う
- 作業フォルダの確保
- VSCodeを使おう
  - Rust向けの拡張機能
    - rust-analyzer
    - rust
    - rust Syntax
- Gitを使おう
- MarkdownとVivliostyleを使おう

### インストール
- `brew install rust`もしくは`curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`
  - `rustup update`
  - `rustup self uninstall`
- `rustc --version`

### プロジェクトのセットアップ
- `cargo init tennis`
- `cd tennis`
- `code .`
- `cargo run`

## コーティング
標準入出力だけでゲームを作る

### 画面をクリア
- `println!("Hello, world!");` => `println!("\x1B[2J");`
  - xterm control sequence
  - [ANSIエスケープシーケンス チートシート #Bash - Qiita](https://qiita.com/PruneMazui/items/8a023347772620025ad6)

### コートの描画
- `const COAT_SIZE: i32 = 64;`
- `fn draw()`
  - mut
  - | + " " x COAT_SIZE + |
- `println!("\x1B[1;1H{}", buf);`
- main()内で実行
- `cargo run`

### メインループの作成
- `fn game_loop()`
  - 1/60秒ごとに実行
  - let mut time: SystemTime = SystemTime::now();
    - SystemTimeより、Instantの方が適切かも
  - `loop{}`
  - `time += Duration::from_nanos(16_666_667);`
  - `if let Ok(dur) = time.duration_since(SystemTime::now())`
    - Result型が返ってくる
    - `sleep(dur);`
  - `draw()`の移設
- `main()`内で実行
- `cargo run`（動きは変わらないけど、エラーが出なければOK）

### 描画処理
- draw()にボールの位置を引数にするように
  - `draw(0.0)`
- 引数の位置にボールを表示するように変更
  - ボール位置を浮動小数点数から64bit整数にキャスト
    - `fn draw(ball: f64)`
    - `let ball: i32 = (COAT_SIZE as f64 * ball).round() as i32;`
    - `buf += if i == ball { "@" } else { " " };`
- Rustは同じ名前の変数を定義可能
  - 新しいものが定義されると、古いものはアクセス不可に
  - 型を変更した際に、前のものは不使用にすることが明示できる
- Rustではifを式として使える
- `cargo run`（動きは変わらないけど、エラーが出なければOK）

### ゲームオブジェクトの作成
- Game構造体の作成 => `struct Game`
  - ballフィールドをもたせる => `pub ball: f64,`
    - pubを付けて公開
- 構造体のメソッドを実装 => `impl Game`
  - `pub fn new() -> Self`
  - `main()`内で関連関数として初期化 => `let mut game: Game = Game::new();`
  - その参照を`game_loop()`で使う => `game_loop();` -> `game_loop(&mut game);`
    - `fn game_loop()` => `fn game_loop(game: &mut Game)`
  - `draw()`の引数としてballフィールドを設定
    - `draw(0.0);` => `draw(game.ball);`
    - ballは実数型なので、moveでなく、copyになる
- `cargo run`（動きは変わらないけど、エラーが出なければOK）

### ボールを移動する
- メソッドと関連関数の違い
  - 引数にselfを持つかどうか
    - selfは構造体自身を指す
    - selfも所有権や参照がある
    - &mut selfで書き換え可能
- 呼び出し毎にボールの位置を0.1ずつ移動する
  - `game.update()`
  - `pub fn update(&mut self)`
  - `self.ball += 0.1;` => `self.ball += 0.01;`（はやすぎるので）
- `cargo run`で確認

### ゲームオーバー
- `update()`の中で判定
  - falseを受け取る
    - `pub fn update(&mut self) -> boo`l
    - `if self.ball > 1.0 { return false; } return true`
    - Rustでは関数の最後の式が戻り値になる
      - `self.ball <= 1.0` とする
  - `cargo run`で確認
- `game_loop()`の中で中断処理
  - `game.update()`からfalseが返ってきたら、`break `=> `!game.update()`
  - `println!("Game Over");`を最後に表示するようにする
  - `cargo run`で確認

前半終了

### 標準入力からエンターキーを待ち受け、入力をチェック
- サブスレッドで待ち受け、
  - `spawn( || );`
  - `fn sub_main() -> ! `
    - ! は関数が終了しないことを示す
    - 無限ループにして入力を受け続ける
    - 改行されたら（エンターキーが押されたら）処理
  - `let input: Stdin = stdin();`
    - 標準入力を取得
    - `loop{}`内に`input.read_line();`
      - 標準入力を待ち受け（改行）のタイミングでそれまでの入力を得る
  - `let mut buf: String = String::new();`
    - 受取用のbuf変数を用意
    - `input.read_line(&mut buf).unwrap();`
    - 戻り値がResult型
    - `unwrap()` => エラー処理を省略して値を取り出す
      - エラーが発生した場合は、panicで強制終了
  - `sub_main()`を`spawn()`から呼び出すようにする
    - `spawn( || sub_main());`
  - `cargo run`で確認
- エンターキーの入力でボールの向きを反転する
  - ボールのスピードを変数（構造体のフィールド）にしてしまう
    - `pub speed: f64,`
  - 初期化もする
    - `speed: 0.01,`
  - `update()`メソッドも修正
    - `self.ball += 0.01;` => `self.ball += self.speed;`
  - スイング判定フラグ
    - エンターキー => true `game_loop()`内でfalseにリセット
    - `main()`と`sub_main()`で読み書き
      - `let is_swing: Arc<Mutex<bool>> = Default::default();`
      - `fn sub_main(is_swing: &Mutex<bool>) -> !` 
      - `*is_swing.lock().unwrap() = true;`
      - Mutex => 複数箇所で読み書き
      - Arc => 複数箇所で所有権をシェア
      - Defaultトレイトを利用してインスタンスを初期化 => new()の処理を簡略化できる
        - `Default::default()`
      - `is_swing`を複数箇所で使う => Arcにしてclone()を使い、所有権を2つに分ける
        - クロージャ（変数に保存したり、引数として他の関数に渡すことのできる匿名関数）の中
          - [クロージャ：環境をキャプチャできる匿名関数](https://doc.rust-jp.rs/book-ja/ch13-01-closures.html)
            - コードの再利用や、動作のカスタマイズに利用
          - `sub_main()`
          - `game_loop()`
        - ブロックの中で`clone()`を使う
          - `let is_swing: Arc<Mutex<bool>> = is_swing.clone();`
          - `spawn( || sub_main());` => `spawn( move || sub_main(&is_swing));`
          - Mutexなので、mutにしない（読み取り専用）
      - `sub_main()`の`loop()`内
        - `read_Line()`の処理戻り => `is_swing`を`true`にする
          - `*is_swing.lock().unwrap() = true;`
          - `lock()` => 書き換えようのインスタンスを取得
          - `*` => 可変参照に対する加算代入のため、参照外し
          - [Rust初心者殺しの文法10選 #Rust - Qiita](https://qiita.com/muumu/items/8cdcc79fa881912adf51)
      - 当たり判定で、向きを反転（`update()`内）
        - `if *is_swing { self.speed *= -1.0; }`
        - `*is_swing = false;`
      - ボールの範囲判定
        - `0.0 < self.ball && self.ball <= 1.0`

### ラケットの位置でボールがラケットの位置に近いときだけ反転処理[完成]
- ラケットのサイズを定数にする
  - `const HARF_PADDLE_SIZE: f64 = 0.2 / 2.0;`
- ボールの位置がラケットの範囲内かどうかを確認
  - `let is_hit_left: bool = *is_swing && (-HARF_PADDLE_SIZE..HARF_PADDLE_SIZE).contains(&self.ball);`
  - `let is_hit_right: bool = *is_swing && (1.0 - HARF_PADDLE_SIZE..1.0 + HARF_PADDLE_SIZE).contains(&self.ball);`
  - 数値 .. 数値とすると、その範囲のインスタンを作成できる
  - `if *is_swing` => `if is_hit_left || is_hit_right`
- ラケットの範囲の分だけ、コートの幅を変更して、ゲームオーバー判定
  - `let out_left: bool = self.ball < -HARF_PADDLE_SIZE;`
  - `let out_right: bool = 1.0 + HARF_PADDLE_SIZE < self.ball`;
  - `0.0 < self.ball && self.ball <= 1.0` => `!out_left && !out_right`

### 打ち返したときにボールのスピードを上げる[味付け]
- `fn strike_back(&mut self) {`
  - `self.speed *= -1.0;`
- `self.speed *= -1.0;` => `self.strike_back();`
- `let is_left = self.ball < 0.5;` => ボールの位置で左右判定
- `let judgement = if is_left { self.ball.abs() } else {(self.ball - 1.0).abs()};`
  - ボールとラケットの距離を判定
- `let is_perfect_timing: bool = judgement < HARF_PADDLE_SIZE / 2.0;`
  - ラケットとボールの距離が近いかどうかを判定
- `self.speed *= -1.0 * if is_perfect_timing { 1.1 } else { 1.0 };`
  - スピードを速くしたり、しなかったり

## 完成コード
tennis/src/main.rs
```rust
use std::time::{ Duration, SystemTime };
use std::thread::{sleep, spawn};
use std::io::{stdin, Stdin};
use std::sync::{Arc, Mutex, MutexGuard};

// 定数
const COAT_SIZE: i32 = 64;
const HARF_PADDLE_SIZE: f64 = 0.2 / 2.0;

// ゲームの状態(構造体)
struct Game {
    pub ball: f64,
    pub speed: f64,
}

// ゲームの状態(構造体)の実装
impl Game {
    // ゲームの状態の初期化
    pub fn new() -> Self {
        Self {  
            ball: 0.0,
            speed: 0.01,
        }
    }

    // ゲームの状態の更新
    pub fn update(&mut self, is_swing: &Mutex<bool>) -> bool {
        // ボールの位置の更新
        self.ball += self.speed;
        // パドルとの当たり判定
        let mut is_swing: MutexGuard<bool> = is_swing.lock().unwrap();
        let is_hit_left: bool = *is_swing && (-HARF_PADDLE_SIZE..HARF_PADDLE_SIZE).contains(&self.ball);
        let is_hit_right: bool = *is_swing && (1.0 - HARF_PADDLE_SIZE..1.0 + HARF_PADDLE_SIZE).contains(&self.ball);
        // ボールの反射
        if is_hit_left || is_hit_right {
            self.strike_back();
        }
        // パドルの振り判定のリセット
        *is_swing = false;
        // ゲームの状態の判定
        let out_left: bool = self.ball < -HARF_PADDLE_SIZE;
        let out_right: bool = 1.0 + HARF_PADDLE_SIZE < self.ball;
        !out_left && !out_right
    }

    // ボールの反射(スプード調整)
    fn strike_back(&mut self) {
        // ボールの位置の判定
        let is_left = self.ball < 0.5;
        let judgement = if is_left { self.ball.abs() } else {(self.ball - 1.0).abs()};
        // ボールのスピードの調整
        let is_perfect_timing: bool = judgement < HARF_PADDLE_SIZE / 2.0;
        self.speed *= -1.0 * if is_perfect_timing { 1.1 } else { 1.0 };
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
fn game_loop(game: &mut Game, is_swing: &Mutex<bool>) {
    // 時間の初期化
    let mut time: SystemTime = SystemTime::now();
    // ループ
    loop{
        // ゲームの状態の更新・ゲームの状態によるループの終了
        if !game.update(is_swing) {
            break;
        }
        // 描画処理
        draw(game.ball);
        // 時間処理
        time += Duration::from_nanos(16_666_667);
        if let Ok(dur) = time.duration_since(SystemTime::now()) {
            sleep(dur);
        }
    }
    // ゲームオーバー
    println!("*** Game Over ***");
}

// サブメイン関数
fn sub_main(is_swing: &Mutex<bool>) -> ! {
    // 初期化
    let input: Stdin = stdin();
    let mut buf: String = String::new();
    // キーワード（標準入力）入力待ち
    loop {
        input.read_line(&mut buf).unwrap();
        *is_swing.lock().unwrap() = true;
    }
}

fn main() {
    // 画面のクリア
    println!("\x1B[2J");
    // パドルの振り判定の初期化
    let is_swing: Arc<Mutex<bool>> = Default::default();
    {
        // パドルの振り判定の初期化(コピー)
        let is_swing: Arc<Mutex<bool>> = is_swing.clone();
        // サブスレッドの生成（標準入力の監視）
        spawn( move || sub_main(&is_swing));
    }
    // ゲームループ処理
    let mut game: Game = Game::new();
    game_loop(&mut game, &is_swing);
}
```

## 独自のカスタマイズ

### アイデア
- 点数制にする
  - スコア表示
- 対戦環境
  - エンターでなく、キーで各ユーザーの操作を判別
  - ネットワーク対戦？
    - 対戦サーバー
    - 待機ロビー
    - ユーザー判別
    - スコアランキング
- サウンド
- ゲームの時間カウント？
- テニスみたいにする？