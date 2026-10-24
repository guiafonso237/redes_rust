use std::net::{TcpStream, SocketAddr, IpAddr};
use std::time::Duration;
use std::io::{self, Write};
use clap::Parser;

fn main() {
    let args = Opcoes::parse();

    exec(args);

    //descobrir_portas();
}

fn exec(args : Opcoes){
    if args.mostrar_todos{
        descobrir_portas();
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

fn descobrir_ip_geral() -> String{
    let ip_inicial = descobrir_ip().to_string();

    match ip_inicial.rfind('.') {
        Some(pos) => ip_inicial[..pos].to_string(),
        None => ip_inicial,
    }
    
}

fn transformar_string_to_ip(ip_string : &str) -> IpAddr {
    if ip_string == "0.0.0.0"{
        panic!("Erro: ip da máquina não foi obtido");
    }

    let ipv4 = ip_string.parse().expect("Não foi possível obter o ipv4");

    let ip = IpAddr::V4(ipv4);

    ip
}

fn descobrir_portas(){
    let mut iterador : u16 = 1;
    let mut porta = String::new();
    let tempo = Duration::from_millis(100);
    let mut ip : SocketAddr;
    let mut faixa_ip : String;

    loop{
        
        faixa_ip = format!("{}{}{}", descobrir_ip_geral(), ".", iterador.to_string());
        
        println!("Verificando o IP: {}", faixa_ip);

        io::stdout().flush().unwrap();


        for i in 1..100 {
            ip = SocketAddr::new(transformar_string_to_ip(&faixa_ip), i);

            porta.clear();
            
            porta = i.to_string();
            
            
            match TcpStream::connect_timeout(&ip, tempo){
                Ok(_) => {
                    println!("Porta {} do ip {} aberta", porta, iterador);
                }
                Err(_) => {
                    //println!("Porta {} do ip {} fechada", porta, iterador);
                }
            }
        }
        
        faixa_ip.clear();
        
        iterador += 1;

        if iterador == 255{
            break;
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Opcoes{
    #[arg(short= 'a', long)]
    mostrar_todos : bool,


}