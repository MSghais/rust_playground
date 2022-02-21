use clap::Parser;
use models::{args::{Args,get_input}, user::user_functions::User};
use std::collections::HashMap;
use std::path::Path;

mod modules;
mod models;
use modules::check;


fn main() {
    // Commands args
    let mut args = Args::parse();

    // Read the json file
    let current_dir = std::env::current_dir();
    println!(
        "json_file_path path {:?}!",
        std::fs::read_dir(current_dir.unwrap().to_owned())
    );

    let json_file_path = Path::new("./src/data/list.json");
    // Convert the JSON into Hashmap<String,User>
    println!("json_file_path path {:?}!", json_file_path);
    let data = std::fs::read_to_string(json_file_path).expect("Unable to read file");
    // Untyped JSON ?
    // let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");

    // Typed JSON
    let users: HashMap<String, User> = serde_json::from_str(&data).expect("Unable to parse");
    let mut string_of_answer: String = String::new();

    // Run or exit the program
    let mut want_to_continue: String = "yes".to_string();
    while want_to_continue == "yes" || want_to_continue == "y" {
        // UX performance about the design ?
        let time = std::time::Instant::now();

        let (mut user, mut user_exist): (Option<User>, bool) =
            check::this_args_id_exist(users.clone(), args.clone());
        // Check if the id exist
        while !user_exist {
            let msg_change_select_id: String =
                "The user doesn't exist, please select another id ".to_string();

            let new_arg_id = get_input(msg_change_select_id.to_string());
            args.id = new_arg_id.to_string();

            let (user_ok, is_user_exist) = check::this_args_id_exist(users.clone(), args.clone());

            if is_user_exist {
                user_exist = true;
                user = user_ok
            }
        }

        // Duration of the search algo
        let time_search_commands = std::time::Instant::now();

        // If user exist check the corresponding commands
        if let Some(us) = user {
            println!("us is : {:?}", us);
            string_of_answer = check::run_search_commands(users.clone(), us.clone(), args.clone());
        }

        println!("duration of UXis : {:?}", std::time::Instant::now() - time);
        println!(
            "duration is search basic algo : {:?}",
            std::time::Instant::now() - time_search_commands
        );
        // Display the length of the data wanted and display the names
        println!("{:?}", string_of_answer.to_string());

        // Input for answer continue or exit
        let question_for_continue: String =
            "Do you want to continue ? Write yes or y, otherwise everything else :p ".to_string();
        want_to_continue = get_input(question_for_continue.to_string());

        if want_to_continue == "yes" || want_to_continue == "y" {
            //  For the next iter : Change --id and --attribute ?
            let question_for_change_id: String = "Select another id for an user ?".to_string();
            args.id = get_input(question_for_change_id.to_string());

            let question_change_attribute: String = "Change the attribute to search with :
            'childrens' | or 'parents' the id for iteration ?"
                .to_string();
            args.attribute = get_input(question_change_attribute.to_string());
        }
    }
}


