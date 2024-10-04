struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// The `ptr` must contain an owned box of `Foo`.
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // SAFETY: The `ptr` contains an owned box of `Foo` by contract.
    // We reconstruct the box from that pointer.
    let mut ret: Box<Foo> = unsafe { Box::from_raw(ptr) };

    // 修改字段 b 的值
    ret.b = Some("hello".to_owned());

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let data = Box::new(Foo { a: 1, b: None });

        let ptr_1 = &data.a as *const u128 as usize;

        // SAFETY: We pass an owned box of `Foo`.
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };

        let ptr_2 = &ret.a as *const u128 as usize;

        // 验证两个指针相等，意味着我们成功恢复了原来的 Box<Foo>
        assert!(ptr_1 == ptr_2);
        // 验证字段 b 已被正确设置为 "hello"
        assert!(ret.b == Some("hello".to_owned()));
    }
}
