#![cfg_attr(
    not(test),
    deny(clippy::expect_used, clippy::unwrap_used, clippy::panic, clippy::todo, clippy::indexing_slicing,)
)]
#![deny(irrefutable_let_patterns)]

use std::process;

use clap::Command;

mod command_add;
mod command_diff;
mod command_docs;
mod command_info;
mod command_init;
mod command_list;
mod command_mcp;
mod command_search;
mod command_starters;
mod command_update;
mod command_view;
mod shared;

// * cargo run --bin ui init
// * cargo run --bin ui add button demo_button demo_button_variants demo_button_sizes
// * cargo run --bin ui add demo_use_floating_placement
// * cargo run --bin ui starters

/* ========================================================== */
/*                         🦀 MAIN 🦀                         */
/* ========================================================== */

#[tokio::main]
async fn main() {
    let mut mut_program = Command::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(command_init::_init::command_init())
        .subcommand(command_add::_add::command_add())
        .subcommand(command_info::_info::command_info())
        .subcommand(command_list::_list::command_list())
        .subcommand(command_search::_search::command_search())
        .subcommand(command_update::_update::command_update())
        .subcommand(command_diff::_diff::command_diff())
        .subcommand(command_docs::_docs::command_docs())
        .subcommand(command_starters::_starters::command_starters())
        .subcommand(command_view::_view::command_view())
        .subcommand(command_mcp::_mcp::command_mcp());

    let matches = mut_program.clone().get_matches();

    // Handle commands
    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            let force = sub_matches.get_flag("yes") || sub_matches.get_flag("force");
            let reinstall = if sub_matches.get_flag("reinstall") { Some(true) } else { None };
            let rtl = if sub_matches.get_flag("rtl") {
                Some(true)
            } else if sub_matches.get_flag("no-rtl") {
                Some(false)
            } else {
                None
            };
            match command_init::_init::process_init(force, reinstall, rtl).await {
                Err(e) => {
                    eprintln!("{e}");
                    process::exit(1);
                }
                Ok(outcome) if !outcome.to_reinstall.is_empty() => {
                    if let Err(e) = command_add::_add::process_add_components(
                        outcome.to_reinstall,
                        &outcome.base_path,
                        outcome.rtl,
                    )
                    .await
                    {
                        eprintln!("{e}");
                        process::exit(1);
                    }
                }
                Ok(_) => {}
            }
        }
        Some(("add", sub_matches)) => {
            if let Err(e) = command_add::_add::process_add(sub_matches).await {
                eprintln!("{e}");
                process::exit(1);
            }
        }
        Some(("info", sub_matches)) => {
            if let Err(e) = command_info::_info::process_info(sub_matches) {
                eprintln!("{e}");
                process::exit(1);
            }
        }
        Some(("list", sub_matches)) => {
            if let Err(e) = command_list::_list::process_list(sub_matches).await {
                eprintln!("{e}");
                process::exit(1);
            }
        }
        Some(("search", sub_matches)) => {
            if let Err(e) = command_search::_search::process_search(sub_matches).await {
                eprintln!("{e}");
                process::exit(1);
            }
        }
        Some(("update", sub_matches)) => {
            if let Err(e) = command_update::_update::process_update(sub_matches).await {
                eprintln!("{e}");
                process::exit(1);
            }
        }
        Some(("diff", sub_matches)) => {
            if let Err(e) = command_diff::_diff::process_diff(sub_matches).await {
                eprintln!("{e}");
                process::exit(1);
            }
        }
        Some(("docs", sub_matches)) => {
            if let Err(e) = command_docs::_docs::process_docs(sub_matches) {
                eprintln!("{e}");
                process::exit(1);
            }
        }
        Some(("starters", _)) => {
            if let Err(e) = command_starters::_starters::process_starters().await {
                eprintln!("{e}");
                process::exit(1);
            }
        }
        Some(("view", sub_matches)) => {
            if let Err(e) = command_view::_view::process_view(sub_matches).await {
                eprintln!("{e}");
                process::exit(1);
            }
        }
        Some(("mcp", sub_matches)) => match sub_matches.subcommand() {
            Some(("init", init_matches)) => {
                if let Err(e) = command_mcp::_mcp::process_mcp_init(init_matches) {
                    eprintln!("{e}");
                    process::exit(1);
                }
            }
            _ => {
                if let Err(e) = command_mcp::_mcp::process_mcp_server().await {
                    eprintln!("{e}");
                    process::exit(1);
                }
            }
        },
        _ => {
            if let Err(err) = mut_program.print_help() {
                eprintln!("Error printing help: {err}");
            }
            process::exit(1);
        }
    }
}
