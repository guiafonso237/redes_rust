use std::net::{TcpStream, SocketAddr, IpAddr};
use std::time::Duration;

fn main() {
    let mut iterador : u16 = 1;
    let mut porta = String::new();
    let tempo = Duration::from_millis(100);
    let mut ip : SocketAddr;

    loop{
        porta.clear();

        porta = iterador.to_string();

        ip = SocketAddr::new(transformar_string_to_ip(descobrir_ip()), iterador);

        match TcpStream::connect_timeout(&ip, tempo){
            Ok(_) => {
                println!("Porta {} aberta", porta);
            }
            Err(_) => {
                println!("Porta {} fechada", porta);
            }
        }

        iterador += 1;

        if iterador == 100{
            break;
        }
    }
}

fn descobrir_ip() -> String {
    if let Some(ip) = local_ipaddress::get() {
        return ip;
    }else{
        let erro : String = "0.0.0.0".to_string();

        return erro;
    }
}

fn transformar_string_to_ip(ip_string : String) -> IpAddr {
    let ipv4 = ip_string.parse().expect("Não foi possível obter o ipv4");

    let ip = IpAddr::V4(ipv4);

    ip
}