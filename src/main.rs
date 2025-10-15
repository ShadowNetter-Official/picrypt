use std::{fs, process, io::{BufWriter, Write}, fs::File};
use image::{Pixel, GenericImageView, io::Reader as ImageReader, ImageFormat};
use prompted::input;

fn caesar(target: char, shift: u32, encrypt: bool) -> char {
    if !target.is_ascii_lowercase() { return target; }
    let ascii = target as u8 - b'a';
    let shifted = if encrypt {
        (ascii + (shift as u8)) % 26
    } else {
        (26 + ascii - (shift as u8 % 26)) % 26
    };
    (b'a' + shifted) as char
}

fn vignere(pass: &str, key: &Vec<u32>, encrypt: bool) -> String {
    let len = key.len();
    pass.chars()
        .enumerate()
        .map(|(i, c)| {
            let shift = key[i%len];
            caesar(c, shift, encrypt)
        })
        .collect()
}

fn get_key() -> Vec<u32> {
    let key = input!("Key (DO NOT FORGET THIS KEY) >> ");
    convert_key(&key)
}

fn convert_key(key: &String) -> Vec<u32> {
    if !key.chars().all(|c| c.is_ascii_alphabetic()) {
        println!("Key must contain only ASCII letters");
        process::exit(1);
    }
    key.to_lowercase().chars().map(|c| (c as u8 - b'a') as u32).collect()
}

fn encrypt_text() -> String {
    let key = get_key();
    let password = input!("text: ");
    let encrypted = vignere(&password, &key, true);
    return encrypted;
}

fn to_ppm(input: &str) -> String {
    let mut map: Vec<String> = Vec::new();
    for i in input.chars() {
        let i = i as u8;
        let formatted = format!("{:03}", i);
        map.push(formatted);
    }
    let width: usize = 1;
    let height: usize = (map.len() + width - 1) / width;
    let mut output: Vec<Vec<String>> = vec![vec![String::new(); width]; height];
    let mut index: usize = 0;
    for y in 0..height {
        for x in 0..width {
            if index < map.len() {
                output[y][x] = map[index].clone();
                index+=1;
            }
        }
    }
    let mut pixels: String = String::new();
    for i in output {
        for j in i {
            for _ in 0..3 {
                pixels = pixels + &j.to_string() + " ";
            }
        }
        pixels = pixels + "\n";
    }
    let ppm = format!("P3\n{width} {height}\n255\n{pixels}");
    return ppm;
}

fn hide(file_path: &str, png_path: &str) {
    let input = encrypt_text();
    let ppm = to_ppm(&input);
    let mut file = match File::create(file_path) {
        Ok(file) => file,
        Err(e) => {
            println!("{e}");
            process::exit(1);
        }
    };
    match file.write_all(ppm.as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            println!("{e}");
            process::exit(1);
        }
    }
    ppm_to_png(file_path, png_path);
}

fn decrypt(file: &str, output_file: &str) -> String {
    let key = get_key();
    png_to_ppm(file, output_file);
    let ppm = match fs::read_to_string(output_file) {
        Ok(ppm) => ppm,
        Err(e) => {
            println!("{e}");
            process::exit(1);
        }
    };
    return unhide(&key, &ppm);
}

fn unhide(key: &Vec<u32>, ppm: &str) -> String {
    let mut output: Vec<char> = Vec::new();
    for line in ppm.lines().skip(3) {
        for block in line.as_bytes().chunks(12) {
            let letter = match std::str::from_utf8(block) {
                Ok(letter) => letter,
                Err(e) => {
                    println!("{e}");
                    process::exit(1);
                }
            };
            let numbers: Vec<char> = vec![
                letter.chars().nth(0).unwrap(),
                letter.chars().nth(1).unwrap(),
                letter.chars().nth(2).unwrap(),
            ];
            let mut character = String::new();
            for n in numbers {
                character.push(n);
            }
            let character = character.trim();
            if !character.is_empty() {
                let u8_char: u8 = match character.parse() {
                    Ok(u8_char) => u8_char,
                    Err(e) => {
                        println!("{e}");
                        process::exit(1);
                    }
                };
                output.push(u8_char as char);
            }
        }
    }
    let mut text: String = String::new();
    for i in output { text.push(i); }
    return vignere(&text, key, false);
}

fn ppm_to_png(ppm_file: &str, new_file: &str) {
    let reader = match ImageReader::open(ppm_file) {
        Ok(reader) => reader,
        Err(e) => {
            println!("{e}");
            process::exit(1);
        }
    };
    let img = match reader.decode() {
        Ok(img) => img,
        Err(e) => {
            println!("{e}");
            process::exit(1);
        }
    };
    let file = match File::create(new_file) {
        Ok(file) => file,
        Err(e) => {
            println!("{e}");
            process::exit(1);
        }
    };
    let writer = BufWriter::new(file);
    match img.write_to(&mut std::io::BufWriter::new(writer), ImageFormat::Png) {
        Ok(_) => (),
        Err(e) => {
            println!("{e}");
            process::exit(1);
        }
    }
}

fn png_to_ppm(png: &str, ppm: &str) {
    let img = match image::open(png) {
        Ok(img) => img,
        Err(e) => {
            println!("{e}");
            process::exit(1);
        }
    };
    let (width, height) = img.dimensions();
    let file = match File::create(ppm) {
        Ok(file) => file,
        Err(e) => {
            println!("{e}");
            process::exit(1);
        }
    };
    let mut writer = BufWriter::new(file);
    if let Err(e) = writeln!(writer, "P3") {
        println!("{e}");
        process::exit(1);
    }
    if let Err(e) = writeln!(writer, "{} {}", width, height) {
        eprintln!("{e}");
        process::exit(1);
    }
    if let Err(e) = writeln!(writer, "255") {
        println!("{e}");
        process::exit(1);
    }
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y).to_rgb();
            match writeln!(writer, "{} {} {}", pixel[0], pixel[1], pixel[2]) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("{e}");
                    process::exit(1);
                }
            }
        }
    }
}

fn help() {
    println!("picrypt | a CLI tool to convert text into encrypted image and vice versa\n");
    println!("usage:\n");
    println!("picrypt encrypt | encrypts text to image");
    println!("picrypt decrypt | decrypt image to text\n");
}

fn get_cla() -> bool {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        help();
        process::exit(1);
    }
    match &args[1]  as &str {
        "decrypt" => return false,
        "encrypt" => return true,
        _ => {
            help();
            process::exit(1);
        }
    }
}

fn main() {
    let encrypt = get_cla();
    if encrypt {
        let file = input!("ppm file name: ");
        let png = input!("png file name: ");
        hide(&file, &png);
    } else {
        let file = input!("png file name: ");
        let ppm = input!("ppm file name: ");
        let output = decrypt(&file, &ppm);
        println!("{output}");
    }
}

