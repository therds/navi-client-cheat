# navi-client-cheat
navi add client cheat local  (readme not complete, navi-2.25.0b1-cheat.aarch64 worr for termux android arm64)


## install rust on termux andriod
$ Installed-Size: 295 MB
  Download size: 82.4 MB
  Space needed: 410 MB
...
** Check Install **
$ cargo -V
cargo 1.87.0 (99624be96 2025-05-06) (built from a source tarball)
$ which cargo
/data/data/com.termux/files/usr/bin/cargo

## Install  hello-rust for test cargo tool
$ mkdir rust ; cd rust
$ cargo new hello-rust
  Creating binary (application) `hello-rust` package
  note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
$ cd hello-rust
$ cargo run
Compiling hello-rust v0.1.0 (/data/data/com.termux/files/home/rust/hello-rust)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.34s
Running `target/debug/hello-rust`
Hello, world!
$ ./target/debug/hello-rust
$ cargo build --release
   Compiling hello-rust v0.1.0 (/data/data/com.termux/files/home/rust/hello-rust)
    Finished `release` profile [optimized] target(s) in 1.12s

## Start clone navi  from git hub (latest version)  
```bash
$ git clone https://github.com/denisidoro/navi
...
$ cargo run
or ./target/debug/navi -
- If error , Try check config or create form command
$ navi info config-example > "$(navi info config-path)"
$ cat $(navi info config-path)   
```
- check you config navi

* navi on andriod
$ cargo run -- --cheat.sh cheat

$ cargo run -- --tldr cheat
if error , check you install and config tldr

## add cheat client 
1. download pact navi-2.25.0b1-cheat.aarch64.patch
  $ wget navi-2.25.0b1-cheat.aarch64.patch
2. git and cheat.sh to src/client/cheat.sh
3. test run
   $navi --cheat tar
4. build


    




