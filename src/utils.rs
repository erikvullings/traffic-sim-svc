use std::env;

/// Environment variables
pub struct Variables {
    pub broker: String,
    pub schema: String,
    pub group_id: String,
}

/// Read environment variables to get the Kafka broker and schema registry.
pub fn read_env() -> Variables {
    let broker = env::var("KAFKA_HOST").unwrap_or("localhost:3501".to_string());
    let schema = env::var("SCHEMA_REGISTRY").unwrap_or("http://localhost:3502".to_string());
    let group_id = env::var("GROUP_ID").unwrap_or("traffic_sim_svc".to_string());
    Variables {
        broker,
        schema,
        group_id,
    }
}
