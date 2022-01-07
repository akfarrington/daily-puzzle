# daily-puzzle-wasm

A convoluted mess of a program to solve a [daily puzzle from Dragon Fjord](https://www.dragonfjord.com/product/a-puzzle-a-day/).

Written with Rust and [Seed-rs](https://seed-rs.org/) and compiled to Web Assembly.

Keep in mind that this ***can run very slowly***. With a native binary, this takes probably less than 1 minute per date. With wasm I've had a few test dates run for MUCH LONGER than that, but less so after fixing wasm-opt.