use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// 错误类型定义
#[derive(Debug, PartialEq)]
enum IntoColorError {
    BadLen,
    IntConversion,
}

// 元组实现
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;

    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        let (r, g, b) = tuple;
        if (0..=255).contains(&r) && (0..=255).contains(&g) && (0..=255).contains(&b) {
            Ok(Color {
                red: r as u8,
                green: g as u8,
                blue: b as u8,
            })
        } else {
            Err(IntoColorError::IntConversion)
        }
    }
}

// 数组实现
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;

    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        let [r, g, b] = arr;
        if (0..=255).contains(&r) && (0..=255).contains(&g) && (0..=255).contains(&b) {
            Ok(Color {
                red: r as u8,
                green: g as u8,
                blue: b as u8,
            })
        } else {
            Err(IntoColorError::IntConversion)
        }
    }
}

// 切片实现
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;

    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            return Err(IntoColorError::BadLen);
        }

        let r = slice[0];
        let g = slice[1];
        let b = slice[2];

        if (0..=255).contains(&r) && (0..=255).contains(&g) && (0..=255).contains(&b) {
            Ok(Color {
                red: r as u8,
                green: g as u8,
                blue: b as u8,
            })
        } else {
            Err(IntoColorError::IntConversion)
        }
    }
}

fn main() {
    // 测试元组转换
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1);

    // 测试数组转换
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2);

    // 测试切片转换
    let v = vec![183, 65, 14];
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3);

    // 测试切片转换使用 TryInto
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4);
}
