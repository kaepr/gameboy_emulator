default:
    @just --list

help:
    cargo run -- --help

zellij:
    zellij -l layout.kdl

dev:
    cd frontend/web && npm run dev

