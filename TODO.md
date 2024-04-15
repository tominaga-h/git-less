# TODOリスト

## ざっくりとした流れ

- [ ] コマンドライン引数から`ls-tree`する`object`を取得
- [ ] `git ls-tree`を実行して結果を取得する
- [ ] `ls-tree`の結果をパースして`ref`を取り出す
- [ ] `skim`を使用して`ref`をユーザーに選択させる
- [ ] ユーザーが選択した`ref`を使用して`git cat-file -p`する
- [ ] した結果をlessコマンドで読み込む

## インストールするもの

- [ ] `skim` fuzz finderのrustバージョン
- [ ] `subprocess` 子プロセスを動かす
- [ ] `clap` コマンドライン引数のパーサー
