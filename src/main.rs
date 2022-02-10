use clap::{AppSettings, Parser, Subcommand};
use idasen::{get_instance, Device, Idasen};
use tokio_stream::StreamExt;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
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
    Position { pos: u16 },

    /// Show position of desk.
    Debug,

    /// Show the current position.
    Show,

    /// Start moving down.
    Down,

    /// Start moving down.
    Up,

    /// Start moving down.
    Stop,

    /// Listen to position and speed changes.
    Listen,
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
        Commands::Position { pos } => {
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
        Commands::Up => {
            let desk: Idasen<_> = get_instance().await?;
            desk.up().await?
        },
        Commands::Down=> {
            let desk: Idasen<_> = get_instance().await?;
            desk.down().await?
        },
        Commands::Stop => {
            let desk: Idasen<_> = get_instance().await?;
            desk.stop().await?
        }
        Commands::Listen => {
            let desk: Idasen<_> = get_instance().await?;
            println!("start listening use CTRL+C to stop");
            let mut stream = desk.position_and_speed_stream().await?;
            while let Some(change) = stream.next().await {
                println!("pos: {:>5} speed: {:>5}", change.position, change.speed);
            }
        }
    }
    Ok(())
}
