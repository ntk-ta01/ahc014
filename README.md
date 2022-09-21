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
- ~~optuna使ってパラメータをいい感じにする~~

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
- あとは打つ順番をいじっていくゲームか？
- 長方形の面積を二乗じゃなくてちゃんと計算するようにしてパラメータ調整。41M

# 方針3 9/20
局所探索法を適用したい。頻度メモリを使いたい。ILSでよくないか？greedyにしている挿入をとにかくする。kickはrandomにd個削除。
とりあえずILSじゃなくていいか

とりあえずrandomにd個削除+greedy同様の挿入で焼きなまし法。プレテスト42.8M。
大きい面積の長方形から消したいな…
面積に応じて再挿入禁止でもいい

construct_insertableにタブーリスト渡して「これはどうせ使わないやつ」とか調べないようにしたい

面積5以上を削除、2点swap、1点relocateどれもスコアが上がらないな…
select_insertableのパラメータ調整、面積が大きいやつのスコアを下げるとよさそう。提出43M。

seed=0で83万点が出た。c7c7さんの90万点と比べると、より外側に点を打てるようにする必要がありそう
けっこう外周に打つやつもあるから端っこまで調べない、とかいう雑なことはできないな

# 方針4 9/21
パラメータ調整optuna編
コマンドライン引数としてRustに読み込ませるか 提出45M。

ランダムにd個削除が1個のがよさそうだった。そしてやっぱり実行不可能解も探索していきたいな。現時点でサイクリングがあるかも気になる
