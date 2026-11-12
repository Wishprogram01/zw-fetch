use sysinfo::System;
use colored::*;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

// Fungsi untuk mendapatkan laluan fail ~/.zw_todo
fn get_todo_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".zw_todo")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let todo_path = get_todo_path();

    // --- LOGIK INTERAKTIF: TAMBAH / PADAM TUGASAN ---
    if args.len() > 1 {
        match args[1].as_str() {
            "add" => {
                if args.len() > 2 {
                    let task = args[2..].join(" ");
                    let mut file = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(&todo_path)
                        .expect("Gagal membuka fail todo");
                    
                    writeln!(file, "{}", task).expect("Gagal menulis tugasan");
                    println!("{} Tugasan ditambah: {}", "✓".green().bold(), task.bright_white());
                } else {
                    println!("{} Sila masukkan teks tugasan. Contoh: zw-fetch add \"Kemaskini database\"", "!".yellow().bold());
                }
                return; // Tamatkan program selepas tambah tugasan
            },
            "clear" => {
                let _ = fs::write(&todo_path, "");
                println!("{} Semua tugasan telah dikosongkan!", "✓".green().bold());
                return;
            },
            _ => {}
        }
    }

    // --- PAPARAN UTAMA ZW-FETCH ---
    let mut sys = System::new_all();
    sys.refresh_all();

    let username = whoami::username();
    let hostname = whoami::hostname();
    
    let total_mem = sys.total_memory() / 1024 / 1024;
    let used_mem = sys.used_memory() / 1024 / 1024;
    let os_name = System::name().unwrap_or_else(|| "Linux".to_string());

    let raw_logo = [
        "{R}            .-/+oossssoo+/-.            ",
        "{R}        `:+ssssssssssssssssss+:`        ",
        "{R}      -+ssssssssssssssssssyyssss+-      ",
        "{R}    .ossssssssssssssssss{W}dMMMNy{R}sssso.    ",
        "{R}   /sssssssssss{W}hdmmNNmmyNMMMMh{R}ssssss/   ",
        "{R}  +sssssssss{W}hm{R}yd{W}MMMMMMMNddddy{R}ssssssss+  ",
        "{R} /ssssssss{W}hNMMMyhhyyyyhmNMMMNh{R}ssssssss/ ",
        "{R} .ssssssss{W}dMMMNh{R}ssssssssss{W}hNMMMd{R}ssssssss.",
        "{R}  +ssss{W}hhhyNMMNy{R}ssssssssssss{W}yNMMMy{R}sssssss+",
        "{R}  oss{W}yNMMMNyMMh{R}ssssssssssssss{W}hmmmh{R}ssssssso",
        "{R}  oss{W}yNMMMNyMMh{R}sssssssssssssshmmmmhssssssso",
        "{R}  +ssss{W}hhhyNMMNy{R}ssssssssssss{W}yNMMMy{R}sssssss+",
        "{R} .ssssssss{W}dMMMNh{R}ssssssssss{W}hNMMMd{R}ssssssss.",
        "{R} /ssssssss{W}hNMMMyhhyyyyhdNMMMNh{R}ssssssss/ ",
        "{R}  +sssssssss{W}dm{R}yd{W}MMMMMMMMddddy{R}ssssssss+  ",
        "{R}   /sssssssssss{W}hdmNNNNmyNMMMMh{R}ssssss/   ",
        "{R}    .ossssssssssssssssss{W}dMMMNy{R}sssso.    ",
        "{R}      -+sssssssssssssssss{W}yyy{R}ssss+-      ",
        "{R}        `:+ssssssssssssssssss+:`        ",
        "{R}            .-/+oossssoo+/-.            ",
    ];

    let title_art = [
        r"▒███████▒ █     █░        █████▒▓█████▄▄▄█████▓ ▄████▄   ██░ ██ ",
        r"▒ ▒ ▒ ▄▀░▓█░ █ ░█░      ▓██   ▒ ▓█   ▀▓  ██▒ ▓▒▒██▀ ▀█  ▓██░ ██▒",
        r"░ ▒ ▄▀▒░ ▒█░ █ ░█  █████▒████ ░ ▒███  ▒ ▓██░ ▒░▒▓█    ▄ ▒██▀▀██░",
        r"  ▄▀▒   ░░█░ █ ░█  ░░░░░░▓█▒  ░ ▒▓█  ▄░ ▓██▓ ░ ▒▓▓▄ ▄██▒░▓█ ░██ ",
        r"▒███████▒░░██▒██▓       ░▒█░    ░▒████▒ ▒██▒ ░ ▒ ▓███▀ ░░▓█▒░██▓",
        r"░▒▒ ▓░▒░▒░ ▓░▒ ▒         ▒ ░    ░░ ▒░ ░ ▒ ░░   ░ ░▒ ▒  ░ ▒ ░░▒░▒",
        r"░░▒ ▒ ░ ▒  ▒ ░ ░         ░       ░ ░  ░   ░      ░  ▒    ▒ ░▒░ ░",
        r"░ ░ ░ ░ ░  ░   ░         ░ ░       ░    ░      ░         ░  ░░ ░",
        r"  ░ ░      ░                       ░  ░        ░ ░       ░  ░  ░",
        r"░                                              ░                ",
    ];

    let mut info: Vec<String> = Vec::new();
    
    for line in title_art.iter() {
        info.push(line.bright_white().bold().to_string());
    }

    info.push("".to_string());

    // Info Sistem
    info.push(format!("{}@{}", username.cyan().bold(), hostname.magenta().bold()));
    info.push("-".repeat(65).truecolor(100, 100, 100).to_string());
    info.push(format!("{}: {}", "OS".blue().bold(), os_name));
    info.push(format!("{}: {} MB / {} MB", "RAM".blue().bold(), used_mem, total_mem));
    info.push(format!("{}: {}", "Shell".blue().bold(), "bash/zsh"));
    
    info.push("".to_string()); // Pemisah sebelum bahagian To-Do

    // --- BACA & PAPARKAN QUICK NOTE / TO-DO ---
    info.push(format!("{}", "📌 QUICK NOTES / TO-DO:".yellow().bold()));
    
    if let Ok(contents) = fs::read_to_string(&todo_path) {
        let tasks: Vec<&str> = contents.lines().filter(|l| !l.trim().is_empty()).collect();
        if tasks.is_empty() {
            info.push(format!("   {}", "Tiada tugasan aktif. (Guna: zw-fetch add \"tugasan\")".truecolor(120, 120, 120)));
        } else {
            // Paparkan maksimum 4 tugasan terkini supaya paparan kekal kemas
            for (index, task) in tasks.iter().take(4).enumerate() {
                info.push(format!("   {}. {}", (index + 1).to_string().cyan(), task.bright_white()));
            }
        }
    } else {
        info.push(format!("   {}", "Tiada nota lagi. (Guna: zw-fetch add \"tugasan\")".truecolor(120, 120, 120)));
    }

    println!(); 

    let max_lines = std::cmp::max(raw_logo.len(), info.len());
    
    let red_ansi = "\x1b[31m";
    let white_ansi = "\x1b[97m";
    let reset_ansi = "\x1b[0m";
    
    for i in 0..max_lines {
        let left = if i < raw_logo.len() { 
            raw_logo[i].replace("{R}", red_ansi).replace("{W}", white_ansi) + reset_ansi
        } else { 
            " ".repeat(40) 
        };
        
        let right = if i < info.len() { &info[i] } else { &String::new() };
        
        println!("  {}    {}", left, right);
    }
    
    println!();
}
