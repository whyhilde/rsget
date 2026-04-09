name := "rsget"
install_path := "/usr/bin"

default:
  @just --list

build:
  @cargo build --release

run:
  @cargo run --release

install:
  @cargo build --release
  @sudo cp ./target/release/{{name}} {{install_path}}
