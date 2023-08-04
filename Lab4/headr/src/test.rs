// pub fn run(config: Config) -> MyResult<()> {
//     let num_files = config.files.len();

//     for (file_num, filename) in config.files.iter().enumerate() {
//         match open(filename) {
//             Err(err) => eprintln!("{}: {}", filename, err),
//             Ok(mut file) => {
//                 if num_files > 1 {
//                     println!(
//                         "{}==> {} <==",
//                         if file_num > 0 { "\n" } else { "" },
//                         &filename
//                     );
//                 }

//                 if let Some(num_bytes) = config.bytes {
//                     let mut handle = file.take(num_bytes as u64);
//                     let mut buffer = vec![0; num_bytes];
//                     let bytes_read = handle.read(&mut buffer)?;
//                     print!(
//                         "{}",
//                         String::from_utf8_lossy(&buffer[..bytes_read])
//                     );
//                 } else {
//                     let mut line = String::new();
//                     for _ in 0..config.lines {
//                         let bytes = file.read_line(&mut line)?;
//                         if bytes == 0 {
//                             break;
//                         }
//                         print!("{}", line);
//                         line.clear();
//                     }
//                 }
//             }
//         }
//     }
//     Ok(())
// }

// fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
//     match filename {
//         "-" => Ok(Box::new(BufReader::new(io::stdin()))),
//         _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
//     }
// }
