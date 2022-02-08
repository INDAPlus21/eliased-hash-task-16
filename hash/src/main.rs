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
    to_input: Vec<String>,
}

fn insertionCLI() -> Vec<String> {
    println!("Hello, world!");
    let args = Cli::parse();

    println!("{:?}", args.to_input);
    return args.to_input;
}

// By default, struct field names are deserialized based on the position of
// a corresponding field in the CSV data's header record.
#[derive(Debug, Deserialize, Serialize)]
struct City {
    name: String,
    region: String,
    population: Option<u64>,
}

fn parseCSV() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: City = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn writeCSV(to_insert: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("src/cities.csv")
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
    wtr.write_record(&to_insert)?;

    // wtr.flush()?;
    Ok(())
}

fn main() {
    // writeCSV();
    // parseCSV();
    let to_insert = insertionCLI();
    println!("{:?}", to_insert); 
    writeCSV(to_insert);
}
