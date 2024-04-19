# TODOリスト

## インストールするもの

- [x] `skim` fuzz finderのrustバージョン
- [x] `subprocess` 子プロセスを動かす
- [x] `clap` コマンドライン引数のパーサー

## ざっくりとした流れ

- コマンドライン引数から`ls-tree`する`object`を取得
- `git ls-tree`を実行して結果を取得する
- `ls-tree`の結果をパースして`ref`を取り出す
- `skim`を使用して`ref`をユーザーに選択させる
- ユーザーが選択した`ref`を使用して`git cat-file -p`する
- した結果をlessコマンドで読み込む

## TODO

- [x] コマンドライン引数の定義
- [x] コマンドライン引数のテスト
- [x] `ls-tree`を実行してみる
- [x] `ls-tree`の実行に`-r`オプションをつける
- [x] `ref`を`rev`になおす(`rev`は`revision`を意味し、`ref`は自分の勘違い)
- [x] `ls-tree`の実行結果をパースしてハッシュ値とファイル名を取得する
- [x] `parse`関数のパーシング部分を`parse_line`関数に抽出
- [x] ハッシュ値から`cat-file -p`を実行して中身を取得
- [x] `main`関数のネストが深くなるので`app`関数に抽出
- [x] `TreeParser`のテストを書く
- [x] `ls-tree`と`cat-file`のエラー処理をちゃんと書く
- [x] `skim`で選択させる

## main.rsの流れ

- [ ] コマンドライン引数のパース
- [ ] コマンドライン引数(tree, revのみ)によって`RepositoryObject`を作成
- [ ] `RepositoryObject`を使用して`ls-tree`を実行後`TreeItem`を取得
