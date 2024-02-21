mod consumer;
mod move_listener;
mod simulation_control;
mod simulation_time;
mod utils;

use clap::{Arg, Command};
use utils::read_env;

use crate::simulation_time::SimulationTimer;

#[tokio::main]
async fn main() {
    let env_vars = read_env();
    let matches = Command::new("traffic-sim-svc")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .author("Erik Vullings <erik.vullings@tno.nl>")
        .about("Simple traffic simulator")
        .arg(
            Arg::new("brokers")
                .short('b')
                .long("brokers")
                .help("Broker list in kafka format"),
        )
        .arg(
            Arg::new("schema-registry")
                .short('s')
                .long("schema")
                .help("URL of the schema registry"),
        )
        .arg(
            Arg::new("group_id")
                .short('i')
                .long("id")
                .help("Group id of the consumer group"),
        )
        .get_matches();

    let broker = matches
        .get_one::<String>("brokers")
        .unwrap_or(&env_vars.broker);
    let registry = matches
        .get_one::<String>("schema-registry")
        .unwrap_or(&env_vars.schema);
    let group_id = matches
        .get_one::<String>("group_id")
        .unwrap_or(&env_vars.group_id);
    let sim_timer = SimulationTimer::new(broker, registry, group_id).await;

    println!(
        r#"Connecting to broker at {} and schema registry at {} with GROUP_ID "{}"."#,
        broker, registry, group_id
    );

    loop {
        println!(
            "The simulation state is {:?}, its speed {}, and the time is now {:?}",
            sim_timer.get_state(),
            sim_timer.get_speed(),
            sim_timer.get_time()
        );

        // Pause for a while
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
