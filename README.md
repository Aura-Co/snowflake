# Snowflake
&nbsp;&nbsp;&nbsp;&nbsp;基于 Twitter 雪花算法的 ID 生成器。多线程 / 线程安全的。

[![Build Status](https://travis-ci.org/hanskorg/snowflake-rust.svg?branch=master)](https://travis-ci.org/hanskorg/snowflake-rust)

## Usage

&nbsp;&nbsp;&nbsp;&nbsp;Add this to your `Cargo.toml`:

```toml
[dependencies]
snowflake = { git = "https://github.com/Aura-Co/snowflake", branch = "master" }
```

&nbsp;&nbsp;&nbsp;&nbsp;Kubernetes 中使用，会基于容器 ip 的最后两节，生成可用的 worker_id：：

```rust
 let mut id_gen = SnowFlakeId::kubernetes(1);
 println!("{}", id_gen.generate_id().unwrap());
```

&nbsp;&nbsp;&nbsp;&nbsp;基本方法：

```rust
 let workerId: i64 = 1;
 let datacenterId: i64 = 1;
 let mut id_gen = SnowFlakeId::new(workerId, datacenterId);
 println!("{}", id_gen.generate_id().unwrap());
```

## Benchmarks
&nbsp;&nbsp;&nbsp;&nbsp;笔者 17款 4核 标压 macbook pro，以 lib 方式使用：500w/s。


## Other
&nbsp;&nbsp;&nbsp;&nbsp;Fork 自 hanskorg/snowflake-rust，并持续迭代。
