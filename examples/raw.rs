// Copyright 2018 TiKV Project Authors. Licensed under Apache-2.0.

#![type_length_limit = "8165158"]

mod common;

use std::collections::VecDeque;

use std::fmt::Write;
use std::mem;
use std::sync::Arc;
use std::time::Duration;




use indicatif::MultiProgress;
use indicatif::ProgressBar;

use indicatif::ProgressState;
use indicatif::ProgressStyle;
use tikv_client::Config;
use tikv_client::IntoOwnedRange;
use tikv_client::Key;
use tikv_client::KvPair;
use tikv_client::RawClient as Client;
use tikv_client::Result;
use tikv_client::Value;


use crate::common::parse_args;

const KEY: &str = "TiKV";
const VALUE: &str = "Rust";

async fn client_put(pb: Arc<ProgressBar>, client: Arc<Client>, k: Key, v: Value) -> i32 {
    let data_len = v.len();
    let result = client.put(k, v).await;
    if let Err(e) = result {
        eprintln!("error: {e:?}");
        0
    } else {
        pb.clone().inc(data_len as u64);
        1
    }
}

async fn client_batch_put(pb: Arc<ProgressBar>, client: Arc<Client>, data: VecDeque<KvPair>) -> i32 {
    let data_len = data.iter().fold(0, |c, d|c+d.value().len());
    let result = client.batch_put(data).await;
    if let Err(e) = result {
        eprintln!("error: {e:?}");
        0
    } else {
        pb.clone().inc(data_len as u64);
        1
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // You can try running this example by passing your pd endpoints
    // (and SSL options if necessary) through command line arguments.
    let args = parse_args("raw");

    // Create a configuration to use for the example.
    // Optionally encrypt the traffic.
    let mut config = if let (Some(ca), Some(cert), Some(key)) = (args.ca, args.cert, args.key) {
        Config::default().with_security(ca, cert, key)
    } else {
        Config::default()
    }
    // This example uses the default keyspace, so api-v2 must be enabled on the server.
    .with_default_keyspace();

    config.timeout = Duration::from_secs(60);

    // When we first create a client we receive a `Connect` structure which must be resolved before
    // the client is actually connected and usable.
    let client = Arc::new(Client::new_with_config(args.pd, config).await?);

    // Requests are created from the connected client. These calls return structures which
    // implement `Future`. This means the `Future` must be resolved before the action ever takes
    // place.
    //
    // Here we set the key `TiKV` to have the value `Rust` associated with it.
    client.put(KEY.to_owned(), VALUE.to_owned()).await.unwrap(); // Returns a `tikv_client::Error` on failure.
    println!("Put key {KEY:?}, value {VALUE:?}.");

    // Unlike a standard Rust HashMap all calls take owned values. This is because under the hood
    // protobufs must take ownership of the data. If we only took a borrow we'd need to internally
    // clone it. This is against Rust API guidelines, so you must manage this yourself.
    //
    // Above, you saw we can use a `&'static str`, this is primarily for making examples short.
    // This type is practical to use for real things, and usage forces an internal copy.
    //
    // It is best to pass a `Vec<u8>` in terms of explicitness and speed. `String`s and a few other
    // types are supported as well, but it all ends up as `Vec<u8>` in the end.
    let value: Option<Value> = client.get(KEY.to_owned()).await?;
    assert_eq!(value, Some(Value::from(VALUE.to_owned())));
    println!("Get key `{KEY}` returned value {value:?}.");

    // You can also set the `ColumnFamily` used by the request.
    // This is *advanced usage* and should have some special considerations.
    client
        .delete(KEY.to_owned())
        .await
        .expect("Could not delete value");
    println!("Key: `{KEY}` deleted");

    // Here we check if the key has been deleted from the key-value store.
    let value: Option<Value> = client
        .get(KEY.to_owned())
        .await
        .expect("Could not get just deleted entry");
    assert!(value.is_none());

    // You can ask to write multiple key-values at the same time, it is much more
    // performant because it is passed in one request to the key-value store.
    let pairs = vec![
        KvPair::from(("k1".to_owned(), "v1".to_owned())),
        KvPair::from(("k2".to_owned(), "v2".to_owned())),
        KvPair::from(("k3".to_owned(), "v3".to_owned())),
    ];
    client.batch_put(pairs.clone()).await.expect("Could not put pairs");
    client.batch_put(pairs).await.expect("Could not put pairs a second time");

    // Same thing when you want to retrieve multiple values.
    let keys = vec![Key::from("k1".to_owned()), Key::from("k2".to_owned())];
    let values = client
        .batch_get(keys.clone())
        .await
        .expect("Could not get values");
    println!("Found values: {values:?} for keys: {keys:?}");

    // Scanning a range of keys is also possible giving it two bounds
    // it will returns all entries between these two.
    let start = "k1";
    let end = "k2";
    let pairs = client
        .scan((start..=end).into_owned(), 10)
        .await
        .expect("Could not scan");

    let keys: Vec<_> = pairs.into_iter().map(|p| p.key().clone()).collect();
    assert_eq!(
        &keys,
        &[Key::from("k1".to_owned()), Key::from("k2".to_owned()),]
    );
    println!("Scanning from {start:?} to {end:?} gives: {keys:?}");

    let k1 = "k1";
    let k2 = "k2";
    let k3 = "k3";
    let batch_scan_keys = vec![
        (k1.to_owned()..=k2.to_owned()),
        (k2.to_owned()..=k3.to_owned()),
        (k1.to_owned()..=k3.to_owned()),
    ];
    let kv_pairs = client
        .batch_scan(batch_scan_keys.to_owned(), 10)
        .await
        .expect("Could not batch scan");
    let vals: Vec<_> = kv_pairs
        .into_iter()
        .map(|p| String::from_utf8(p.1).unwrap())
        .collect();
    assert_eq!(
        &vals,
        &[
            "v1".to_owned(),
            "v2".to_owned(),
            "v2".to_owned(),
            "v3".to_owned(),
            "v1".to_owned(),
            "v2".to_owned(),
            "v3".to_owned()
        ]
    );
    println!("Scanning batch scan from {batch_scan_keys:?} gives: {vals:?}");

    let test_data_size = 1 << 30;   // 1 GB
    let block_size = 64 << 10;      // 64 KB
    let chunk_block_cnt = 16;
    let block_cnt = test_data_size / block_size;

    let m = MultiProgress::new();
    let sty = ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-");

    let pb = m.add(ProgressBar::new(test_data_size));
    pb.set_style(sty.clone());
    pb.set_message("todo");
    let pb2 = m.add(ProgressBar::new(test_data_size));
    pb2.set_style(sty.clone());
    pb2.set_message("finished");

    let pb3 = Arc::new(m.insert_after(&pb2, ProgressBar::new(test_data_size)));
    pb3.set_style(sty);

    m.println("starting!").unwrap();
    let id = rand::random::<u64>();
    println!("rand id: {id}\n\n\n");

    let mut success_cnt = 0;
    let mut done = Vec::new();
    let mut progress = VecDeque::new();
    let mut progress2 = VecDeque::new();
    for i in 0..block_cnt {
        let kv_pair = {
            let mut data = Vec::with_capacity(block_size as usize);
            for _j in 0..(block_size / 8) {
                let rv = rand::random::<u64>().to_ne_bytes();
                data.extend(rv.iter());
            }
            pb.inc(block_size);
            KvPair(format!("RKB64_TEST{id}_{i}").into(), data)
        };

        pb2.inc(block_size);

        progress2.push_back(kv_pair);

        if progress2.len() < 4 {
            continue;
        }

        if progress.len() == 1 {
            let KvPair(k,v) = progress2.pop_front().unwrap();
            progress.push_back(tokio::spawn(client_put(pb3.clone(), client.clone(), k, v)));
        } else {
            progress.push_back(tokio::spawn(client_batch_put(pb3.clone(), client.clone(), mem::take(&mut progress2))));
        }

        if progress.len() > 4 {
            let result = progress.pop_front().unwrap();
            let r = result.await;
            match &r {
                Ok(v) => { success_cnt += v; },
                Err(e) => {
                    eprintln!("join error: {e:?}");
                }
            }
            done.push(r);
        }
    }

    for jh in progress {
        let r = jh.await;
        done.push(r);
    }

    println!("put {success_cnt} * {chunk_block_cnt} * {block_size} ({} bytes) into the TiKV", success_cnt as u64 * chunk_block_cnt as u64 * block_size);

    // Delete all keys in the whole range.
    //client.delete_range("".to_owned().."".to_owned()).await?;

    Ok(())
}
