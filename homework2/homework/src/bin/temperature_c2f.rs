use clap::{Arg, App};

fn main(){

    let matches = App::new("temperature_c2f")
                    .version("0.1.0")
                    .author("Thura Aung")
                    .about("convert the temperature from C to F")
                    .arg(
                        Arg::with_name("cel")
                            .value_name("cel")
                            .help("Enter Celsius for temperature conversion")
                            .required(true)
                            .index(1)
                    ).get_matches();
    
    let cel_arg = matches.value_of("cel").unwrap().to_string();
    let cel = cel_arg.parse().unwrap_or(0.0);

    let fah = ((9.0 * cel)/5.0) + 32.0;

    println!("Temperature at {cel} Celsius is {fah} Fahrenheit");
}
