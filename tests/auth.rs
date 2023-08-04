#[cfg(test)]
mod auth_tests {

    mod test_save_user_info {

        use assert_fs::prelude::*;
        use predicates::prelude::*;
        use space_traders_rust::{
            auth::save_user_info,
            domain::{Agent, RegisterResponse},
        };
        use std::fs;

        #[test]
        fn saves_user_info_after_registration() {
            let current_user_dir = assert_fs::TempDir::new().unwrap();
            let current_user_file = current_user_dir.child("current_user.json");
            current_user_file.touch().unwrap();

            let register_resp = RegisterResponse {
                agent: Agent {
                    credits: 100,
                    headquarters: String::from("headquarters"),
                    ship_count: None,
                    starting_faction: String::from("starting_faction"),
                    symbol: String::from("test_agent"),
                },
                token: String::from("dummy_token"),
            };

            save_user_info(&register_resp, &current_user_dir.path());

            let file_contents = fs::read_to_string(current_user_file.path()).unwrap();
            let result = serde_json::to_string(&register_resp).unwrap();
            assert_eq!(file_contents, result);
        }

        #[test]
        fn overwrites_if_called_twice() {
            let current_user_dir = assert_fs::TempDir::new().unwrap();
            let current_user_file = current_user_dir.child("current_user.json");
            current_user_file.touch().unwrap();

            let register_resp_1 = RegisterResponse {
                agent: Agent {
                    credits: 100,
                    headquarters: String::from("headquarters"),
                    ship_count: None,
                    starting_faction: String::from("starting_faction"),
                    symbol: String::from("test_agent"),
                },
                token: String::from("dummy_token"),
            };

            save_user_info(&register_resp_1, &current_user_dir.path());

            let file_contents = fs::read_to_string(current_user_file.path()).unwrap();
            let result = serde_json::to_string(&register_resp_1).unwrap();
            assert_eq!(file_contents, result);

            let register_resp_2 = RegisterResponse {
                agent: Agent {
                    credits: 200,
                    headquarters: String::from("second headquarters"),
                    ship_count: None,
                    starting_faction: String::from("pro_faction"),
                    symbol: String::from("test_agent_2"),
                },
                token: String::from("dummy_token_2"),
            };

            save_user_info(&register_resp_2, &current_user_dir.path());

            let file_contents = fs::read_to_string(current_user_file.path()).unwrap();
            let result = serde_json::to_string(&register_resp_2).unwrap();
            assert_eq!(file_contents, result);
        }
    }
}
