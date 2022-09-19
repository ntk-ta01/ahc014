# AHC014
Visualizerを触る。seed=0で45万点くらいまで手動で出る。Twitterで80万くらいまで出してる人がいた。

とりあえず書きたいコードとして、
- ~~入力を受け取る~~
- ~~出力をする~~
- ~~印付き点を管理する~~
- ~~印を打つ候補を列挙する~~
- ~~得点計算関数を作成する~~
- ~~Nの大きさごとに得点の分布を見る(AHC013のように)~~
- なんらかのメタヒューリスティクスを用いて解を改善する
- ~~印の打点候補列挙の計算量を落とす。差分で持つ。~~
- 手元のテストの計算時間を計測する
- ~~長方形の面積^2計算関数を書く~~

# tester
```
cargo run -p tools --release --bin tester 0
```
/tools/in/0000.txtを用いてテスト
```
cargo run -p tools --release --bin tester 0 1 2
```
/tools/in/0000.txt ~ /tools/in/0002.txtを用いてテスト
```
cargo run -p tools --release --bin tester
```
/tools/in/にあるケース全部でテスト

1ケース4.5secにしても1000ケース1m18secくらい

```
cargo run -p tools --release --bin tester 0 out
```
`out`をつけると`/tools/out/*.txt`に解を記録。

```
cargo run -p tools --release --bin tester avg
```
`avg`をつけると、`n=(3*)`などNの大きさごとに分けて得点を追加で表示する。

# 方針1 9/18
打点可能点を列挙して重みで降順ソート。roulette-wheel-selectionして時間いっぱい貪欲してbestを取る。プレテスト35M。

# 方針2 9/19
印の打点候補列挙の更新を差分計算にした。4.5sec貪欲してbestを取る。プレテスト38M。

# アイデア
- c7c7さんのseed=0, 0.9M点を見ると細かい長方形が多い。これにならって面積の二乗を打点候補列挙の評価値に入れてみたが、方針2からそれほどよくならない。狙いを持って打点したり、打点列の操作をする必要がありそう。プレテスト38M。