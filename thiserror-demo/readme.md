# ThisError使用
本文将基于1.0.59版本进行介绍使用，并提供相关demo。thiserror这个crate 则进一步简化了在 Rust中定义自定义错误类型的过程。它基于derive过程宏提供了一种便捷的方式来为程序定义自定义错误类型。与手动实现std::error::Error trait 相比,使用 thiserror 可以大大减少样板代码,并提供更好的类型安全性和可读性。
## 相关宏介绍
### `#[derive(Error)]`
这是thiserror的核心宏,用于为自定义错误类型自动派生std::error::Error trait的实现。只能用于
enum、struct这两种数据结构上面。
```rust
#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid rdo_lookahead_frames {0} (expected < {})", i32::MAX)]
    InvalidLookahead(u32),
}
```
### `#[error(...)]`
这个属性宏用于定义错误变体的显示格式。它接受一个格式化字符串,可以包含占位符 {}。如果占位符中包含一个数字,它将对应该变体中的同一个字段索引。只能用于struct类上，或者enum里面的字段。
支持格式如下：
- #[error("{var}")] ⟶ write!("{}", self.var)
- #[error("{0}")] ⟶ write!("{}", self.0)
- #[error("{var:?}")] ⟶ write!("{:?}", self.var)
- #[error("{0:?}")] ⟶ write!("{:?}", self.0)
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown data store error")]
    Unknown,
}
```
### `#[error(transparent)]`
这个属性可以应用于错误变体,使其成为透明的错误包装器。这意味着当错误被显示时,它将直接显示包装的错误的内容,而不是显示自己的格式化字符串。
```rust
#[derive(Error, Debug)]
pub enum MyError {
    ...

    #[error(transparent)]
    Other(#[from] anyhow::Error),  // source and Display delegate to anyhow::Error
}
```
### `#[from]`
这个属性可以应用于单个字段,实现From traint,主要用于将其他Error转换成指定的Error。
```rust
#[derive(Error, Debug)]
pub enum MyError {
    ...
    #[error("unknown data store error")]
    Other(#[from] anyhow::Error),  // delegate anyhow::Error
}
```
### `#[source]`
这个属性可以应用于单个字段,以将其标记为错误源。当显示错误时,该字段的值将被用作错误源,而不是使用格式化字符串.如果字段是source，默认该字段被标记`#[source]`.
```rust
#[derive(Error, Debug)]
pub struct MyError {
    msg: String,
    #[source]  // optional if field name is `source`
    source: anyhow::Error,
}
```
### `#[backtrace]`
这个属性可以应用于错误定位,以启用为struct/enum添加回溯(backtrace)信息的功能。在调试模式下,错误消息将包含回溯信息,有助于诊断错误。该宏目前还是处于`nightly`，不能用于稳定版本，在rust目前的版本上面会报错，导致编译不通过。
```rust
use std::backtrace::Backtrace;

#[derive(Error, Debug)]
pub struct MyError {
    msg: String,
    backtrace: Backtrace,  // automatically detected
}
````
## 总结
这些宏提供了定义自定义错误类型的便利方式,同时还支持诸如错误包装、回溯信息等高级功能。使用 `thiserror`可以极大地简化`Rust`中的错误处理代码,提高代码的可读性和可维护性。相关代码可以[参考代码](https://github.com/bearBoy80/rust-crates-demo/tree/main/thiserror-demo)