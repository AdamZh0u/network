# Rust

## Librarys
- https://github.com/memgraph/memgraph

## 教程

- [安装 - Rust 程序设计语言 中文版 --- Installation - The Rust Programming Language](https://doc.rust-lang.org/stable/book/ch01-01-installation.html)
- 模块
  - [std::prelude - Rust](https://doc.rust-lang.org/stable/std/prelude/index.html)



## Cargo

- [Semantic Versioning 2.0.0 | Semantic Versioning](https://semver.org/)

```bash
cargo --version
rustr main.rs

cargo new hello_cargo
cd hello_cargo

cargo init # 已经存在的文件夹
cargo build # 编译
./target/debug/main # 运行

cargo run # 编译运行

cargo check

cargo build --release # 构建到target/release
```

```rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();  // mutable, 
  // ：：new 行中的 ：： 语法指示 new 是 String 类型的关联函数。关联函数是在类型上实现的函数，在本例中为 String。完全创建了一个可变变量，该变量当前绑定到 String 的新空实例

    io::stdin()
        .read_line(&mut guess) //&mut guess 作为参数传递给 read_line 以告诉它要将用户输入存储在哪个字符串中
  // &表示此参数是一个引用，它为您提供了一种方法，可以让代码的多个部分访问一个数据，而无需多次将该数据复制到内存中
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

- 由于 *Cargo.lock* 文件，您的项目将保持在 0.8.5 版本，直到您明确升级为止
- cargo update



```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; //parse 会返回一个 Result type 和 Result 是具有变体 Ok 和 Err 的枚举

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

