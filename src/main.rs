extern crate rand;

use rand::Rng;
use std::env;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str::from_utf8;


fn client(ip: &String) {
    match TcpStream::connect(ip) {
        Ok(mut stream) => {
			
			let s_hash: String = get_hash();
	
			let mut s_key: String = get_key();
			
			stream.write((&s_hash).as_bytes()).unwrap();

			let mut data = [0 as u8; 15];
			stream.read(&mut data);
			println!("{}",from_utf8(&data[0..15]).unwrap().to_string());
	
			
			for _i in 0..3 {
				let mut data1 = [0 as u8; 10];
				println!("Sending: hash- {}, key- {}|",s_hash,s_key);
				
				stream.write((&s_key).as_bytes()).unwrap();
				
				stream.read(&mut data1);

				let message = from_utf8(&data1[0..10]).unwrap();
				
				println!("Test message: {}; out key: {}\n",message.to_string(),next_session_key(&s_hash,&s_key));
				
				if message == next_session_key(&s_hash,&s_key) {
					println!("<<<Key match>>>\n");
					s_key = next_session_key(&s_hash,&s_key);
				} else {
					break
				}

			}

        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    //println!("Terminated.\n");
}


fn handle_request(mut stream: TcpStream) {
    let mut data = [0 as u8; 5];

	stream.read(&mut data);
	stream.write("Hash recieved!\n".as_bytes()).unwrap();
	let get = from_utf8(&data).unwrap();
	let hash = get.to_string();

	for _i in 0..3 {
		let mut income = [0 as u8; 10];
		stream.read(&mut income);

		let ar= from_utf8(&income).unwrap();
		let key= ar.to_string();
		println!("Incoming data:");
		
		println!("hash: {}, key: {}|\n",hash,from_utf8(&income).unwrap());
		let newmessage: String= next_session_key(&hash,&key);

		stream.write((&newmessage).as_bytes()).unwrap();
	}

}

fn server(port: &String) {
    let listener = TcpListener::bind("127.0.0.1:".to_string()+&port).unwrap();

    println!("Server listening on port: {}\n",port);
	
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
					handle_request(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}

fn get_key() -> String{
    let mut result="".to_string();
	let mut rng= rand::thread_rng();
	
	for _i in 1..11{
	    let mut temp: f64 =rng.gen();
	    temp = temp*9.0;
		result = result + &(temp as i64).to_string();
	}
	return result.to_string()
}

fn get_hash() -> String{
	let mut li = "".to_string();
	let mut rng = rand::thread_rng();
	for _i in 0..5{
	    let mut temp: f64 =rng.gen();
	    temp = temp*6.0;
		li = li + &(temp as i64).to_string();
	}
	return li
}

fn calc_hash(session_key: &String, val: i64) -> String{
	let mut result = "".to_string();
	if val == 1 {
		let t1: i64 = session_key[0..5].parse().unwrap();
		let temp: String = "00".to_string() + &(t1 % 97).to_string();
		return temp[temp.len()-2..temp.len()].to_string();
	} else if val == 2 {
		for i in 1..session_key.len() {
			result+= &session_key[session_key.len()-i..session_key.len()-i+1].to_string();
		}
		return result + &session_key[0..1];
	} else if val == 3 {
		return session_key[session_key.len()-5..session_key.len()].to_string() + &session_key[0..5]
	} else if val == 4 {
		let mut num: i64 = 0;
		for i in 1..9 {
			let temp: i64 = session_key[i..i+1].parse().unwrap();
			num += temp + 41;
		}
		return num.to_string();
	} else if val == 5 {
		let mut num: i64 = 0;
		for i in 0..session_key.len() {
			let mut ch = (( session_key.chars().nth(i).unwrap() as u8) ^43) as char;
			if !ch.is_numeric() {
				ch = (ch as u8) as char;
			}
			num += ch as i64;
		}
		return num.to_string();
	}
	let temp: i64 = session_key.parse().unwrap();
	return (temp + val).to_string();
}

fn next_session_key(hash: &String,session_key: &String)-> String{
	let mut result: i64 = 0;
	for i in 0..hash.len() {
        let temp: i64 = calc_hash(&session_key, hash.chars().nth(i).unwrap().to_digit(10).unwrap() as i64).parse().unwrap();
		result += temp;
	}
	let mut l = result.to_string().len();
	if l > 10 {l = 10};
	let r = "0".repeat(10).to_string() + &(result.to_string())[0..l].to_string();
	return r[r.len()-10..r.len()].to_string();

}

fn main() {
	
	let args: Vec<String> = env::args().collect();

	if args[1].len() > 5 {
	
		let test: i32= args[3].parse().unwrap();

		println!("Connecting to ip:port = {}", args[1] );
		
		if (args[2]=="-n")&&(test > 0) {
			for _i in 0..test {
				client(&args[1]);

			}
		} else {
			println!("Не указано количество клиентов");
		}

	} else {
		server(&args[1]);
	};
	
}