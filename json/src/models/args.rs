use clap::Parser;
use std::io;

/// Simple commands program to search the parent or children of a specif User.id
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    // Select to search between "childrens" and "parents" depends the id selected
    #[clap(short, long)]
    pub attribute: String,

    // Id select for a specific familly research depends on --attribute
    #[clap(short, long)]
    pub id: String,

    // /// yes if you want to get the len of the "childrens" or "parents"
    #[clap(short, long)]
    pub len: String,

    // /// yes if you want to get the name
    #[clap(short, long)]
    pub name: String,

}

pub fn get_input(message_input: String) -> String {
    println!("{:?}", message_input);
    let mut input: String = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => {}
    }

    input.trim().to_string()
}