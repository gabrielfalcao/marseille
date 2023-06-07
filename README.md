# Marseille

![image](./die-erstes-regeln-ist.jpeg)

```
- .... .
..-. .. --.- ... -
--.- ..- .-.. .
--- ..-.
- .... .
..-. .. ..- .... -
-.-. .-.. ..- .---
.. ...
```

Marseille is a plaintext-to-morse-code encoding library and command-line
tool.

Well, the CLI is really named `morsel`.

> Don't worry if you don't get do yourself a favor and find a restaurant
> that serves morsel mushrooms and feel a true delight.


## Installing

```bash
make release
```

## Usage

### command-line

```bash
morsel string
```


### Rust

```rust
use marseille::modulate;

pub fn main() {
    println!("{}", modulate("The first rule of the Fight Club is you do not talk about it".to_string(), &" "))
}
```
