use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    //设置读取缓存大小
    let mut buf = [0; 512];

    //println!("");
    //最大接收次数 也可改成 loop
    for _ in 0..1000 {
        //定义要输出的字符串
        let recieved_message;
        //阻塞读取，将待取字符 存入buf 并得到长度buf_len
        let buf_len = stream.read(&mut buf)?;

        //收到的消息回写
        stream.write(&buf[..buf_len])?;

        //将[u8]转为string 并注意 length
        recieved_message = std::str::from_utf8(&buf[0..buf_len]).unwrap().to_string();
        //打印收到的内容
        println!("收到了信息: {}", recieved_message);
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    //创建一个对8080端口的监听 listener
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        match stream {
            //链接成功
            Ok(stream) => {
                println!("新客户端链接成功！");
                //在新线程里进行
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
            //链接失败，错误处理。
            Err(_e) => {
                println!("链接出错!");
            }
        }
    }
    Ok(())
}
