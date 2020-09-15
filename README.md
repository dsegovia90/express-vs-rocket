# Express vs Rocket Speed Experiment
## Express
To run the [express](https://expressjs.com/) server you need [node](https://nodejs.org/en/).

The recommended node version for this repo is 12.x.x LTS, and I also recommend using [nvm](https://github.com/nvm-sh/nvm) to manage different node versions with ease (for windows users, use [nvm-windows](https://github.com/coreybutler/nvm-windows)).

After installing node, run:
```bash
cd express
npm install
npm run start
```

## Rocket
To run [rocket](https://rocket.rs/) you need [rust](https://www.rust-lang.org/tools/install), I highly recommend you use rustup for this, it's pretty much like nvm. If you are using windows, keep in mind that you need to install [Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/).

After installing rust via rustup, run:
```bash
cd rust-rocket
rustup override set nightly
cargo build --release
```
The second command will install rust nighlty, and only switch to it on this folder of your machine.

The third command installs dependencies and compiles the binary/executable which you can run with the following command from the rust-rocket folder:

Unix:
```bash
target/release/rust-rocket
```

Windows:
```cmd
target/release/rust-rocket.exe
```

Please open an issue if you found a bug, typo, instructions are not clear, or your build is not working properly.
