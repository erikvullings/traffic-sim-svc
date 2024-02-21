use apache_avro::from_value;
use rdkafka::{
    config::RDKafkaLogLevel,
    consumer::{CommitMode, Consumer, StreamConsumer},
    ClientConfig, Message,
};
use schema_registry_converter::async_impl::{
    easy_avro::EasyAvroDecoder, schema_registry::SrSettings,
};
use serde::Deserialize;
use std::fmt::Debug;
use tokio::sync::mpsc::UnboundedSender;

pub struct KafkaConsumer {
    consumer: StreamConsumer,
    avro_decoder: EasyAvroDecoder,
    topic: String,
}

impl KafkaConsumer {
    pub fn new(
        bootstrap_servers: String,
        schema_registry_url: String,
        group_id: String,
        topic: String,
    ) -> Self {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", group_id)
            .set("bootstrap.servers", bootstrap_servers)
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "false")
            .set("auto.offset.reset", "latest")
            .set_log_level(RDKafkaLogLevel::Debug)
            .create()
            .expect("Consumer creation error");
        let sr_settings = SrSettings::new(schema_registry_url);
        let avro_decoder = EasyAvroDecoder::new(sr_settings);
        Self {
            consumer,
            topic,
            avro_decoder,
        }
    }

    pub async fn consume<T: Clone + Debug + for<'a> Deserialize<'a>>(
        &self,
        sender: UnboundedSender<T>,
    ) {
        self.consumer
            .subscribe(&[&self.topic])
            .expect("Can't subscribe to specific topics");

        while let Ok(message) = self.consumer.recv().await {
            let value_result = match self.avro_decoder.decode(message.payload()).await {
                Ok(v) => Ok(v.value),
                Err(e) => {
                    println!("Error getting value: {}", e);
                    Err(e)
                }
            };

            if let Ok(value) = value_result {
                if let Ok(deserialized_payload) = from_value::<T>(&value) {
                    // println!(
                    //                 "key: '{:?}', payload: '{:?}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                    //                 message.key(),
                    //                 deserialized_payload,
                    //                 message.topic(),
                    //                 message.partition(),
                    //                 message.offset(),
                    //                 message.timestamp()
                    //             );
                    if let Err(e) = sender.send(deserialized_payload) {
                        panic!("Error while sending via channel: {}", e);
                    }
                    // println!("Message consumed successfully");
                } else {
                    panic!("Error while deserializing message payload");
                }
            } else {
                panic!("Error while deserializing message payload");
            }
            self.consumer
                .commit_message(&message, CommitMode::Async)
                .unwrap();
        }
    }
}
