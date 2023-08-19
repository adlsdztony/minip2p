use std::io::Write;

type PassError = Box<dyn std::error::Error>;

use minip2p::{Client, buf_to_string};


fn chat(socket: Client) {
    // multi thread
    let socket2 = socket.try_clone().unwrap();
    println!("start thread");
    std::thread::spawn(move || {
        loop {
            let mut buf = [0; 1024];
            let amt_warp = socket2.recv(&mut buf);
            if amt_warp.is_err() {
                // std::thread::sleep(std::time::Duration::from_secs(1));
                continue;
            }
            let amt = amt_warp.unwrap();
            let msg = buf_to_string(&buf[..amt]);
            // stripe \n
            let msg = msg.trim_end_matches('\n');
            
            // print msg but keep cursor
            print!("\r\n<< {}", msg);
            std::io::stdout().flush().unwrap();
            print!("\r\n>> ");
            std::io::stdout().flush().unwrap();


        }
    });

    loop {
        let mut msg = String::new();
        print!(">> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut msg).unwrap();
        let msg = msg.trim_end_matches('\n');
        socket.send(msg.as_bytes()).unwrap();
    }
}


fn main() -> Result<(), PassError>{
    // get args
    let args: Vec<String> = std::env::args().collect();
    // -r: relay server ip:port
    // -p: port
    // -c: code
    let mut relay_server = String::from("");
    let mut port = 0;
    let mut code = String::from("123");
    for i in 0..args.len() {
        if args[i] == "-r" {
            relay_server = args[i + 1].clone();
        } else if args[i] == "-p" {
            port = args[i + 1].parse::<u16>().unwrap();
        } else if args[i] == "-c" {
            code = args[i + 1].clone();
        }
    }

    let client = Client::new(port);
    client.connect_to_relay(relay_server)?
        .pair_with(code)?;
    chat(client);
    Ok(())
}
