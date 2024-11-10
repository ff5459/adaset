use core::str;
use std::net::IpAddr;
use std::process::Command;
use std::io::{self, Result, Write};
use std::fs::File;

fn main() {
    let adapters: Vec<String>           = get_adapters();
    let mut adapter_index: String       = String::new();
    let mut file_name: String           = String::new();
    let mut ip: String                  = String::new();
    let mut mask: String                = String::new();
    let mut gateway: String             = String::new();
    let mut dns: String                 = String::new();

    println!("Выберете сетевой адаптер: ");
    print_adapters(&adapters);

    io::stdin().read_line(&mut adapter_index).expect("Ошибка чтения адаптера.");
    let adapter_index: usize = match adapter_index.trim().parse() {
        Ok(adapter) => adapter,
        Err(_) => {
            println!("Ожидалось число.");
            return;
        }
    };
    let adapter: &String = match adapters.get(adapter_index) {
        Some(adapter) => adapter,
        None => {
            println!("Ожидалось число от 0 до {}.", adapters.len() - 1);
            return;
        }
    };

    println!("Введите название: ");
    io::stdin().read_line(&mut file_name).expect("Ошибка чтения названия.");
    let file_name: &str = file_name.trim();

    println!("Введите IP-адресс: ");
    io::stdin().read_line(&mut ip).expect("Ошибка чтения IP.");
    let ip: &str = ip.trim();

    println!("Введите маску подсети: ");
    io::stdin().read_line(&mut mask).expect("Ошибка чтения маски.");
    let mask: &str = mask.trim();

    println!("Введите основной шлюз: ");
    io::stdin().read_line(&mut gateway).expect("Ошибка чтения шлюза.");
    let gateway: &str = gateway.trim();

    println!("Введите DNS-сервер: ");
    io::stdin().read_line(&mut dns).expect("Ошибка чтения DNS.");
    let dns: &str = dns.trim();

    match save_preset(&file_name, &adapter, &ip, &mask, &gateway, &dns) {
        Ok(res) => {
            println!("{res}");
        },
        Err(err) => {
            println!("{err}");
        }
    };
}

fn save_preset(file_name: &str, adapter_name: &str, ip: &str, mask: &str, gateway: &str, dns: &str) -> Result<String> {
    let mut file:File = File::create(format!("{file_name}.bat"))?;
    let command: String = format!("netsh interface ip set address name=\"{adapter_name}\" static {ip} {mask} {gateway} && netsh interface ip set dns name=\"{adapter_name}\" static {dns}");
    file.write_all(command.as_bytes())?;

    Ok(String::from("Файл успешно записан."))
}

fn get_adapters() -> Vec<String> {
    let output = Command::new("ipconfig")
        .output()
        .expect("Ошибка исполнения команды.");

    let output: &str = str::from_utf8(&output.stdout)
        .expect("Ошибка чтения вывода команды.");

    let mut output = output.lines();

    let mut adapters: Vec<String> = Vec::new();
    while let Some(line) = output.next() {
        if line.starts_with("Ethernet adapter") {
            adapters.push(line[17..line.len() - 1].to_string());
        }
    }

    adapters
}

fn print_adapters(adapters: &Vec<String>) {
    for (index, adapter) in adapters.iter().enumerate() {
        println!("({index}) {adapter}");
    }
}