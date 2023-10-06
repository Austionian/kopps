# Kopps

For Wisconsin custurd lovers, a simple web scraper written in Rust to get the flavors of the day.

## How to use

Run:
``` shell
$ kopps

// Output:
__TODAY__
🍦  Chocolate Peanut Butter Chocolate
🍦  Strawberry Rhubarb Pie

__TOMORROW__
🍦  Mint Chip
🍦  Twisted Turtle
```

## Installation (on Mac)

Build the release binary:
`cargo build --release`

Symlink the binary to your PATH:
`sudo ln -s /User/<..>/kopps/target/release/kopps`


