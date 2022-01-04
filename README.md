# daily-puzzle-wasm

A convoluted mess of a program to solve a [daily puzzle from Dragon Fjord](https://www.dragonfjord.com/product/a-puzzle-a-day/).

Written with Rust and [Seed-rs](https://seed-rs.org/) and compiled to Web Assembly.

Keep in mind that this ***runs very slowly***. On a computer, this takes probably less than 2 minutes per date, but with wasm I've had a few test dates run for MUCH LONGER than that.