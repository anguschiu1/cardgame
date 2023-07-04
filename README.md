# Spot it! (a.k.a. Dobble) card game CLI

This is a simple SpotIt card game written in Rust.

There are four goals to be achieved incrementally:

1. A CLI game that allows one to set the table and play Spot It! by himself (or herself)
2. It will become a Rust cargo library to be used in other projects, a.k.a. the immediate follow up would be a WASM game of Spot It! in the browser.
3. The game will be fine-tuned and become deployable into a Pi Zero 2 W, and playable in the browser using WASM
4. The game wiill become a multiple-player game, with a server and a client, and a web interface, running on Pi Zero 2 W

## How to play

SpotIt is a game where you have to find the same symbol on two cards. Each card has exactly one symbol in common with any other card. The first player to find the symbol wins the round and gets a point. The player with the most points at the end of the game wins.

## Maths behind the game

### Reference

[Here](https://www.petercollingridge.co.uk/blog/mathematics-toys-and-games/dobble/) and [there](https://www.smithsonianmag.com/science-nature/math-card-game-spot-it-180970873/)

## How to run

`cargo run`

## How to build

`cargo build`

## How to test

`cargo test`
