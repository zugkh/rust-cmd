use clap::{builder::Command, Arg};
fn main() {
    let matches = Command::new("echor")
                    .arg(Arg::new("text")
                        .value_name("TEXT")
                        .help("input text")
                        .required(true))
                    .get_matches();
    let mut text = matches.get_raw("text").expect("msg").into_iter();
    if let Some(con) = text.next() {
        print!("{:?}", con);
    }
    
}