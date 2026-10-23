use std::net::{TcpStream, SocketAddr, IpAddr, Ipv4Addr};
use std::time::Duration;

fn main() {
    let mut iterador : u16 = 1;
    let mut porta = String::new();
    let tempo = Duration::from_millis(100);
    let mut ip : SocketAddr;

    loop{
        porta.clear();

        porta = iterador.to_string();

        ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 200, 6, 219)), iterador);

        match TcpStream::connect_timeout(&ip, tempo){
            Ok(_) => {
                println!("Porta {} aberta", porta);
            }
            Err(_) => {
                
            }
        }

        iterador += 1;

        if iterador == 65_000{
            break;
        }
    }
}