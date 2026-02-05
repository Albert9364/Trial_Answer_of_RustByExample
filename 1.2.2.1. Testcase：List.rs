// use std::fmt; // 导入 `fmt` 模块。

// // 定义一个名为 `List` 的结构体，包含一个 `Vec`。
// struct List(Vec<i32>);

// impl fmt::Display for List {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // 使用元组索引提取值，
//         // 并创建一个指向 `vec` 的引用。
//         let vec = &self.0;

//         write!(f, "[")?;

//         // 遍历 `vec` 中的 `v`，同时用 `index` 枚举迭代索引。
//         for (index, v) in vec.iter().enumerate() {
//             // 除第一个元素外，为每个元素添加逗号。
//             // 使用 ? 运算符在出错时返回。
//             if index != 0 { write!(f, ", ")?; }
//             write!(f, "{}", v)?;
//         }

//         // 闭合左括号并返回 fmt::Result 值
//         write!(f, "]")
//     }
// }

// fn main() {
//     let v = List(vec![1, 2, 3]);
//     println!("{}", v);
// }

/*
尝试修改程序，使向量中每个元素的索引也被打印出来。新的输出应该如下所示：
[0: 1, 1: 2, 2: 3]
*/

use std::fmt; 

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (index, v) in vec.iter().enumerate() {
            if index != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", index, v)?;
        }

        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}