use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 获取当前的 UNIX 时间戳
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // 设置环境变量 TEST_FOO
    env::set_var("TEST_FOO", timestamp.to_string());

    // 输出符合 Cargo 要求的键值对格式
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 通知 Cargo，重新编译时需要考虑到环境变量的变化
    println!("cargo:rerun-if-changed=build.rs");

    // 启用 `pass` 特性，使得 `tests8.rs` 中的条件编译生效
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
