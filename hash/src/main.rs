/*use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Args {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    println!("Hello, world!");
    let args = Args::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    println!("{:?}", content);

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");
}*/

// Implement hash table, hashing, maybe dynamically increase size to never get the load factor above 70%, then linear search
// Imlement hashing of.. all the data? It's only by pure convention that you think that the city name is the method you should use for the key

use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process;

use serde::Deserialize;
use serde::Serialize;
use std::fs::OpenOptions;

use clap::Parser;

use ctrlc;
use std::sync::mpsc::channel;

#[derive(Parser)]
struct Args {
    /// The pattern to look for
    file_path: String,
    input: Vec<String>,
}

/*enum Option<T> {
    None,
    Some(T),
}

impl<T> Option<T> {
    fn unwrap(self) -> T {
        match self {
            Option::Some(val) => val,
            Option::None =>
              panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}*/

fn getArgs() -> Args {
    println!("Hello, world!");
    let args = Args::parse();

    // println!("{:?}", args.input);
    // println!("{:?}", args.file_path);
    return args; //args.to_input;
}

// By default, struct field names are deserialized based on the position of
// a corresponding field in the CSV data's header record.
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
struct City {
    name: String,
    region: String,
    population: u32,
}

// inspiration: https://github.com/tsoding/rust-hash-table/blob/main/src/main.rs

fn parseCSV(mut file: &File) -> Result<Vec<City>, Box<dyn Error>> {
    /*let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    // .append(true)
    .read(true)
    .open("src/cities.csv")
    .unwrap();*/

    // ::<_, _>

    // default defaults to the types' default values! (fascinating!)

    let mut table = vec![City::default(); 100]; // Vec::new(); // vec![City; 100];
                                                // Vec::with_capacity(100); //new();

    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: City = result?; //?;
                                    // println!("{:?}", &record);

        let hash = getHash(&record.name);
        println!("hash: {:?}", hash);

        table[hash] = record; 

        // table.push(record);
        // let record: Vec<String> = result?;
    }

    /*for i in 0..150 {
        let uppsala = City {
            name: "Uppsala".to_string(),
            region: "Uppland".to_string(),
            population: 9686,
        };
        table.push(uppsala);
    }*/

    return Ok(table);
}

fn printTable(table: &Vec<City>) {
    println!("----------------------------------------------------");
    for record in table {
        println!("{:?}", record);
    }
    println!("----------------------------------------------------");
}

fn removeCSV(to_remove: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        // .append(true)
        .read(true)
        .open("src/cities.csv")
        .unwrap();

    let mut write_file = OpenOptions::new()
        .write(true)
        .create(true)
        // .append(true)
        .read(true)
        .open("src/cities.csv")
        .unwrap();

    let mut rdr = csv::Reader::from_reader(file);
    let mut wtr = csv::Writer::from_writer(write_file);
    let deserialized = rdr.deserialize();

    // wtr.flush()?;

    let mut do_once = 0;
    for result in deserialized {
        if do_once < 2 {
            // Notice that we need to provide a type hint for automatic
            // deserialization.
            // let record: City = result?;
            let record: Vec<String> = result?;
            if record != to_remove {
                wtr.write_record(&record)?;
            }
            wtr.flush()?;
            println!("{:?}", record);
            do_once += 1;
        }
    }

    Ok(())
}

fn writeCSV(args: Args) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(args.file_path)
        .unwrap();

    // let mut wtr = csv::Writer::from_writer(io::stdout());
    let mut wtr = csv::Writer::from_writer(file);
    // wtr = wtr.has_headers(true);

    // When writing records with Serde using structs, the header row is written
    // automatically.
    /* wtr.serialize(City {
        name: "Uppsala".to_string(),
        region: "Uppland".to_string(),
        population: Some(9686),
    })?;  */
    /* wtr.serialize(City {
        name: "Uppsala".to_string(),
        region: "Uppland".to_string(),
        population: Some(9686),
    })?;  */
    // wtr.write_record(&["Uppsala", "Uppland", "177074"])?;

    wtr.write_record(&args.input)?;

    wtr.flush()?;
    Ok(())
}

fn addRecord(record: City, mut table: Vec<City>) -> Vec<City> {
    table.push(record);

    return table;
}

/*fn removeRecord(record: City, mut table: Vec<City>) -> Vec<City>  {
    table.pop(record);

    return table
}*/

fn search(args: Args) -> Result<(), Box<dyn Error>> {
    println!("{:?}", &args.file_path);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .read(true)
        .open(args.file_path)
        .unwrap();

    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.deserialize() {
        let record: Vec<String> = result?;
        // println!("{:?} {:?}", record, &args.input[0]);
        for element in &args.input {
            if record.contains(element) {
                println!("{:?}", record);
            }
            break;
        }
    }

    Ok(())
}

fn writeToCSV(file_path: &String, table: Vec<City>) -> Result<(), Box<dyn Error>> {
    // Overwrites it (therefore I have to redefine the file with the truncate)
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)
        .unwrap();

    let mut wtr = csv::Writer::from_writer(file);

    for record in table {
        wtr.serialize(record)?;
    }

    Ok(())
}

// static mut global_something : Args;

// follow this tutorial (but write in Rust instead of C): https://cstack.github.io/db_tutorial/parts/part1.html

/* fn hash(to_hash: String) {
    to_hash.
} */

/*fn saveOnExit() {
    let (tx, rx) = channel();

    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    println!("Waiting for Ctrl-C...");
    rx.recv().expect("Could not receive from channel.");
    println!("Got it! Exiting...");
    process::exit(0x0100);
    writeToCSV(&args.file_path, table);
}*/

fn getHash(key: &String) -> usize {
    let mut hash = 0;
    for char in key.chars() {
        hash += char as usize;
        // println!("{}", char as u32);
    }

    println!("before modulo {:?}", hash);

    return hash % 100;
}

fn main() /*-> Result<T, E>*/
{
    //saveOnExit();
    // println!("{:?}", 'a' as u32);
    // let hash = getHash("Malmö".to_string());
    // println!("hash: {:?}", hash);

    let input = io::stdin();

    let args = getArgs();

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        // .append(true)
        .open(&args.file_path)
        .unwrap();

    let mut table = parseCSV(&file).unwrap();
    printTable(&table);

    for _line in input.lock().lines().map(|_line| _line.unwrap()) {
        let command: Vec<String> = _line
            .split(' ')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        println!("{:?}", command);
        if command.contains(&"add".to_string()) {
            /*let uppsala = City {
                name: "Uppsala".to_string(),
                region: "Uppland".to_string(),
                population: 9686,
            };*/

            // let record: City = Vec::<std::string::String>::parse(); // command.parse();

            /*City {
                name: command[1],
                region: command[2],
                population: Option(command[3]? as u32)
            };*/

            let record: City = City {
                name: command[1].parse().unwrap(),
                region: command[2].parse().unwrap(),
                population: command[3].parse().unwrap(),
            };

            table = addRecord(record, table);
        } else if command.contains(&"exit".to_string()) {
            writeToCSV(&args.file_path, table);
            process::exit(0x0100);
        }

        printTable(&table);
    }
    // search(global_args);
    // writeCSV(global_args);
    // println!("{:?}", table);

    writeToCSV(&args.file_path, table);

    // let global_args = getARGS();
    // removeCSV(to_remove);
    // let to_insert = getARGS();
    // println!("{:?}", to_insert);
    // writeCSV(to_insert);
}
