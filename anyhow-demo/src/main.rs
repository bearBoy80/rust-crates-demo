use std::fs::read;

use anyhow::{Context, Result};
/**
 * anyhow它提供了一种简单、方便的方式来处理错误。
 * 在 Rust中，错误处理通常涉及到定义自定义错误类型、实现 std::error::Error trait 等繁琐的工作。anyhow 库旨在简化这一过程，使得错误处理变得更加直观和易于使用。
 * 常用宏：anyhow!
 * 提供一个错误上下文
 */
fn main(){
    println!("Hello, world!");
    //执行这段代码会报一个错误，错误会带一个我们自定义的上下文
    let content = anyhow_with_context();
    println!("{:?}", content);
}
fn anyhow_with_context()->Result<Vec<u8>>{
    let content = read("/data/hello_world.csv").with_context(||{
        format!("Failed to read instrs from {}", "/data/hello_world.csv")
    })?;
    Ok(content)
}
