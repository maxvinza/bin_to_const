use std::{
    env,
    fs::File,
    io::Read,
};


fn main() {
    let arguments_collection: Vec<String> =  env::args().collect();
    if arguments_collection.len() == 2 {
        if &arguments_collection[1] == "-help" {
            println!("\nUsage ./bin_to_const [FILENAME] > out.txt\n");
        } else {
            let mut file = File::open(&arguments_collection[1]).unwrap();
            let mut buffer = Vec::new();
            println!("\nconst DATA: &[u8] = &[");
            let mut counter = 0;
            let mut printed_string = String::new();
            file.read_to_end(&mut buffer).unwrap();
            for byte in &buffer {
                if counter > 0 {
                    printed_string.push_str(", 0x");
                } else {
                    printed_string.push_str("    0x");
                }

                let text_byte = format!("{:x}", byte);

                if text_byte.len() == 1 {
                    printed_string.push('0');
                }

                printed_string.push_str(&text_byte);
                counter += 1;

                if counter > 10 {
                    println!("{},", printed_string);
                    counter = 0;
                    printed_string = String::new();
                }
            }

            if printed_string.len() > 0 {
                println!("{},", printed_string);
            }

            println!("];\n");
        }
    } else {
        println!("\nWrong num input arguments, please use -help to see help info\n");
    }
}
