use std::path::PathBuf;

use clap::{AppSettings, Parser, Subcommand};
use idasen::{get_instance, Device, Idasen};
use tokio_stream::StreamExt;

#[derive(Parser)]
#[clap(version, about)]
#[clap(long_about = "CLI to Control IKEA® IDÅSEN standing desk via Bluetooth.\n(This is neither made nor endorsed by IKEA®.)")]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Moves desk <rel> mm. Positive numbers move up, negative numbers move down.
    Rel { rel: i16 },

    /// Moves desk to position <pos> mm.
    Pos { pos: u16 },

    /// Show the current position.
    Show,

    /// Dump data about desks we can find.
    Debug,

    /// Listen to position and speed changes.
    Listen,

    /// List all saved positions
    List,

    /// Save position of desk under a certain name.
    Save { name: PathBuf },

    /// Move desk to position saved under a certain name.
    Restore { name: PathBuf },
}

async fn report_old_pos_and_move_to<F>(f: F) -> anyhow::Result<()>
where
    F: Fn(i32) -> i32,
{
    let desk: Idasen<_> = get_instance().await?;
    let now = desk.position().await? / 10;
    println!("position {}mm", now);
    let target = clamp(f(now as i32));
    println!("moving to {}mm", target);
    desk.move_to(target * 10).await?;
    Ok(())
}

fn clamp(pos: i32) -> u16 {
    if pos < 620 {
        620
    } else if pos > 1270 {
        1270
    } else {
        pos as u16
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Rel { rel } => {
            report_old_pos_and_move_to(|old| old as i32 + *rel as i32).await?;
        }
        Commands::Pos { pos } => {
            report_old_pos_and_move_to(|_| *pos as i32).await?;
        }
        Commands::Show => {
            let desk: Idasen<_> = get_instance().await?;
            let now = desk.position().await? / 10;
            println!("position {}mm", now);
        }
        Commands::Debug => {
            let desks = idasen::get_desks(None).await?;
            for desk in desks {
                println!("Desk address {}", desk.address());
                println!(
                    "connected: {}",
                    if desk.is_connected().await? {
                        "true"
                    } else {
                        "false"
                    }
                );
                println!(
                    "local_name {}",
                    desk.properties()
                        .await?
                        .and_then(|p| p.local_name)
                        .unwrap_or_else(|| "NONE".to_string())
                );
                println!("services:");
                if let Some(properties) = desk.properties().await? {
                    properties.services.iter().for_each(|c| {
                        println!("    {}", c);
                    });
                    properties.service_data.iter().for_each(|s| {
                        println!("    {}    {:#?}", s.0, s.1);
                    });
                }
                desk.connect().await?;
                desk.discover_services().await?;
                desk.characteristics().iter().for_each(|c| {
                    println!("{} {}", c.uuid, c);
                });
            }
        }
        Commands::Listen => {
            let desk: Idasen<_> = get_instance().await?;
            println!("start listening use CTRL+C to stop");
            let mut stream = desk.position_and_speed_stream().await?;
            while let Some(change) = stream.next().await {
                println!("pos: {:>5} speed: {:>5}", change.position, change.speed);
            }
        }
        Commands::List => {
            if let Some(project_dirs) = directories::ProjectDirs::from("org", "idasen", "cli") {
                std::fs::create_dir_all(project_dirs.config_dir())?;
                for entry in project_dirs.config_dir().read_dir()? {
                    if let Err(err) =  print_dir_entry(entry) {
                        eprintln!("error traversing {}: {}",
                                  project_dirs.config_dir().to_string_lossy(),
                                  err)
                    }
                }
            } else {
                eprintln!("oops");
            }
        },
        Commands::Save{ name } => {
            if let Some(project_dirs) = directories::ProjectDirs::from("org", "idasen", "cli") {
                std::fs::create_dir_all(project_dirs.config_dir())?;
                let fname = project_dirs.config_dir().join(name);

                let desk: Idasen<_> = get_instance().await?;
                let now = desk.position().await? / 10;
                std::fs::write(fname, format!("{}", now))?;
            } else {
                eprintln!("oops");
            }
        },
        Commands::Restore{ name } => {
            if let Some(project_dirs) = directories::ProjectDirs::from("org", "idasen", "cli") {
                std::fs::create_dir_all(project_dirs.config_dir())?;
                let fname = project_dirs.config_dir().join(name);
                let target: u16 = std::fs::read_to_string(fname)?.parse()?;
                report_old_pos_and_move_to(|_| target as i32).await?;
            } else {
                eprintln!("oops");
            }
        },

    }
    Ok(())
}

fn print_dir_entry(entry: Result<std::fs::DirEntry, std::io::Error>) -> anyhow::Result<()> {
    let entry = entry?;
    if entry.file_type()?.is_file() {
        println!("{}", entry.file_name().to_string_lossy());
    }
    Ok(())
}
