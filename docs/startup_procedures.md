cargo new <directory_name>
cd <directory_name>

git init

docsディレクトリ作成

.vscode/setting.json作成

.editorconfig作成

.rustfmt.toml作成

rust-toolchain.toml作成

rustup toolchain add stable

rustup toolchain add nightly

postgres環境構築
  (wsl)
  sudo apt update
  sudo apt install -y postgresql postgresql-contrib

  sudo service postgresql start
  sudo service postgresql status

  sudo -u postgres psql

  # ユーザー作成
  CREATE USER <user> WITH PASSWORD '<password>';

  # データベース作成
  CREATE DATABASE <name> OWNER <user>;

  # 権限付与
  GRANT ALL PRIVILEGES ON DATABASE <name> TO <user>;

  \q

  # ファイルを編集
  sudo nano /etc/postgresql/*/main/pg_hba.conf
    (旧)local all all peer
    (新)local all all md5

  # postgres再起動
  sudo service postgresql restart

  # VSCodeターミナルで以下実行
  psql -h <host> -p <port> -U <user> -d <name>


cargo add sqlx --features runtime-tokio,tls-native-tls,postgres
cargo install sqlx-cli --no-default-features --features native-tls,postgres

sqlx migrate add create_<tb名>_table
⇒中身を記載
