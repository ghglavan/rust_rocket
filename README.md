# rust_rocket
rust rocket webserver for my raspberry pi

## setup

### download and install rustup
```sh
curl https://sh.rustup.rs -sSf | sh
```
### install nightly for rust (needed by rocket)
```sh
rustup install nightly
```
### add target for armv7-unknown-linux-gnueabihf
```sh
rustup target add armv7-unknown-linux-gnueabihf
```
### install cross compiler
```sh
sudo apt-get install gcc-4.7-multilib-arm-linux-gnueabihf
```
## compile
```sh
cargo build --target=armv7-unknown-linux-gnueabihf
```
