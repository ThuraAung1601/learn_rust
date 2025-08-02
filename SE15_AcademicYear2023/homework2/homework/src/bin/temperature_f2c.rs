use clap::{Arg, App};

fn main() {
    
    let matches = App::new("temperature_f2c")
            .version("0.1.0")
            .author("Thura Aung")
            .about("convert the temperature from F to C")
            .arg(
                Arg::with_name("fah")
                    .value_name("Fahrenheit")
                    .help("Enter Fahrenheit for temperature conversion")
                    .required(true)
                    .index(1)
            ).get_matches();
    
    let fah_arg = matches.value_of("fah").unwrap().to_string();
    let fah = fah_arg.parse().unwrap_or(0.0);
    
    let cel = (5.0/9.0)*(fah-32.0);

    println!("Temperature at {fah} Fahrenheit is {cel} Celsius");
    
    
}
