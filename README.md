# Express vs Rocket Speed Experiment
## Express
To run the express server you need [node](https://nodejs.org/en/), the recommended node version for this repo is 12.x.x LTS, and I also recommend using [nvm](https://github.com/nvm-sh/nvm) to manage different node versions with ease (for windows users, use [nvm-windows](https://github.com/coreybutler/nvm-windows)).

After installing node, run:
```bash
cd express
npm install
npm run start
```

## Rocket
To run rocket you need [rust](https://www.rust-lang.org/tools/install), I highly recommend you use rustup for this, it's pretty much like nvm.

After installing rust, run:
```bash
cd rust-rocket
rustup override set nightly
cargo build --release
```
The second command will install rust nighlty, and only switch to it on this folder of your machine.

The third command installs dependencies and compiles the binary which you can run with the following command from the rust-rocket folder (unix):
```bash
target/release/rust-rocket
```
