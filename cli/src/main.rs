pub mod options;
pub mod generators;
pub mod commands;
pub mod launchers;
pub mod fs;
pub mod utils;
pub mod formatters;

use self::formatters::{
    format_body, 
    results_to_string, 
    results_to_string_github, 
    results_issues_count,
    results_issues_errors_count,
    results_issues_warnings_count
};
use clap::{Parser};
use std::env;
use options::{Cli, Commands};
use commands::{Build, Start, Stop, Deploy, ApiClient};
use fs::temp::{TempFs};
use serde_json::{json};
use std::io::{self, Write};

const INCLUDE_FRONTEND: &str = "INCLUDE_FRONTEND";
const EXTERNAL: &str = "EXTERNAL";

fn main() {
    let cli = Cli::parse();
    let mut file_manager = TempFs::new();

    let api_token = cli.set_token.unwrap_or_default();
    let cv_token = cli.set_cv_token.unwrap_or_default();
    let cv_url = cli.set_cv_url.unwrap_or_default();

    if !api_token.is_empty() {
        file_manager.set_token(&api_token).unwrap();
        io::stdout().write_all(&"API token saved".as_bytes()).unwrap();
    }

    if !cv_token.is_empty() {
        file_manager.set_cv_token(&cv_token).unwrap();
    }

    if !cv_url.is_empty() {
        file_manager.set_cv_url(&cv_url).unwrap();
    }
    
    if cli.find_results {
        println!("{}", &file_manager.results_file);
    }

    if cli.github_results_path {
        println!("{}", &file_manager.results_github_file);
    }

    if cli.find_tmp_dir {
        println!("{}", &file_manager.get_temp_dir());
    }

    if cli.github_api_url {
        use self::utils::get_api;
        println!("{}", &get_api());
    }

    if cli.results_parsed {
       println!("{}", results_to_string(&file_manager));
    }

    if cli.results_parsed_github {
        println!("{}", results_to_string_github(&file_manager));
    }

    if cli.results_issues {
        let count = results_issues_count(&file_manager);

        println!("{}", count);
    }
    if cli.results_issues_errors {
        let count = results_issues_errors_count(&file_manager);

        println!("{}", count);
    }

    if cli.results_issues_warnings {
        let count = results_issues_warnings_count(&file_manager);

        println!("{}", count);
    }

    match &cli.command {
        Some(Commands::BUILD { frontend, local }) => {
            env::set_var(INCLUDE_FRONTEND, frontend.to_string());
            Build::process(&local);
        },
        Some(Commands::START { frontend, local, upgrade }) => {
            env::set_var(INCLUDE_FRONTEND, frontend.to_string());
            if *upgrade {
                Build::upgrade(&local);
            }
            Start::process(&local);
        },
        Some(Commands::STOP { frontend, local }) => {
            env::set_var(INCLUDE_FRONTEND, frontend.to_string());
            Stop::process(&local);
        },
        Some(Commands::DEPLOY { frontend, all, backend }) => {
            if !frontend && !backend {
                Deploy::process(&all);
            }
        },
        Some(Commands::TERMINATE { frontend, all, backend }) => {
            if !frontend && !backend {
                Deploy::process_terminate(&all);
            }
        },
        Some(Commands::SCAN { url, external, save }) => {
            if *external {
                env::set_var(EXTERNAL, external.to_string());
            }

            let result = ApiClient::scan_website(&url, &file_manager).unwrap_or_default();
            let json_results = json!(result);

            if *save {
                file_manager.save_results(&json_results).unwrap();
            }

            println!("{}", json_results);
        },
        Some(Commands::EXTRACT { platform }) => {
            if platform == "github" {
                let json_data = format_body(&file_manager, cli.github_results_path);
                println!("{}", json_data);
            }
        },
        Some(Commands::CRAWL { url, external, save }) => {
            if *external {
                env::set_var(EXTERNAL, external.to_string());
            }
            let result = ApiClient::crawl_website(&url, &file_manager).unwrap_or_default();
            let json_results = json!(result);

            if *save {
                TempFs::new().save_results(&json_results).unwrap();
            }

            println!("{}", json_results);
        },
        None => {}
    }
}