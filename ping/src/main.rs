//Importation des traits et des structures nécessaires de la bibliothèque clap
use clap::{Arg, Command};
//Importation des adresses ip afin de travailler avec
use std::net::IpAddr;
//Importation pour convertir une chaîne en type IpAddr
use std::str::FromStr;
//Importation pour utiliser des durées
use tokio::time::Duration;
//Importation du pinger ICMP de tokio
use tokio_icmp_echo::Pinger;

//Fonction asynchrone vérifiant si l'hôte est actif
async fn host_is_up(address: &str) -> bool {
    let ip_addr = IpAddr::from_str(address).unwrap();
    let pinger = Pinger::new().await.unwrap();
    let timeout = Duration::from_secs(1);
    let identifier = 0; // Identifiant pour le ping (peut être n'importe quel entier 16 bits)
    let sequence = 0; // Séquence pour le ping (peut être n'importe quel entier 16 bits)

    let result = pinger.ping(ip_addr, identifier, sequence, timeout).await;

    result.is_ok()
}


#[tokio::main]
async fn main() {
    //Definition de la commande avec clap
    let matches = Command::new("host_checker")
        .arg(
            Arg::new("address")
                .required(true)
                .index(1),
        )
        .get_matches();
    
    //Stocke la variable adress dans un type String 
    let address = matches.get_one::<String>("address").unwrap();
    //Appelle de la fonction host_is_up pour vérifier si l'hôte est actif ou non
    let is_up = host_is_up(address).await;
    //Affichage du résultat dans la console
    println!("L'hôte {} est actif: {}", address, is_up);
}
