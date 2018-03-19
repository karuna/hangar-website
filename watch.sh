#!/bin/sh
watchexec --exts rs,toml,sql,tera --restart "CARGO_INCREMENTAL=0 RUST_BACKTRACE=1 cargo run"
