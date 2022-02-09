/*use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    println!("Hello, world!");
    let args = Cli::parse();
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

use std::error::Error;
use std::io;
use std::process;

use serde::Deserialize;
use serde::Serialize;
use std::fs::OpenOptions;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    file_path: String,
    input: Vec<String>,
}

/*fn getCLI() -> Cli {
    println!("Hello, world!");
    let args = Cli::parse();

    // println!("{:?}", args.input);
    // println!("{:?}", args.file_path);
    return args //args.to_input;
}*/

// By default, struct field names are deserialized based on the position of
// a corresponding field in the CSV data's header record.
#[derive(Debug, Deserialize, Serialize)]
struct City {
    name: String,
    region: String,
    population: Option<u64>,
}

fn parseCSV() -> Result<Vec<City>, Box<dyn Error>> {
    let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    // .append(true)
    .read(true)
    .open("src/cities.csv")
    .unwrap(); 

    let mut table = Vec::new(); 

    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: City = result?; //?;
        println!("{:?}", &record);
        table.push(record); 
        // let record: Vec<String> = result?; 
    }

    return Ok(table);
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

fn writeCSV(cli: Cli) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(cli.file_path)
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
    
    wtr.write_record(&cli.input)?;

    wtr.flush()?;
    Ok(())
}

fn search(cli: Cli) -> Result<(), Box<dyn Error>> {
    println!("{:?}", &cli.file_path); 

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .read(true)
        .open(cli.file_path)
        .unwrap();


    let mut rdr = csv::Reader::from_reader(file);
    
    for result in rdr.deserialize() {
        let record: Vec<String> = result?; 
        // println!("{:?} {:?}", record, &cli.input[0]);
        for element in &cli.input {
            if record.contains(element) {
                println!("{:?}", record);
            }
            break; 
        }
    }

    Ok(())
}

// static mut global_something : Cli; 

// follow this tutorial (but write in Rust instead of C): https://cstack.github.io/db_tutorial/parts/part1.html

/* fn hash(to_hash: String) {
    to_hash.
} */

fn main() {
    // let global_cli = getCLI();
    // search(global_cli); 
    // writeCSV(global_cli);
    let table = parseCSV().unwrap(); 
    println!("{:?}", table);
    // let global_cli = getCLI();
    // removeCSV(to_remove); 
    // let to_insert = getCLI();
    // println!("{:?}", to_insert); 
    // writeCSV(to_insert);
}
