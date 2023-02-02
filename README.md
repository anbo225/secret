# Secret 

## Intro

A simple tool to hide secret message in a png file. 

With the help of [this great article](https://picklenerd.github.io/pngme_book), I develop this tool to learn rust.


## How to install
```
cargo install --path .
```
## Usage Example : 



```
secret encode ./dice.png ruSt "This is a secret message!

secret decode ./dice.png ruSt

secret remove ./dice.png ruSt

secret print ./dice.png

```