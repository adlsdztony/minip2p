use std::collections::HashMap;
use std::net::UdpSocket;

fn buf_to_string(buf: &[u8]) -> String {
    let mut s = String::new();
    buf.iter().for_each(|&x| s.push(x as char));
    s
}

fn server(port: u16) {
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", port)).unwrap();

    // hashmap to store client addr
    let mut client_map: HashMap<String, String> = HashMap::new();

    // get port after bind
    let addr = socket.local_addr().unwrap();
    println!("server listen on: {}", addr);
    
    loop {
        let mut buf = [0; 1024];
        let (amt, src) = socket.recv_from(&mut buf).unwrap();
        let code = buf_to_string(&buf[..amt]);
        // check if code in map
        if client_map.contains_key(&code) {
            // ckeck if addr is same
            if client_map.get(&code).unwrap() == &src.to_string() {
                println!("client: {:?} // Code {} already in map", src, code);
                continue;
            }
            // send addr to client
            let msg = client_map.get(&code).unwrap().clone();
            socket.send_to(msg.as_bytes(), src).unwrap();

            // send addr to old client
            let msg = src.to_string();
            socket.send_to(msg.as_bytes(), client_map.get(&code).unwrap()).unwrap();

            // delete from map
            client_map.remove(&code);

            println!("client: {:?} // Code {} remove from map", src, code)
        } else {
            // add to map
            client_map.insert(code.clone(), src.to_string());
            socket.send_to(b"wait", src).unwrap();
            println!("client: {:?} // Code {} add to map", src, code);
        }

    }
}

fn main() {
    // get args
    let args: Vec<String> = std::env::args().collect();
    // -p: port
    let mut port = 0;
    for i in 0..args.len() {
        if args[i] == "-p" {
            port = args[i + 1].parse::<u16>().unwrap();
        }
    }
    server(port);
}