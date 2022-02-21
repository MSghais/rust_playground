// pub mod check {
use crate::models::args::Args;
use crate::models::user::user_functions::{get_all_predecessors, User};
use std::collections::HashMap;

// Check if the user with args.id exist on the JSON file
// Return a bool to check fastly and the Some(user) to used it if it's exist
pub fn this_args_id_exist(users: HashMap<String, User>, args: Args) -> (Option<User>, bool) {
    let id: String = args.id;
    let _id: String = id.clone();

    let mut user_by_id: Option<User> = None;
    for (i, user) in users.clone() {
        if i == id {
            user_by_id = Some(user);
        }
    }
    match user_by_id {
        Some(user) => {
            let us: User = user;
            (Some(us.clone()), true)
        }
        None => (None, false),
    }
}

pub fn run_search_commands(users: HashMap<String, User>, user: User, args: Args) -> String {
    // Create the string to send for a result of search "engine"
    let mut string_of_answer: String = String::new();
    // Add id on each User struct
    // Transform all news User on a vector
    let all_users: Vec<User> = users
        .clone()
        .into_iter()
        .map(|(_id, mut score)| {
            score.id = Some(_id);
            score
        })
        .collect();
    let _id: String = args.id.clone();

    // Search by arguments ( children or parents)
    // Args contains the children attribute for searching the len and names on it
    if args.attribute == "childrens" {
        let how_many_children: usize = user.children.len();
        if args.name == "yes" {
            println!(
                "len of children is : {:?} with the name of {:?}",
                how_many_children,
                user.children.clone().into_iter()
            );
            if args.len == "yes" {
                string_of_answer = format!(
                    "Length of childrens is : {:?} with the names : {:?}",
                    how_many_children,
                    user.children.clone().into_iter()
                );
            } else {
                string_of_answer = format!(
                    "Names of childrens are {:?}",
                    user.children.clone().into_iter()
                );
            }
        } else {
            println!("len of children is : {:?}", how_many_children,);
            // string_of_answer = format!("len of children is : {:?}", how_many_children);
            if args.len == "yes" {
                string_of_answer = format!(
                    "Length of childrens is : {:?} with the names : {:?}",
                    how_many_children,
                    user.children.clone().into_iter()
                );
            } else {
                string_of_answer = format!(
                    "Names of childrens are {:?}",
                    user.children.clone().into_iter()
                );
            }
        }
    } else if args.attribute == "parents" {
        let mut all_parents: Vec<String> = vec![];
        let mut all_parents_id: Vec<String> = vec![];
        let mut all_familly_users: Vec<User> = vec![];
        all_parents_id.push(_id.to_string());

        // Loop if the user selected have one direct parent and recursively check if another parent are ok
        // let last_user: &User = &users[&_id];
        // println!("user : {:?}", last_user);

        for us in &all_users {
            if us.children.contains(&_id) {
          
                if let Some(i) = &us.id {
                    // Get the first parent ID and loop recursively on this functions
                    get_all_predecessors(
                        &users,
                        &all_users,
                        &_id,
                        &mut all_familly_users,
                        &mut all_parents,
                        &mut all_parents_id,
                    );
                }
            }
        }
        let count: usize = all_parents.len();
        // println!(
        //     "len is {:?} and the name of each parens are {:?}",
        //     count, all_parents
        // );
        // string_of_answer = format!(
        //     "len is {:?} and the name of each parens are {:?}",
        //     count, all_parents
        // );
        if args.len == "yes" {
            if args.name == "yes" {
                string_of_answer = format!(
                    "len is {:?} and the name of each parens are {:?}",
                    count,
                    all_parents.clone()
                );
            } else {
                string_of_answer = format!(
                    "len is {:?} and the name of each parens are {:?}",
                    count,
                    all_parents.clone()
                );
            }
        } else {
            string_of_answer = format!("the names of each parens are {:?}", all_parents.clone());
        }
    }
    string_of_answer.to_string()
}

#[cfg(test)]
pub mod module_check_test {

    use super::*;
    #[test]
    pub fn test_check_if_user_exist() {
        let current_dir = std::env::current_dir();
        println!(
            "json_file_path path {:?}!",
            std::fs::read_dir(current_dir.unwrap().to_owned())
        );
        let json_file_path = std::path::Path::new("./src/data/list.json");
        // Convert the JSON into Hashmap<String,User>
        println!("json_file_path path {:?}!", json_file_path);
        let data = std::fs::read_to_string(json_file_path).expect("Unable to read file");
        // Typed JSON
        let users: HashMap<String, User> = serde_json::from_str(&data).expect("Unable to parse");

        let args: Args = Args {
            attribute: "parents".to_string(),
            name: "yes".to_string(),
            len: "yes".to_string(),
            id: "7".to_string(),
        };
        let (_, is_exist): (Option<User>, bool) = this_args_id_exist(users.clone(), args.clone());

        if users.contains_key(&args.id.to_string()) {
            assert_eq!(is_exist, true);
        } else {
            assert_eq!(is_exist, false);
        }

        for i in 0..100 {
            let args: Args = Args {
                attribute: "parents".to_string(),
                name: "yes".to_string(),
                len: "yes".to_string(),
                id: i.to_string(),
            };
            let (_, is_exist): (Option<User>, bool) =
                this_args_id_exist(users.clone(), args.clone());

            if users.contains_key(&args.id.to_string()) {
                assert_eq!(is_exist, true);
            } else {
                assert_eq!(is_exist, false);
            }
        }
    }

    #[test]
    pub fn test_and_get_childrens() {
        // let users =
        let current_dir = std::env::current_dir();
        println!(
            "json_file_path path {:?}!",
            std::fs::read_dir(current_dir.unwrap().to_owned())
        );
        let json_file_path = std::path::Path::new("./src/data/list.json");
        // Convert the JSON into Hashmap<String,User>
        eprint!("json_file_path path {:?}!", json_file_path);
        let data = std::fs::read_to_string(json_file_path).expect("Unable to read file");
        // Untyped JSON ?
        // let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");
        // Typed JSON
        let users: HashMap<String, User> = serde_json::from_str(&data).expect("Unable to parse");
        for i in 0..100 {
            let mut string_of_answer: String = String::new();

            let args: Args = Args {
                attribute: "parents".to_string(),
                name: "yes".to_string(),
                len: "yes".to_string(),
                id: i.to_string(),
            };
            let (user, is_exist): (Option<User>, bool) =
                this_args_id_exist(users.clone(), args.clone());

            if users.contains_key(&args.id.to_string()) {
                assert_eq!(is_exist, true);
            } else {
                assert_eq!(is_exist, false);
            }

            if let Some(us) = user {
                string_of_answer = run_search_commands(users.clone(), us.clone(), args.clone());
                println!("string_of_answer {:?}!", string_of_answer);
            }
        }
    }

    #[test]
    pub fn test_check_and_get_parents() {
        // let users =
        let current_dir = std::env::current_dir();
        println!(
            "json_file_path path {:?}!",
            std::fs::read_dir(current_dir.unwrap().to_owned())
        );
        let json_file_path = std::path::Path::new("./src/data/list.json");
        // Convert the JSON into Hashmap<String,User>
        println!("json_file_path path {:?}!", json_file_path);
        let data = std::fs::read_to_string(json_file_path).expect("Unable to read file");
        // Untyped JSON ?
        // let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");
        // Typed JSON
        let users: HashMap<String, User> = serde_json::from_str(&data).expect("Unable to parse");
        let mut string_of_answer: String = "".to_string();

        for i in 0..100 {
            let args: Args = Args {
                attribute: "parents".to_string(),
                name: "yes".to_string(),
                len: "yes".to_string(),
                id: i.to_string(),
            };
            let (user, is_exist): (Option<User>, bool) =
                this_args_id_exist(users.clone(), args.clone());
            if users.contains_key(&args.id.to_string()) {
                assert_eq!(is_exist, true);
            } else {
                assert_eq!(is_exist, false);
            }

            if let Some(us) = user {
                string_of_answer = run_search_commands(users.clone(), us.clone(), args.clone());
            }

            println!("string_of_answer {:?}!", string_of_answer);
        }
    }
}
