use std::{backtrace::Backtrace, io, num::ParseIntError};

use thiserror::Error;
// 主要用于简化error操作处理、打印报错信息、堆栈信息
fn main() {
    println!(" hellow this error");
}

#[derive(Debug, Error)]
#[allow(dead_code)]
enum CommonError {
    //将错误信息转换成特定的格式
    #[error("Api error info : `{0}` ")]
    ApiError(String),
    //#[From]可以将错误转换成特定的Error
    #[error("An IO error occurred: {0}")]
    Io(#[from] io::Error),
    // 直接打印原始信息
    #[error(transparent)]
    Other(#[from] ParseIntError),
}
#[derive(Debug, Error)]
#[error("invalid rdo_lookahead_frames {msg}")]
pub struct MyError {
    msg: String,
    #[source]
    source: CommonError,
}

#[derive(Error, Debug)]
#[error("invalid rdo_lookahead_frames {msg}")]
pub struct CutomerError{
    msg: String,
    backtrace: Backtrace,  // automatically detected
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    //运行test，控制台将输出Api error info : `错误码：1001`
    fn test_common_api_error() {
        let api_error = CommonError::ApiError("错误码：1001".into());
        println!("{}", api_error);
    }
    #[test]
    // 运行test，将会看到Error: Io(Os { code: 2, kind: NotFound, message: "No such file or directory" })
    fn test_common_io_error()->Result<(),CommonError> {
        let result = std::fs::read_to_string("non_existent_file.txt");
        let _ = result.map_err(CommonError::Io)?;
        Ok(())
    }
    #[test]
    //运行test，将会看到
    fn test_common_other_error() -> Result<(),CommonError> {
        let result = "input".parse::<i32>()?;
        println!("{:?}", result);
        Ok(())
    }
    #[test]
    // 运行test,将会看到Error: MyError { msg: "non_existent_file.txt", source: Io(Os { code: 2, kind: NotFound, message: "No such file or directory" }) }
    fn test_my_error()->Result<(),MyError>{
        let result = std::fs::read_to_string("non_existent_file.txt");
        let _ = result.map_err(|err| MyError{
            msg: String::from("non_existent_file.txt"),
            source:CommonError::Io(err)
        })?;

        Ok(())
    }
}
