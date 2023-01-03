#[macro_use]
extern crate criterion;
extern crate rand;
extern crate sealpir;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use criterion::Criterion;
use rand::{thread_rng, RngCore};
use sealpir::client::PirClient;
use sealpir::server::PirServer;
use std::time::Duration;

const SIZE: usize = 288;
const DIM: u32 = 2;
const LOGT: u32 = 23;
const POLY_DEGREE: u32 = 2048;
const NUMS: [u32; 3] = [1 << 16, 1 << 18, 1 << 20];

#[derive(Serialize, Clone)]
struct Element {
    #[serde(serialize_with = "<[_]>::serialize")]
    e: [u8; SIZE],
}

fn setup(c: &mut Criterion) {
    c.bench_function_over_inputs(
        &format!("setup_d{}", DIM),
        |b, &&num| {
            // setup
            let mut rng = thread_rng();
            let mut collection = vec![];
            for _ in 0..num {
                let mut x = [0u8; SIZE];
                rng.fill_bytes(&mut x);
                collection.push(x);
            }
            // measurement
            b.iter(|| {
                let mut server = PirServer::new(num, SIZE as u32, POLY_DEGREE, LOGT, DIM);
                server.setup(&collection);
            })
        },
        &NUMS,
    );
}

fn query(c: &mut Criterion) {
    c.bench_function_over_inputs(
        &format!("query_d{}", DIM),
        |b, &&num| {
            // setup
            let client = PirClient::new(num, SIZE as u32, POLY_DEGREE, LOGT, DIM);
            // measurement
            b.iter_with_setup(|| rand::random::<u32>() % num, |idx| client.gen_query(idx));
        },
        &NUMS,
    );
}

fn expand(c: &mut Criterion) {
    c.bench_function_over_inputs(
        &format!("expand_d{}", DIM),
        |b, &&num| {
            // setup
            let mut rng = thread_rng();
            let mut collection = vec![];
            for _ in 0..num {
                let mut x = [0u8; SIZE];
                rng.fill_bytes(&mut x);
                collection.push(x);
            }

            let mut server = PirServer::new(num, SIZE as u32, POLY_DEGREE, LOGT, DIM);
            let client = PirClient::new(num, SIZE as u32, POLY_DEGREE, LOGT, DIM);
            let key = client.get_key();
            server.setup(&collection);
            server.set_galois_key(key, 0);

            // measurement
            b.iter_with_setup(
                || client.gen_query(rand::random::<u32>() % num),
                |query| server.expand(&query, 0),
            );
        },
        &NUMS,
    );
}

fn reply(c: &mut Criterion) {
    c.bench_function_over_inputs(
        &format!("reply_d{}", DIM),
        |b, &&num| {
            // setup
            let mut rng = thread_rng();
            let mut collection = vec![];
            for _ in 0..num {
                let mut x = [0u8; SIZE];
                rng.fill_bytes(&mut x);
                collection.push(x);
            }

            let mut server = PirServer::new(num, SIZE as u32, POLY_DEGREE, LOGT, DIM);
            let client = PirClient::new(num, SIZE as u32, POLY_DEGREE, LOGT, DIM);
            let key = client.get_key();
            server.setup(&collection);
            server.set_galois_key(key, 0);

            // measurement
            b.iter_with_setup(
                || client.gen_query(rand::random::<u32>() % num),
                |query| server.gen_reply(&query, 0),
            );
        },
        &NUMS,
    );
}

fn decode(c: &mut Criterion) {
    c.bench_function_over_inputs(
        &format!("decode_d{}", DIM),
        |b, &&num| {
            // setup
            let mut rng = thread_rng();
            let mut collection = vec![];
            for _ in 0..num {
                let mut x = [0u8; SIZE];
                rng.fill_bytes(&mut x);
                collection.push(x);
            }

            let mut server = PirServer::new(num, SIZE as u32, POLY_DEGREE, LOGT, DIM);
            let client = PirClient::new(num, SIZE as u32, POLY_DEGREE, LOGT, DIM);
            let key = client.get_key();
            server.setup(&collection);
            server.set_galois_key(key, 0);
            let idx = rand::random::<u32>() % num;
            let query = client.gen_query(idx);
            let reply = server.gen_reply(&query, 0);

            // measurement
            b.iter(|| client.decode_reply::<Element>(idx, &reply));
        },
        &NUMS,
    );
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(10)
        .measurement_time(Duration::new(5, 0))
        .without_plots();
    targets = setup, query, expand, reply, decode
}

criterion_main!(benches);
