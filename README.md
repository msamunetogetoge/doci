# doci
マークダウン形式で書かれた文書からwikiを生成するアプリ。ユーザー、グループ登録、招待、マークダウンの読み込み、plantumlを使って図を挿入、あたりが出来るようになる予定。

## 最初にやること
.envにpostgresの環境を書く
```
cd back
cargo install sqlx-cli --no-default-features --features native-tls,postgres
sqlx database create
sqlx migrate run

cd ..
cd front
npm install

```
