#[cfg(test)]
mod cli_tests {

    mod test_whoami_command {
        type TestResult = Result<(), Box<dyn std::error::Error>>;
        const PRG: &str = "space_traders_rust";

        use std::{
            fs::File,
            io::{BufWriter, Write},
        };

        use assert_cmd::Command;
        use assert_fs::prelude::{FileTouch, PathChild};
        use space_traders_rust::{api::ApiResponse, domain::Agent, UserInfo};

        #[test]
        fn reports_status() -> TestResult {
            let current_user_dir = assert_fs::TempDir::new().unwrap();
            let current_user_file = current_user_dir.child("current_user.json");
            current_user_file.touch().unwrap();

            let test_user_info = UserInfo {
                token: String::from("fake_token"),
                agent: Agent {
                    credits: 100,
                    symbol: "Fake_Agent".to_string(),
                    ship_count: None,
                    headquarters: "fake_headquarters".to_string(),
                    starting_faction: "starting_faction".to_string(),
                },
            };

            let api_response_agent: Agent = Agent {
                credits: 1000,
                symbol: "Fake_Agent".to_string(),
                ship_count: None,
                headquarters: "fake_headquarters_2".to_string(),
                starting_faction: "starting_faction_2".to_string(),
            };

            let api_response: ApiResponse<Agent> = ApiResponse {
                data: api_response_agent,
            };
            let file = File::create(current_user_file).unwrap();
            let mut writer = BufWriter::new(file);
            serde_json::to_writer(&mut writer, &test_user_info).unwrap();
            writer.flush().unwrap();

            let mut server = mockito::Server::new();
            let url = server.url();

            let mock = server
                .mock("GET", "/my/agent")
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(serde_json::to_string(&api_response).unwrap())
                .create();

            Command::cargo_bin(PRG)?
                .args(&["whoami"])
                .env("TEST_CURRENT_USER_DIR", current_user_dir.to_str().unwrap())
                .env("TEST_API_BASE_URL", url)
                .assert()
                .success();

            mock.assert();

            Ok(())
        }
    }
}
