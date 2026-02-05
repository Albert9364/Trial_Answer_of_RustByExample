// use std::fmt; // 导入 `fmt`

// // 定义一个包含两个数字的结构体。派生 `Debug` 特性，
// // 以便与 `Display` 的结果进行对比。
// #[derive(Debug)]
// struct MinMax(i64, i64);

// // 为 `MinMax` 实现 `Display` 特性。
// impl fmt::Display for MinMax {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // 使用 `self.number` 引用每个位置的数据点。
//         write!(f, "({}, {})", self.0, self.1)
//     }
// }

// // 定义一个结构体，其字段可命名以便比较。
// #[derive(Debug)]
// struct Point2D {
//     x: f64,
//     y: f64,
// }

// // 同样，为 `Point2D` 实现 `Display` 特性。
// impl fmt::Display for Point2D {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // 自定义实现，只显示 `x` 和 `y`。
//         write!(f, "x: {}, y: {}", self.x, self.y)
//     }
// }

// fn main() {
//     let minmax = MinMax(0, 14);

//     println!("比较结构体：");
//     println!("Display：{}", minmax);
//     println!("Debug：{:?}", minmax);

//     let big_range =   MinMax(-300, 300);
//     let small_range = MinMax(-3, 3);

//     println!("大范围是 {big}，小范围是 {small}",
//              small = small_range,
//              big = big_range);

//     let point = Point2D { x: 3.3, y: 7.2 };

//     println!("比较点：");
//     println!("Display：{}", point);
//     println!("Debug：{:?}", point);

//     // 错误。虽然实现了 `Debug` 和 `Display`，但 `{:b}` 需要
//     // 实现 `fmt::Binary`。这行代码无法工作。
//     // println!("Point2D 的二进制表示是什么：{:b}？", point);
// }


/*
查看上述示例的输出后，参考 Point2D 结构体，向示例中添加一个 Complex 结构体。
以相同方式打印时，输出应为：
Display: 3.3 +7.2i
Debug: Complex { real: 3.3, imag: 7.2 }

Display: 4.7 -2.3i
Debug: Complex { real: 4.7, imag: -2.3 }
*/
use std::fmt;

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "real: {}, imag: {}", self.real, self.imag)
    }
}

fn main() {
    let complex1 = Complex { real: 3.3, imag: 7.2};
    let complex2 = Complex { real: 4.7, imag: -2.3};

    // Rust By Example中提示：Check the documentation for Sign/#/0 in std::fmt.（https://doc.rust-lang.org/std/fmt/#sign0）
    // {:+}：此标记用于数值类型，表示应始终打印符号。默认情况下，仅打印有符号值的负号，而省略正号或无符号值的符号。使用此标志表明始终打印精确的符号（+ 或 -）。
    println!("Display： {} {:+}i", complex1.real, complex1.imag);
    println!("Debug： {:?}", complex1);

    //输出一个空行
    println!();

    println!("Display： {} {:+}i", complex2.real, complex2.imag);
    println!("Debug： {:?}", complex2);

}