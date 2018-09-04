use std::fs::File;
use std::io::Read;
use std::io;

fn main() {
    let mut file_name = "N2.txt";
    let mut file = File::open(file_name).expect("Ошибка открытия файла");
    let mut file_data_string = String::new();
//    file.read_line(&mut file_data_string).expect("Ошибка считывания первой строки")
    file.read_to_string(&mut file_data_string).expect("Ошибка считывания файла");

    let mut split_file = file_data_string.split("\n").collect::<Vec<&str>>();
    let molecule_name_string = String::from(split_file[0]);
    let mut split_line = molecule_name_string.split("\t").collect::<Vec<&str>>();
    let molecule_name = split_line[0];
    println!("{}",split_file.len());
    println!("{}", number_of_points);



//    println!("{}", file_data_string);
//    println!("Hello, world!");
}
