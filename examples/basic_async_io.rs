use anyhow::Result;
use serde_yaml::Value;
use tokio::{fs, try_join}; // 使用 tokio::fs 返回一个 Future

/// 在这段代码里，我们使用了 tokio::fs，而不是 std::fs，
/// tokio::fs 的文件操作都会返回一个 Future，然后可以 join 这些 Future，得到它们运行后的结果。
/// join / try_join 是用来轮询多个 Future 的宏，它会依次处理每个 Future，
/// 遇到阻塞就处理下一个，直到所有 Future 产生结果。
#[tokio::main]
async fn main() -> Result<()> {
    // 读取 Cargo.toml, IO 操作1
    let f1 = fs::read_to_string("./Cargo.toml");
    // 读取 Cargo.lock, IO 操作2
    let f2 = fs::read_to_string("./Cargo.lock");

    // 读取 Cargo.toml，IO 操作 1
    //     let content1 = fs::read_to_string("./Cargo.toml").await?;
    // 读取 Cargo.lock，IO 操作 2
    //     let content1 = fs::read_to_string("./Cargo.lock").await?;
    // 这样写的话，和第一版同步的版本没有区别，因为 await 会运行 Future 直到 Future 执行结束，
    // 所以依旧是先读取 Cargo.toml，再读取 Cargo.lock，并没有达到并发的效果。
    let (content1, content2) = try_join!(f1, f2)?;

    // 计算
    let yaml1 = toml2yaml(&content1)?;
    let yaml2 = toml2yaml(&content2)?;

    // 写入/tmp/Cargo.yml, IO 操作3
    let f3 = fs::write("/tmp/Cargo.yml", &yaml1)?;
    // 写入/tmp/Cargo.lock, IO 操作3
    let f4 = fs::write("/tmp/Cargo.lock", &yaml2)?;
    try_join!(f3, f4)?;

    // 打印
    println!("{}", yaml1);
    println!("{}", yaml2);

    Ok(())

}

fn toml2yaml(content: &str) -> Result<String> {
    let value: Value = toml::from_str(content)?;
    Ok(serde_yaml::to_string(&value)?)
}
