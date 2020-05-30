dockerize -wait tcp://$DB_HOST:$DB_PORT
diesel migration run
echo "RUST_ENV: $RUST_ENV"
if [[ $RUST_ENV == *"dev"* ]]; then
  systemfd --no-pid -s "0.0.0.0:$SERVER_PORT" -- cargo watch -x run
else
  cargo run --release
fi