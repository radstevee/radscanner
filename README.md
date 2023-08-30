# radscanner

You've somehow landed on my pretty-much-alpha version of a Minecraft server scanner.
This is essentially my rust learning Project.

# Usage
Configure config.toml to your liking.

## Compiling manually
```
cargo build --release
./target/release/radscanner
```
## Docker
```
docker build -t radscanner .
docker create --name radscanner radscanner
```