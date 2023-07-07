# Astreum Node

```
*      .       *    .               *     .    *          *
.  .        .           *    .     *  .            .
    *   .      *           *               * .       *    .   .
    .                     *    .    * .            .         .   .   .

 .vvv.    .vvv.  .vvvvv.  .vvvv.   .vvvv.  .v   v.  .v.     .v.
.v   v.  .v         v     .v   v.  .v      .v   v.  .v v   v v.
.vvvvv.   .vv.      v     .vvvv.   .vvv.   .v   v.  .v  v v  v.
.v   v.      v.     v     .v  v.   .v      .v   v.  .v   v   v.
.v   v.  .vvv.      v     .v   v.  .vvvv.   .vvv.   .v       v.  .v.
```

Official Astreum Node written in Rust.

## Authors

- Roy R. O. Okello: [Email](mailto:royokello@protonmail.com), [GitHub](https://github.com/royokello) & [Twitter](https://twitter.com/RealOkello)

## Features

- Create & View Accounts
- Create Transactions
- Sync Blockchain
- Validate Blockchain
- Stake & Withdraw Solar from Consensus Account

## Usage

Ensure the following prerequisites are installed on your device,

- [Rust](https://www.rust-lang.org/tools/install)
- [Git](https://git-scm.com/downloads)

Steps:

- Clone the repo with `git clone https://github.com/astreum/astreum-rs.git`
- Open the repo with `cd astreum-rs`
- Run through Cargo with `cargo run [command]`
- Update repo with `git pull --all`

CLI Commands:

```
help .................................................. lists commands
new ................................................... creates a new account
sync [chain] .......................................... validates the blockchain
mine [chain] [address] ................................ extends the blockchain
stake [chain] [address] [value] ....................... adds stake
withdraw [chain] [address] [value] .................... removes stake
send [value] on [chain] from [address] to [address] ... send solar
```

## License

MIT License

Copyright (c) 2023 Astreum Foundation

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

## Disclaimer

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

Roy R. O. Okello
12023-07-07
