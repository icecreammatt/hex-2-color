# hex2color

```bash
Usage: hex2color <FFFFFF or (0-255) (0-255) (0-255)>
```

<img width="491" alt="Screen Shot 2023-05-01 at 6 50 34 PM" src="https://user-images.githubusercontent.com/1154569/235562540-afdbc63c-3514-474d-b5a8-a77f13273619.png">

## Building with Rust
- `cargo build`
- `cargo run FF00FF`
- `cargo install --path .`

## Run with nix
- `nix run github:icecreammatt/hex2color FFFF00`

## Build and run with nix
- `nix build`
- `./result/bin/hex2color FF0000`
