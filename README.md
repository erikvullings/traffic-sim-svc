# Traffic simulation service

A simple service that:

- Connects to Kafka.
- Consumes `simulation_time_mgmt` messages (slightly adapted from the original, since `logicalType: 'timestamp-millis` gave problems) to set the time and state.

TODO

- Consumes `simulation_request_move` to compute a route (received, for example, from the TMT).
- Queries Valhalla routing service to request a route, including manoeuvres, for that route.
- Publishes routes to Kafka as GeoJSON polyline (what topic?).
- Consumes `start_inject` to start, stop or clear a simulated entity. The message format is `start ID`, `stop ID` or `clear ID`:
  - Start: the traffic simulator starts moving the entity along the computed route, regularly publishing position updates (GeoJSON Points, what topic?) according to the simulation time. In the properties, its speed is specified.
  - Stop: the traffic simulator stops moving the entity. Upon receiving the stop signal, a current position update is sent.
  - Clear: the traffic simulator removes it from the list of entities to simulate. Upon receiving the stop signal, a current position update is sent, including a `clear` state.

## Debug

```bash
cargo run
```

```bash
cargo run -- --help # for instructions
```

## Tips

Converting an AVRO schema to a Rust struct, use `rsgen-avro PATH_TO_SCHEMA_FILE.AVSC -`.