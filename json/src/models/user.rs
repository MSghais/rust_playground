pub mod user_functions {
    use serde::Deserialize;
    use std::collections::HashMap;
    #[derive(Debug, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct User {
        pub id: Option<String>,
        pub name: String,
        pub category: String,
        pub children: Vec<String>,
    }
    
    // This function are a recursively call if the next parent get have another parent (check direct and indirect parent)
    pub fn get_all_predecessors(
        users_map: &HashMap<String, User>,
        all_users: &Vec<User>,
        for_id_parent: &String,
        all_familly_users: &mut Vec<User>,
        vec_parent_names: &mut Vec<String>,
        vec_parent_id: &mut Vec<String>,
    ) -> Vec<String> {
        // Loop on all users of the json file and check if one have a children of for_id_parent
        for l_user in all_users {
            // println!("l_user : {:?}", l_user);
            if let Some(i) = &l_user.id {
                // println!(
                //     "l_user.index = : {:?}",
                //     i // l_user.children.contains(for_id_parent)
                // );
                if l_user.children.contains(for_id_parent) {
                    // println!(
                    //     "l_user.children.contains({:?})",
                    //     for_id_parent // l_user.children.contains(for_id_parent)
                    // );
                    all_familly_users.push(l_user.clone());
                    vec_parent_names.push(l_user.name.clone());
                    vec_parent_id.push(l_user.name.clone());
                    get_all_predecessors(
                        users_map,
                        all_users,
                        i,
                        all_familly_users,
                        vec_parent_names,
                        vec_parent_id,
                    );
                }
            }
        }
        vec_parent_names.to_vec()
    }
}
