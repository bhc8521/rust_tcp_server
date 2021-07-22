use std::io::{Read, Write};  //导入io中的读和写模块
use std::net::{TcpListener, TcpStream};  //导入tcp监听和流模块
use std::thread; // 导入线程模块

fn main() {  //主函数入口
    let listener = TcpListener::bind("127.0.0.1:8080").expect("can not launch a tcp server!");  //绑定监听端口，错误则报错
    let mut thread_vec: Vec<thread::JoinHandle<()>> = vec![];
    for stream in listener.incoming() {  //incoming()返回的是迭代器,每个stream表示一个客户端
        let stream = match stream {  // 对stream进行模式匹配
            Ok(v) => v,  // 如果没问题则返回result中的值
            Err(_) => panic!("client error")  //有问题则抛出恐慌
        };    
        let handle = thread::spawn(|| {  // 创建线程，这里使用了闭包
            handle_stream(stream).unwrap_or_else(|error| eprintln!("{:?}", error));  // 将stream借用给这个函数，并对函数返回的错误又进行了一个闭包处理
        });
        thread_vec.push(handle);  // 将线程推进vector
    }

    for handle in thread_vec {  // 线程阻塞，防止主程序先结束
        handle.join().unwrap();
    }
}

fn handle_stream(mut stream: TcpStream) -> Result<(), ()> {  //自定义一个函数， stream为引用，返回Result
    let mut buf = [0;512]; // 创建一个初始值全为0长度为512的数组，在下文中编译器将识别其类型为u8
    for _ in 0..1000 {  // 为这个客户端接受1000次消息
        let bytes_read = stream.read(&mut buf).expect("stream read error");  //读取一次内容，或抛出可恢复错误
        stream.write(&buf[..bytes_read]).expect("stream write error"); //写入读取出的内容，或抛出可恢复错误
        let buffer = buf.to_vec(); // 将buf转为u8的vector
        println!("{}", String::from_utf8(buffer).expect("Could not write buffer as string"));  //将buffer的vector转成String输出
    }
    Ok(()) // 返回一个正常的tuple空值
}
