/// 注意，上面的代码我们混用了 #[tokio::main] 和 futures:executor::block_on，
/// 这只是为了展示 Future 使用的不同方式，在正式代码里，不建议混用不同的 executor，会降低程序的性能，
/// 还可能引发奇怪的问题。
use futures::executor::block_on;
use std::future::Future;
use std::process::Output;

#[tokio::main]
async fn main() {
    let name1 = "liwei".to_string();
    let name2 = "jimny".to_string();

    say_hello1(&name1).await;
    say_hello2(&name2).await;

    // Future 除了可以用 await 来执行外， 还可以用 executor 执行
    block_on(say_hello1(&name1));
    block_on(say_hello2(&name2));
}

async fn say_hello1(name: &str) -> usize {
    println!("Hello, {}!", name);
    42
}

// async fn 关键字相当于一个返回 impl Future<Output> 的语法糖
fn say_hello2<'fut>(name: &'fut str) -> impl Future<Output = usize> + 'fut {
    async move {
        println!("Hello, {}!", name);
        42
    }
}