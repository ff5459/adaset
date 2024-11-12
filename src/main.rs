use std::io::{self, Result, Write};
use std::process::Command;
use std::fs::File;
use core::str;

fn main() {
    let adapters: Vec<String> = get_adapters();
    if adapters.is_empty() {
        eprintln!();
        return;
    }

    println!("Выберете сетевой адаптер: ");
    print_adapters(&adapters);

    let adapter_index: usize = match prompt("").trim().parse() {
        Ok(index) if index < adapters.len() => index,
        _ => {
            eprintln!("Ожидалось число от 0 до {}.", adapters.len() - 1);
            return;
        }
    };

    let adapter: &str = adapters[adapter_index].trim();

    let file_name: String = prompt("Введите название файла: ").trim().to_string();
    let ip: String = prompt("Введите IP-адресс: ").trim().to_string();
    let mask: String = prompt("Введите маску подсети: ").trim().to_string();
    let gateway: String = prompt("Введите основной шлюз: ").trim().to_string();
    let dns: String = prompt("Введите DNS-сервер: ").trim().to_string();

    match save_preset(&file_name, &adapter, &ip, &mask, &gateway, &dns) {
        Ok(msg) => println!("{}", msg),
        Err(err) => println!("{err}")
    };
}

fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().expect("Failed to flush stdout.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка чтения.");
    input
}

fn save_preset(file_name: &str, adapter_name: &str, ip: &str, mask: &str, gateway: &str, dns: &str) -> Result<String> {
    let file_name: String = format!("{}.bat", file_name);
    let mut file: File = File::create(&file_name)?;
    let command: String = format!("netsh interface ip set address name=\"{adapter_name}\" static {ip} {mask} {gateway} && netsh interface ip set dns name=\"{adapter_name}\" static {dns}");
    file.write_all(command.as_bytes())?;

    Ok(format!("Файл успешно записан: {}", file_name))
}

fn get_adapters() -> Vec<String> {
    let output = Command::new("powershell")
        .args(&["Get-NetAdapter", "|", "Select-Object", "-Property", "Name"])
        .output()
        .expect("Ошибка исполнения команды.");

    let output: &str = str::from_utf8(&output.stdout)
        .expect("Ошибка чтения вывода команды.");

    output
        .lines()
        .skip(3)
        .map(str::to_string)
        .filter(|line| !line.trim().is_empty())
        .collect()
}

fn print_adapters(adapters: &[String]) {
    for (index, adapter) in adapters.iter().enumerate() {
        println!("({}) {}", index, adapter);
    }
}