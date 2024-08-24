use mini_redis_zh::{clients::Client, DEFAULT_PORT};

use bytes::Bytes;

use std::str;
use std::time::Duration;
use std::num::ParseIntError;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name="mini_redis_cli",
    version,
    author,
    about="Issue Redis commands",
    )]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    #[arg(id = "hostname", long, default_value = "127.0.0.1")]
    host: String,

    #[arg(long, default_value_t = DEFAULT_PORT)]
    port: u16,
}

#[derive(Debug, Subcommand)]
enum Command {
    Ping {
        /// Message to ping. -> 发送ping
        msg: Option<Bytes>,
    },

    /// Get the value of key. -> 获取key的值
    Get {
        /// Name of ket to get. -> 要获取的key
        key: String,
    },

    /// Set the value of key. -> 设置key的值
    Set {
        /// Name of key to set. -> 设置的key
        key: String,

        /// Value to set. -> 要设置的值
        value: Bytes,

        /// Expire the value after specified amount of time. -> 在指定时间后使值过期
        #[arg(value_parser = duration_from_ms_str)]
        expires: Option<Duration>,
    },

    /// Publish to send a message to a specific channel. -> 向特定频道发布消息
    Publish {
        /// Channel to publish. -> 要发布的频道
        channel: String,

        /// Message to publish. -> 要发布的消息
        message: Bytes,
    },

    /// Subscribe a client to a specific channel or channels. -> 订阅频道
    Subscribe {
        /// Specific channel to subscribe. -> 要订阅的频道
        channels: Vec<String>,
    },
}

/// Entry point for CLI tool.
/// CLI 工具的入口点
///
/// The `[tokio::main]` annotation signals that the Tokio runtime should be
/// started when the function is called. The body of the function is executed
/// within the newly spawned runtime.
/// [tokio::main]注解表示在调用函数时应该启动 Tokio 运行时。
/// 函数的主体在新生成的运行时中执行
///
/// `flavor = "current_thread"` is used here to avoid spawning background
/// threads. The CLI tool use case benefits more by being lighter instead of multi-threads.
/// 这里使用 'flavor = “current_thread”' 来避免生成后台线程。
/// CLI 工具用例通过更轻量级而不是多线程而受益更多。

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Enable logging -> 启用日志记录
    tracing_subscriber::fmt::try_init();

    // Parse the command line arguments -> 解析命令行参数
    let cli = Cli::parse();

    // Get the remote address to connect to -> 获取要连接的远程地址
    let addr = format!("{}:{}", cli.host, cli.port);

    // Establish a connection to the server -> 建立与服务器的连接
    let mut client = Client::connect(&addr).await?;

    // Process the command -> 处理命令
    match cli.command {
        // Command Ping -> 命令Ping
        Command::Ping {msg} => {
            let value = client::ping(msg).await?;
            // if the value is a string, print it as a string -> 如果值是字符串，则将其作为字符串打印
            if let Ok(string) = str::from_utf8(&value) {
                println!("\"{}\"", string);
            } else {
                println!("{:?}", value)
            }
        }

        // Command Get -> 命令Get
        Command::Get { key} => {
            if let Some(value) = client::get(&key).await? {
                // if the value is a string, print it as a string -> 如果值是字符串，则将其作为字符串打印
                if let Ok(string) = str::from_utf8(&value) {
                    println!("\"{}\"", string);
                } else {
                    println!("{:?}", value)
                }
            } else {
                println!("(nil)");
            }
        }

        // Command Set -> 命令Set
        Command::Set {
            key,
            value,
            expires: None} => {
            client.set(&key, value).await?;
            println!("Ok");
        }
        Command::Set {
            key,
            value,
            expires: Some(expires)} => {
            client.set_expires(&key, value, expires).await?;
            println!("Ok");
        }

        /// Command Publish -> 命令Publish
        Command::Publish { channel, message} => {
            // publish the message to the channel -> 将消息发布到频道
            client.publish(&channel, message).await?;
            println!("Ok");
        }

        /// Command Subscribe -> 命令Subscribe
        Command::Subscribe { channels, .. } => {
            if channels.is_empty() {
                return Err("channel(s) must be provided".into());
            }
            let mut subscriber = client.subscribe(channels).await?;
            // await messages on channels -> 在频道上等待消息
            // next_message() -> 下一条消息
            while let Some(message) = subscriber.next_message().await {
                println!("got message from the channel: {}; message = {:?}", message.channel, message.content);
            }
            Ok(())
        }
    }
}

fn duration_from_ms_str(src: &str) -> Result<Duration, ParseIntError> {
    // Parse the string as a number -> 将字符串解析为数字
    let ms = src.parse::<u64>()?;
    // Convert the number to a Duration -> 将数字转换为Duration
    Ok(Duration::from_millis(ms))
}