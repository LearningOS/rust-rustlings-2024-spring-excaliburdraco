// arc1.rs
//
// In this exercise, we are given a Vec of u32 called "numbers" with values
// ranging from 0 to 99 -- [ 0, 1, 2, ..., 98, 99 ] We would like to use this
// set of numbers within 8 different threads simultaneously. Each thread is
// going to get the sum of every eighth value, with an offset.
//
// The first thread (offset 0), will sum 0, 8, 16, ...
// The second thread (offset 1), will sum 1, 9, 17, ...
// The third thread (offset 2), will sum 2, 10, 18, ...
// ...
// The eighth thread (offset 7), will sum 7, 15, 23, ...
//
// Because we are using threads, our values need to be thread-safe.  Therefore,
// we are using Arc.  We need to make a change in each of the two TODOs.
//
// Make this code compile by filling in a value for `shared_numbers` where the
// first TODO comment is, and create an initial binding for `child_numbers`
// where the second TODO comment is. Try not to create any copies of the
// `numbers` Vec!
//
// Execute `rustlings hint arc1` or use the `hint` watch subcommand for a hint.

#![forbid(unused_imports)] // Do not change this, (or the next) line.  // 禁止未使用的导入
use std::sync::Arc; // 导入 Arc 类型，用于共享所有权
use std::thread; // 导入 thread 模块，用于创建线程

fn main() {
    let numbers: Vec<_> = (0..100u32).collect(); // 创建一个从 0 到 99 的向量
    let shared_numbers = Arc::new(numbers); // // 将向量包装成 Arc，以便共享所有权
    let mut joinhandles = Vec::new(); // 创建一个空的 Vector，用于存储线程句柄

    for offset in 0..8 {
        // 遍历 0 到 7
        let child_numbers = Arc::clone(&shared_numbers); // // 克隆 Arc，以便在子线程中使用
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}

/*这个程序做了以下事情：
1. 创建了一个从 0 到 99 的向量。
2. 将向量包装成 `Arc`，以便共享所有权。
3. 创建了 8 个子线程，每个线程计算 offset 余数为当前值的数字之和。
4. 将每个线程的句柄添加到一个 Vector 中。
5. 等待所有线程完成，然后打印结果。
这个程序演示了如何使用 `Arc` 和 `thread` 来并发地执行任务。 */
