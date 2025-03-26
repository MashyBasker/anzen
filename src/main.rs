use clap::{Parser, Subcommand};
use keylist::GPGKey;
use prettytable::{format, row, Table};

mod git;
mod keylist;

#[derive(Parser)]
#[command(name = "az")]
#[command(about = "A minimal GPG key management tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List {},
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List {} => {
            let gpg_keys = keylist::get_keys()?;
            display_gpg_key_info(gpg_keys)?;
        }
    }
    Ok(())
}

fn display_gpg_key_info(gpg_key_list: Vec<GPGKey>) -> Result<(), Box<dyn std::error::Error>> {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_TITLE);

    table.set_titles(row![
        "ID",
        "User",
        "Created On",
        "Expiring On",
        "Signature",
        "Algorithm"
    ]);

    for key in gpg_key_list {
        let user = format!("{}\n({})", key.username, key.email.unwrap());
        let created_date = key.creation_date.unwrap();
        let expiry_date = key.expiry_date.unwrap_or_else(|| String::from("never"));
        let signature = if key.can_sign {
            String::from("yes")
        } else {
            String::from("no")
        };

        table.add_row(row![
            key.id,
            user,
            created_date,
            expiry_date,
            signature,
            key.algorithm
        ]);
    }

    table.printstd();

    Ok(())
}
