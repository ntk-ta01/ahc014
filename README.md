# AHC014
Visualizerを触る。seed=0で45万点くらいまで手動で出る。Twitterで80万くらいまで出してる人がいた。

とりあえず書きたいコードとして、
- ~~入力を受け取る~~
- ~~出力をする~~
- ~~印付き点を管理する~~
- ~~印を打つ候補を列挙する~~
- ~~得点計算関数を作成する~~
- Nの大きさごとに得点の分布を見る(AHC013のように)
- なんらかのメタヒューリスティクスを用いて解を改善する

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

# 方針1
打点可能点を列挙して重みで降順ソート。roulette-wheel-selectionして時間いっぱい貪欲してbestを取る。プレテスト35M。