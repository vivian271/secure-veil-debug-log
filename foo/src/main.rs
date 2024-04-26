// Import necessary standard library modules and external crates.
use std::error::Error; 
use std::io::BufReader; 
use std::fs::File; 
use veil::Redact; // Import the `Redact` trait from the `veil` crate.
use log::{debug, LevelFilter}; // Import `debug` and `LevelFilter` from the `log` crate.
use rand_distr::Distribution; // Import `Distribution` trait from the `rand_distr` crate.
use statrs::distribution::Laplace; // Import `Laplace` distribution from the `statrs` crate.
use csv; // Import the `csv` crate.


// Define a struct `Patient` and derive the `Redact` trait to specify redaction rules for its fields.
#[derive(Redact)]
pub struct Patient {
    id: u32, // Patient ID

    #[redact(fixed = 2)] // Redact age field with fixed length of 2 characters.
    age: u32, 

    #[redact(fixed = 1)] // Redact gender field with fixed length of 1 character.
    gender: u32,

    #[redact(fixed = 2)] // Redact height field with fixed length of 2 characters.
    height: u32,

    #[redact(fixed = 3)] // Redact weight field with fixed length of 3 characters.
    weight: f32,

    #[redact(fixed = 3)] // Redact ap_hi field with fixed length of 3 characters.
    ap_hi: u32,

    #[redact(fixed = 2)] // Redact ap_lo field with fixed length of 2 characters.
    ap_lo: u32,

    #[redact(fixed = 1)] // Redact cholestrol field with fixed length of 1 character.
    cholestrol: u32,

    #[redact(fixed = 1)] // Redact gluc field with fixed length of 1 character.
    gluc: u32,

    #[redact(fixed = 1)] // Redact smoke field with fixed length of 1 character.
    smoke: u32,

    #[redact(fixed = 1)] // Redact alco field with fixed length of 1 character.
    alco: u32,

    #[redact(fixed = 1)] // Redact active field with fixed length of 1 character.
    active: u32,

    #[redact(fixed = 1)] // Redact cardio field with fixed length of 1 character.
    cardio: u32,
}


impl Patient {
    // Function to add Laplacian noise for differential privacy to age, height, and weight fields
    fn add_dp_noise(
        &mut self,
        sensitivity_age: f64,
        epsilon_age: f64,
        sensitivity_height: f64,
        epsilon_height: f64,
        sensitivity_weight: f64,
        epsilon_weight: f64,
    ) {
        // Create Laplace distributions for age, height, and weight with specified sensitivity and epsilon
        let laplace_age = Laplace::new(0.0, sensitivity_age / epsilon_age).unwrap();
        let laplace_height = Laplace::new(0.0, sensitivity_height / epsilon_height).unwrap();
        let laplace_weight = Laplace::new(0.0, sensitivity_weight / epsilon_weight).unwrap();

        // Generate Laplacian noise for age, height, and weight
        let noise_age = laplace_age.sample(&mut rand::thread_rng());
        let noise_height = laplace_height.sample(&mut rand::thread_rng());
        let noise_weight = laplace_weight.sample(&mut rand::thread_rng());

        // Add Laplacian noise to age, height, and weight fields while ensuring non-negative values
        self.age = ((self.age as f64) + noise_age).round().max(0.0) as u32;
        self.height = ((self.height as f64) + noise_height).round().max(0.0) as u32;
        self.weight = ((self.weight as f64) + noise_weight).round().max(0.0) as f32;
    }
}


fn main() -> Result<(), Box<dyn Error>> { // Main function returning a Result, can return errors

    // Initialize the logger with the desired logging level
    env_logger::builder().filter_level(LevelFilter::Debug).init();

    // Read the CSV file
    let file = File::open("/Users/riyaz/Desktop/Staging/eps/project/foo/src/data.csv")?; // Open CSV file
    let reader = BufReader::new(file); // Create buffered reader
    let mut csv_reader = csv::Reader::from_reader(reader); // Create CSV reader

    // Get the only row from the CSV file
    let mut row: Vec<String> = Vec::new(); // Create vector to store CSV row
    for result in csv_reader.records() { // Iterate over CSV records
        let record = result?; // Unwrap CSV record
        row = record.into_iter().map(|field| field.to_string()).collect(); // Map CSV record to vector of strings
    }

    if row.is_empty() { // Check if CSV row is empty
        return Err("CSV file is empty".into()); // Return error if CSV row is empty
    }

    // Parse CSV row into Patient struct fields
    let id: u32 = row[0].parse()?; // Parse ID
    let age: u32 = row[1].parse()?; // Parse age
    let gender: u32 = row[2].parse()?; // Parse gender
    let height: u32 = row[3].parse()?; // Parse height
    let weight: f32 = row[4].parse()?; // Parse weight
    let ap_hi: u32 = row[5].parse()?; // Parse ap_hi
    let ap_lo: u32 = row[6].parse()?; // Parse ap_lo
    let cholestrol: u32 = row[7].parse()?; // Parse cholesterol
    let gluc: u32 = row[8].parse()?; // Parse gluc
    let smoke: u32 = row[9].parse()?; // Parse smoke
    let alco: u32 = row[10].parse()?; // Parse alcohol
    let active: u32 = row[11].parse()?; // Parse active
    let cardio: u32 = row[12].parse()?; // Parse cardio

    let mut patient = Patient { // Create Patient instance
        id,
        age,
        gender,
        height,
        weight,
        ap_hi,
        ap_lo,
        cholestrol,
        gluc,
        smoke,
        alco,
        active,
        cardio
    };

    // Before adding noise
    println!("Age before adding noise: {}", patient.age); // Print age before adding noise
    println!("Height before adding noise: {}", patient.height); // Print height before adding noise
    println!("Weight before adding noise: {}", patient.weight); // Print weight before adding noise

    // Apply differential privacy to the age, height, and weight
    patient.add_dp_noise(8500.0, 0.1, 35.0, 0.2, 25.0, 0.2); // Add Laplacian noise

    // Print the age after adding noise
    println!("Age after adding noise: {}", patient.age); // Print age after adding noise
    println!("Height after adding noise: {}", patient.height); // Print height after adding noise
    println!("Weight after adding noise: {}", patient.weight); // Print weight after adding noise

    // Log the Patient struct using debug! macro
    debug!("Patient details: {:#?}", patient); // Log Patient struct details

    Ok(()) // Return Ok if everything executed successfully
}