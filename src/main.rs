use std::{env, path::Path};

use app_dirs2::{app_dir, AppDataType, AppInfo};

#[tokio::main]
async fn main() {
    let app_info: AppInfo = AppInfo {
        name: "space-traders-cli-rust",
        author: "Ali Ahmed",
    };
    let args = space_traders_rust::get_args().unwrap();
    let current_user_dir = app_dir(AppDataType::UserData, &app_info, "current-user");
    let current_user_dir = current_user_dir.expect("Directory not found");
    let current_user_dir_env = env::var("TEST_CURRENT_USER_DIR");
    let config = space_traders_rust::Config {
        current_user_dir: match current_user_dir_env {
            Ok(test_dir) => Box::new(Path::new(&test_dir).to_owned()),
            Err(_) => Box::new(current_user_dir),
        },
    };
    if let Err(e) = space_traders_rust::run(args, config).await {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
