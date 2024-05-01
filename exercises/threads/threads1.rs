// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.



use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {  //handles中存储的是线程句柄JoinHandle<T>，它表示一个可以等待的线程。
            /*我们初始化一个名为handles的向量来存储线程句柄。然后，我们使用for循环创建10个线程。
            thread::spawn函数用于创建一个新线程，并返回一个线程句柄。
            move || { ... }是一个闭包，它捕获变量i的值，并在新线程中运行。 
            在for循环中，我们创建了10个线程。每次调用thread::spawn时，
            它都会返回一个JoinHandle，我们将这个JoinHandle推入handles向量中。
            */
            let start = Instant::now();//创建一个获取当前时间的一个Instant对象。
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis()
            /*线程睡眠250毫秒，然后打印线程完成的消息，并返回线程运行的时间。
             */
        }));
    }

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // TODO: a struct is returned from thread::spawn, can you use it?
        results.push(handle.join().unwrap());   // unwrap() will panic if the thread panics
    }
    /*handle.join().unwrap()等待线程完成，并返回线程的返回值。
    handle.join()会阻塞主线程直到对应的子线程结束。如果子线程成功结束，unwrap()会返回子线程的返回值；如果子线程发生恐慌，unwrap()会引发恐慌。 */

    if results.len() != 10 {    
        panic!("Oh no! All the spawned threads did not finish!");   // panic if the number of results is not 10
    }

    println!();   // print a newline for spacing  
    for (i, result) in results.into_iter().enumerate() {   //enumerate() returns an iterator of (index, value) pairs
        /*results.into_iter().enumerate() returns an iterator of (index, value) pairs.
         */
        println!("thread {} took {}ms", i, result);
    }
}
