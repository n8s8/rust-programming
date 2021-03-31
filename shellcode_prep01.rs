use std::process::Command;
use std::io::{ stdin, stdout, Write };

fn main() {
    loop {
        let mut input = String::new();
        print!("> ");
        let _ = stdout().flush();
        stdin().read_line(&mut input).expect("Did not enter a correct string");
        println!("{}", input);

        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                    .args(&["/C", &input])
                    .output()
                    .expect("failed to execute process")
        } else {
            Command::new("sh")
                    .arg("-c")
                    .arg(&input)
                    .output()
                    .expect("failed to execute process")
        };
        
        let output = output.stdout;
        println!("{:?}", String::from_utf8_lossy(&output[..]));
    }
}
