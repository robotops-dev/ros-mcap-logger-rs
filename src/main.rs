use std::{collections::BTreeMap, fs, io::BufWriter};

use anyhow::Result;

use mcap::{Channel, records::MessageHeader, Writer};

fn write_it() -> Result<()> {
    // To set the profile or compression options, see mcap::WriteOptions.
    let mut out = Writer::new(
        BufWriter::new(fs::File::create("out.mcap")?)
    )?;

    // Channels and schemas are automatically assigned ID as they're serialized,
    // and automatically deduplicated with `Arc` when deserialized.
    let my_channel = Channel {
        topic: String::from("cool stuff"),
        schema: None,
        message_encoding: String::from("application/octet-stream"),
        metadata: BTreeMap::default()
    };

    let channel_id = out.add_channel(&my_channel)?;

    out.write_to_known_channel(
        &MessageHeader {
            channel_id,
            sequence: 25,
            log_time: 6,
            publish_time: 24
        },
        &[1, 2, 3]
    )?;
    out.write_to_known_channel(
        &MessageHeader {
            channel_id,
            sequence: 32,
            log_time: 23,
            publish_time: 25
        },
        &[3, 4, 5]
    )?;

    out.finish()?;

    Ok(())
}

fn main() {
    write_it();
}
