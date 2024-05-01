// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.


// use std::fs::try_exists;
use std::sync::mpsc; // 消息传递通道，提供了多生者单消费者通道来实现线程间的通信
use std::sync::Arc;// 原子引用计数，用于在多线程环境下共享数据
use std::thread;// 线程库，提供创建和操作线程的功能
use std::time::Duration;    // 用于指定线程的睡眠时间
use std::sync::Mutex;

struct Queue {
    length: u32,  // 队列长度
    first_half: Vec<u32>,// 队列的前半部分
    second_half: Vec<u32>,// 队列的后半部分
}

impl Queue {
    fn new() -> Self {   // 创建一个新的队列
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

// fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () { // 发送消息的函数
    /*这个函数将一个 `Queue` 实例和一个发送器 (`tx`) 作为参数，
    它的作用是将队列中的元素发送到 channel 中。它创建了两个线程，
    每个线程都将队列的一半元素发送到 channel 中。 
    函数的返回值是 ()，表示没有返回值。
    */
    fn send_tx(q: Queue, tx: Arc<Mutex<mpsc::Sender<u32>>>)-> 
    (thread::JoinHandle<()>, thread::JoinHandle<()>){  // 发送消息的函数
    let first_half = q.first_half.clone(); // 创建一个 Arc<T> 实例，
    let second_half = q.second_half.clone(); // 创建一个 Arc<T> 实例，
    // let qc = Arc::new(q);  
     //Arc<Queue> 是可共享的，可以被多个线程同时访问和修改,
    // let qc1 = Arc::clone(&qc); //Arc<Queue> 克隆是单独的引用计数
    // let qc2 = Arc::clone(&qc);//Arc<Queue> 克隆是单独的引用计数
    //Arc::new与Arc::clone的区别是，Arc::new会创建一个新的 Arc<T> 实例，
    // Arc:clone会创建一个新的引用计数，
    let tx1 = Arc::clone(&tx); // 创建一个发送器 (`tx`) 的克隆，
    let handle1=thread::spawn(move || {  
        /*创建一个线程，将队列的前半部分发送到 channel 中。
        这个线程会遍历队列的前半部分，并将每个元素发送到 channel 中。
        这里使用了 Arc<Queue> 来共享队列，
        以便多个线程可以同时访问和修改队列。
        move 关键字将闭包中的变量移动到新线程中，
        这样这些变量就可以在多个线程中使用。
        for循环中的代码将队列的前半部分发送到 channel 中。
        thread::sleep 函数用于让线程休眠 1 秒钟，
        以模拟发送消息的延迟。
        println! 语句用于打印发送的消息，以便在测试时观察发送过程。
         */
        for val in first_half {
            println!("sending {:?}", val);
            // tx1.send(*val).unwrap();
            tx1.lock().unwrap().send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let tx2 =Arc::clone(&tx); // 创建一个发送器 (`tx`) 的克隆，
    let handle2=thread::spawn(move || {  // 创建第二个线程
        for val in second_half {
            println!("sending {:?}", val);
            // tx.send(*val).unwrap();
            tx.lock().unwrap().send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    (handle1, handle2)  // 返回两个线程的句柄
}

fn main() {
    let (tx, rx) = mpsc::channel();
    /*创建一个发送器 (`tx`) 和接收器 (`rx`) 作为 channel。
    他们是一个多生产者单消费者通道，类型为 mpsc::Sender<u32> 和 mpsc::Receiver<u32>。
    let () = mpsc::channel(); 创建一个无缓冲区的 channel，channel类型为什么用（）定义，
    因为 Rust 不知道 channel 中的元素类型，所以需要使用 () 来表示不关心元素类型。
     */
    let tx = Arc::new(Mutex::new(tx)); // 创建一个发送器 (`tx`) 的克隆，
    let queue = Queue::new();
    // 创建一个 Queue 实例，
     
    let queue_length = queue.length;
    // 获取队列的长度

    // send_tx(queue, tx);
    let (handle1, handle2) = send_tx(queue, tx);
    handle1.join().unwrap();
    handle2.join().unwrap();
    // 调用 send_tx 函数，将队列和发送器作为参数传递给它。
    let mut total_received: u32 = 0;
    // 初始化一个变量来记录接收到的消息数量
    for received in rx {  //遍历接收到的rx,
        // 这里rx是一个无限循环，
        println!("Got: {}", received);
        total_received += 1;
    }
    /*        
    为什么而无需显式调用recv函数来接收消息？因为 Rust 中的 channel 是自动接收消息的。

    send_tx的返回值是 ()，为什么rx可以直接使用而不需要显式调用recv函数来接收消息？
    因为 Rust 中的 channel 是自动接收消息的。

    在 Rust 中，channel 是通过发送者和接收者之间的通信来传递消息的。
    当发送者调用 send 函数发送消息时，消息会被自动传递给接收者。
    这里接收者就是rx，所以当 send_tx 函数调用 send 函数发送消息时，
    消息会被自动传递给 rx，从而实现消息的传递和接收。
    在main函数中调用send_tx函数后，就可以直接使用rx，
    因此，不需要显式调用 recv 函数来接收消息。
    for循环第一步，rx会接收到两个消息，一个是1，另一个是6。

    这里创建了一个无限循环，因为rx是一个无限循环。
    每次循环，rx.recv() 都会从 channel 中接收一个消息。
    如果 channel 中没有消息，recv() 函数会阻塞当前线程，直到有新的消息到达。
    recv() 函数返回一个 Result<T, E> 类型的结果，
    如果成功接收到消息，则返回 Ok(T)，其中 T 是接收到的消息。
    如果 channel 中没有消息并且所有发送者都已经关闭，则返回 Err(E)，
    其中 E 是错误类型。
    这里使用 match 语句来处理接收到的消息。
    match 语句会根据接收到的消息类型进行匹配，并执行相应的代码块。
    这里，如果接收到的消息是 Ok(T)，则将 T 的值加到 total_received 上。
    send_tx的返回值是 ()，表示没有返回值，所以 match 语句中的 Ok(T) 分支不会执行任何操作。
     */

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
    
}
