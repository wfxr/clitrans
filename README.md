<h1 align="center">🐙 Clitrans </klɪ'træns/></h1>
<p align="center">
    <em>Yet another command-line translator.</em>
<p align="center">
    <a href="https://github.com/wfxr/clitrans/actions?query=workflow%3ACICD">
        <img src="https://github.com/wfxr/clitrans/workflows/CICD/badge.svg" alt="CICD"/>
    </a>
    <img src="https://img.shields.io/crates/l/clitrans.svg" alt="License"/>
    <a href="https://crates.io/crates/clitrans">
        <img src="https://img.shields.io/crates/v/clitrans.svg?colorB=319e8c" alt="Version">
    </a>
    <a href="https://github.com/wfxr/clitrans/releases">
        <img src="https://img.shields.io/badge/platform-%20Linux%20|%20OSX%20|%20Win%20-orange.svg" alt="Platform"/>
    </a>
</p>

### Features

* No api token required.
* Integrated pronunciation function (with `audio` feature enabled).
* Multiple translate engine support.
* [Multi platforms](https://github.com/wfxr/clitrans/releases) support.

### Usage

```
$ clitrans hello
hello
/heˈləʊ/

int.  * 你好
      * 喂
      * 您好
      * 哈喽

 Web  * 哈罗
      * 哈啰
      * 大家好

Source URL:
      * https://cn.bing.com/dict/search?q=hello&mkt=zh-cn
```

Run `clitrans --help` to view detailed usage.

### Installation

#### On Arch Linux [WIP]

`clitrans` is available in the Arch User Repository. To install it from [AUR](https://aur.archlinux.org/packages/clitrans):

```
yay -S clitrans
```

#### On macOS [WIP]

You can install `clitrans` with Homebrew:

```
brew tap wfxr/clitrans
brew install clitrans
```

#### From binaries

Prebuilt versions of `clitrans` for various architectures are available at [Github release page](https://github.com/wfxr/clitrans/releases).

*Note that you can try the `musl` version (which is statically-linked) if runs into dependency related errors.*

#### From source

`clitrans` is also published on [crates.io](https://crates.io). If you have Rust toolchains (1.40 or above) installed you can use `cargo` to install it from source:

```
cargo install --locked clitrans --features audio
```

If you want the latest version, clone this repository and run `cargo build --release`.

### Related Project

[ydcv](https://github.com/felixonmars/ydcv): A cli wrapper for Youdao online translate (Chinese <-> English) service api.

### License

`clitrans` is distributed under the terms of both the MIT License and the Apache License 2.0.

See the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files for license details.
