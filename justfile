default:
  @just --list

build:
  cargo lbuild

run:
  RUST_LOG=debug cargo lrun

test:
  cargo nextest run

