default:
    @just --list

help:
    cargo run -- --help

zellij:
    zellij -l layout.kdl

dev:
    cd frontend && npm run dev

