use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::fs;

/// 一个实用的文件处理工具（Clap 实战示例）
#[derive(Parser)]
#[command(name = "file-tool")]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// 开启详细日志（全局参数，对所有子命令生效）
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
#[derive(Debug)]
enum Commands {
    /// 查看文件信息
    Info {
        /// 文件或目录路径
        #[arg(value_name = "PATH")]
        path: PathBuf,
    },

    /// 复制文件
    Copy {
        /// 源文件路径
        #[arg(value_name = "SRC")]
        src: PathBuf,

        /// 目标路径
        #[arg(value_name = "DST")]
        dst: PathBuf,

        /// 强制覆盖已存在的目标文件
        #[arg(short, long)]
        force: bool,
    },

    /// 简单问候（基础示例，方便测试）
    Greet {
        /// 要问候的人名
        #[arg(short, long)]
        name: String,

        /// 问候次数
        #[arg(short, long, default_value_t = 1)]
        count: u8,
    },
}

fn main() {
    let cli = Cli::parse();

    if cli.verbose {
        eprintln!("[VERBOSE] 正在执行命令: {:?}", cli.command);
    }

    match cli.command {
        Commands::Info { path } => {
            match fs::metadata(&path) {
                Ok(meta) => {
                    println!("路径      : {}", path.display());
                    println!("类型      : {}", if meta.is_dir() { "目录" } else { "文件" });
                    println!("大小      : {} 字节", meta.len());
                    if let Ok(modified) = meta.modified() {
                        println!("修改时间  : {:?}", modified);
                    }
                }
                Err(e) => {
                    eprintln!("错误: 无法获取信息 → {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Copy { src, dst, force } => {
            if !src.exists() {
                eprintln!("错误: 源文件不存在 → {}", src.display());
                std::process::exit(1);
            }

            if dst.exists() && !force {
                eprintln!("错误: 目标已存在，请添加 --force 强制覆盖");
                std::process::exit(1);
            }

            match fs::copy(&src, &dst) {
                Ok(bytes) => {
                    println!("成功复制 {} 字节", bytes);
                    println!("从 {} → {}", src.display(), dst.display());
                }
                Err(e) => {
                    eprintln!("复制失败: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Greet { name, count } => {
            for _ in 0..count {
                println!("Hello {}!", name);
            }
        }
    }
}