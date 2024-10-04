mod foo {
    // 使用 pub 将函数设为公共的，以便外部可以访问它
    pub fn my_demo_function(a: u32) -> u32 {
        a
    }

    // 可以为它设置一个别名
    pub fn my_demo_function_alias(a: u32) -> u32 {
        a
    }
}

// 重新导出函数，使得在外部可以使用相同的名称
pub use foo::my_demo_function;
pub use foo::my_demo_function_alias;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // 直接调用导出的安全函数
        assert_eq!(my_demo_function(123), 123);
        assert_eq!(my_demo_function_alias(456), 456);
    }
}
