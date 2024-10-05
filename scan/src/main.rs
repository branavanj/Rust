//Importation des traits et des structures nécessaires de la bibliothèque clap
use clap::{Arg, Command};
//Importation des adresses de socket
use std::net::SocketAddr;
//Importation pour convertir une chaîne en type SocketAddr
use std::str::FromStr;
//Importation pour les opérations réseaux asynchrones avec tokio
use tokio::net::TcpStream;
//Importation pour définir des timeouts sur les opérations asynchrones
use tokio::time::{timeout, Duration};

//Fonction asynchrone pour scanner un port sur une adresse Ip
async fn scan_port(ip: &str, port: u16) -> bool {
    let address = format!("{}:{}", ip, port);
    let socket_addr = SocketAddr::from_str(&address).unwrap();

    match timeout(Duration::from_secs(1), TcpStream::connect(socket_addr)).await {
        Ok(Ok(_)) => true,
        _ => false,
    }
}

#[tokio::main]
async fn main() {
    //Definition de la commande avec clap
    let matches = Command::new("Port Scanner")
        .arg(
            Arg::new("address")
                .required(true)//Adresse Ip obligatoire
                .index(1),//Se trouve à la 1ere position dans les arguments
        )
        .arg(
            Arg::new("port")
                .required(true)//Port obligatoire
                .index(2),//Se trouve à la 2eme position dans les arguments
        )
        .get_matches();//recupere les arguments de la ligne de commande
    
    //Stocke la valeur de l'argument "adress" dans une variable String
    let address = matches.get_one::<String>("address").unwrap();
    //Stocke la valeur de l'argument "port" dans une variable String et on la convertit en u16
    let port = matches.get_one::<String>("port").unwrap().parse::<u16>().unwrap();
    
    //Appelle la fonction scan_port() pour scanner le port sur l'adresse ip spécifiée
    let is_open = scan_port(address, port).await;
    //Affiche le résultat dans la console
    println!("Le port {} sur l'adresse {} est ouvert: {}", port, address, is_open);
}
