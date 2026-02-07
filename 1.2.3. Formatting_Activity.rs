// use std::fmt::{self, Formatter, Display};

// struct City {
//     name: &'static str,
//     // 纬度
//     lat: f32,
//     // 经度
//     lon: f32,
// }

// impl Display for City {
//     // `f` 是一个缓冲区，此方法必须将格式化的字符串写入其中。
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
//         let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

//         // `write!` 类似于 `format!`，但它会将格式化后的字符串
//         // 写入一个缓冲区（第一个参数）。
//         write!(f, "{}: {:.3}°{} {:.3}°{}",
//                self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
//     }
// }

// #[derive(Debug)]
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// fn main() {
//     for city in [
//         City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
//         City { name: "Oslo", lat: 59.95, lon: 10.75 },
//         City { name: "Vancouver", lat: 49.25, lon: -123.1 },
//     ] {
//         println!("{}", city);
//     }
//     for color in [
//         Color { red: 128, green: 255, blue: 90 },
//         Color { red: 0, green: 3, blue: 254 },
//         Color { red: 0, green: 0, blue: 0 },
//     ] {
//         // 一旦你为 fmt::Display 添加了实现，就把这里改成使用 {}。
//         println!("{:?}", color);
//     }
// }

/*
为上面的 Color 结构体实现 fmt::Display trait，使输出显示如下：
RGB (128, 255, 90) 0x80FF5A
RGB (0, 3, 254) 0x0003FE
RGB (0, 0, 0) 0x000000
*/

use std::fmt::{self, Formatter, Display};

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "RGB ({0}, {1}, {2}) 0x{0:0>2X}{1:0>2X}{2:0>2X}", self.red, self.green, self.blue)
    }
}

fn main() {
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ] {
        println!("{}", color);
    }
}