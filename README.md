# vanity-addresses-solver


## Prerequisites
cargo 1.76.0    
starkli 0.3.2   

Env variables to set    
STARKNET_NETWORK=*** or use --account \<ACCOUNT>   
STARKNET_KEYSTORE=*** or use --keystore \<KEYSTORE>    


## Usage

Build release to optimise performance
```bash
$ cargo build --release
./target/release/vanity-addresses-solver --help
```


## Example
```bash
cargo build && ./target/debug/vanity-addresses-solver -p secret -c 0x04a7bc5855f74ff5b4db70c1c9d4acd24598022f246fbb168a4ee43a96d972b3 -a "str:blabla 0x123456"
```
