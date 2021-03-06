<h1 align="center">🍒 Clitrans </klɪ'træns/></h1>
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

* No API token required.
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

#### On Arch Linux

`clitrans` is available in the Arch User Repository. To install it from [AUR](https://aur.archlinux.org/packages/clitrans):

```
yay -S clitrans
```

#### On macOS

You can install `clitrans` with Homebrew:

```
brew tap wfxr/clitrans
brew install clitrans
```

#### From binaries

Prebuilt versions of `clitrans` for various architectures are available at [Github release page](https://github.com/wfxr/clitrans/releases).

*Note that you can try the `musl` version (which is statically-linked) if runs into dependency related errors.*

#### From source

`clitrans` is also published on [crates.io](https://crates.io). If you have Rust toolchains installed you can use `cargo` to install it from source:

```
cargo install --locked clitrans
```

If you want the latest version, install it from this repository:
```
cargo install --git https://github.com/wfxr/clitrans --locked
```

### Related Project

[ydcv](https://github.com/felixonmars/ydcv): A cli wrapper for Youdao online translate service api.

[bing-dict](https://github.com/Shawyeok/bing-dict): A cli wrapper for Bing online dictionary.

### Todo

- [x] Dynamically choosing the fastest engine.
- [ ] Support force color ouput.
- [ ] Fzf integration.
- [ ] Vim integration.
- [ ] Display the engine name?
- [ ] Support Youdao API based engine.

### License

`clitrans` is distributed under the terms of both the MIT License and the Apache License 2.0.

See the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files for license details.
