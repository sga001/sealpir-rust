extern crate rand;
extern crate sealpir;
use rand::RngCore;
use sealpir::client::PirClient;
use sealpir::server::PirServer;

#[test]
fn pir_very_small_collection_test() {
    let poly_degree = 2048;
    let log_plain_mod = 19;
    let num = 2;
    let d = 2;

    let mut collection: Vec<[u8; 288]> = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..num {
        let mut x: [u8; 288] = [0; 288];
        rng.fill_bytes(&mut x);
        collection.push(x);
    }

    let truth = collection.clone();

    let mut server = PirServer::new(8168, 288, poly_degree, log_plain_mod, d);
    let mut client = PirClient::new(8168, 288, poly_degree, log_plain_mod, d);

    {
        let key = client.get_key();
        println!("Key size {}", key.len());
        server.set_galois_key(key, 0);
    }

    client.update_params(num, 288, d);
    server.update_params(num, 288, d);

    server.setup(&collection);

    let index = 0;
    let query = client.gen_query(index);
    let reply = server.gen_reply(&query, 0);
    let result = client.decode_reply::<[u8; 288]>(index, &reply);
    assert_eq!(&result[..], &truth[index as usize][..]);
}

#[test]
fn pir_small_collection_test() {
    let poly_degree = 2048;
    let log_plain_mod = 20;
    let num = 100;
    let d = 2;

    let mut collection: Vec<[u8; 288]> = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..num {
        let mut x: [u8; 288] = [0; 288];
        rng.fill_bytes(&mut x);
        collection.push(x);
    }

    let truth = collection.clone();

    let mut server = PirServer::new(8168, 288, poly_degree, log_plain_mod, d);
    let mut client = PirClient::new(8168, 288, poly_degree, log_plain_mod, d);

    {
        let key = client.get_key();
        server.set_galois_key(key, 0);
    }

    client.update_params(num, 288, d);
    server.update_params(num, 288, d);

    server.setup(&collection);

    let index = 0;
    let query = client.gen_query(index);
    let reply = server.gen_reply(&query, 0);
    let result = client.decode_reply::<[u8; 288]>(index, &reply);
    assert_eq!(&result[..], &truth[index as usize][..]);
}

#[test]
fn pir_medium_collection_test() {
    let poly_degree = 2048;
    let log_plain_mod = 23;
    let num = 1 << 16;
    let d = 2;

    let mut collection: Vec<[u8; 288]> = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..num {
        let mut x: [u8; 288] = [0; 288];
        rng.fill_bytes(&mut x);
        collection.push(x);
    }

    let truth = collection.clone();

    let mut server = PirServer::new(8168, 288, poly_degree, log_plain_mod, d);
    let mut client = PirClient::new(8168, 288, poly_degree, log_plain_mod, d);

    {
        let key = client.get_key();
        server.set_galois_key(key, 0);
    }

    client.update_params(num, 288, d);
    server.update_params(num, 288, d);

    server.setup(&collection);

    let index = 0;
    let query = client.gen_query(index);
    let reply = server.gen_reply(&query, 0);
    let result = client.decode_reply::<[u8; 288]>(index, &reply);
    assert_eq!(&result[..], &truth[index as usize][..]);
}

#[test]
fn pir_large_collection_test() {
    let poly_degree = 2048;
    let log_plain_mod = 23;
    let num = 1 << 18;
    let d = 2;

    let mut collection: Vec<[u8; 288]> = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..num {
        let mut x: [u8; 288] = [0; 288];
        rng.fill_bytes(&mut x);
        collection.push(x);
    }

    let truth = collection.clone();

    let mut server = PirServer::new(num, 288, poly_degree, log_plain_mod, d);
    let client = PirClient::new(num, 288, poly_degree, log_plain_mod, d);
    let key = client.get_key();

    server.set_galois_key(key, 0);
    server.setup(&collection);

    let index = 0;
    let query = client.gen_query(index);
    let reply = server.gen_reply(&query, 0);
    let result = client.decode_reply::<[u8; 288]>(index, &reply);
    assert_eq!(&result[..], &truth[index as usize][..]);
}

#[test]
fn pir_very_large_collection_test() {
    let poly_degree = 2048;
    let log_plain_mod = 23;
    let num = 1 << 20;
    let d = 2;

    let mut collection: Vec<[u8; 288]> = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..num {
        let mut x: [u8; 288] = [0; 288];
        rng.fill_bytes(&mut x);
        collection.push(x);
    }

    let truth = collection.clone();

    let mut server = PirServer::new(8168, 288, poly_degree, log_plain_mod, d);
    let mut client = PirClient::new(8168, 288, poly_degree, log_plain_mod, d);

    {
        let key = client.get_key();
        println!("Key size {}", key.len());
        server.set_galois_key(key, 0);
    }

    client.update_params(num, 288, d);
    server.update_params(num, 288, d);

    server.setup(&collection);

    let index = 0;
    let query = client.gen_query(index);
    let reply = server.gen_reply(&query, 0);
    let result = client.decode_reply::<[u8; 288]>(index, &reply);
    assert_eq!(&result[..], &truth[index as usize][..]);
}

#[test]
fn pir_largest_collection_test() {
    let poly_degree = 2048;
    let log_plain_mod = 24;
    let num = 1 << 22;
    let d = 2;

    let mut collection: Vec<[u8; 288]> = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..num {
        let mut x: [u8; 288] = [0; 288];
        rng.fill_bytes(&mut x);
        collection.push(x);
    }

    let truth = collection.clone();

    let mut server = PirServer::new(8168, 288, poly_degree, log_plain_mod, d);
    let mut client = PirClient::new(8168, 288, poly_degree, log_plain_mod, d);

    {
        let key = client.get_key();
        println!("Key size {}", key.len());
        server.set_galois_key(key, 0);
    }

    client.update_params(num, 288, d);
    server.update_params(num, 288, d);

    server.setup(&collection);

    let index = 0;
    let query = client.gen_query(index);
    let reply = server.gen_reply(&query, 0);
    let result = client.decode_reply::<[u8; 288]>(index, &reply);
    assert_eq!(&result[..], &truth[index as usize][..]);
}

#[test]
fn pir_sizes() {
    let size = 288;
    let index = 70;

    let mut collection: Vec<[u8; 288]> = Vec::new();

    let mut rng = rand::thread_rng();

    let logts = vec![20, 23];
    let ds = vec![2];
    let ns = vec![1 << 16, 1 << 18, 1 << 20];

    let mut num_prev = 0;

    for num in ns {
        for _ in num_prev..num {
            let mut x: [u8; 288] = [0; 288];
            rng.fill_bytes(&mut x);

            collection.push(x);
        }

        num_prev = num;

        for d in &ds {
            for logt in &logts {
                let mut server = PirServer::new(num, size, 2048, *logt, *d);


                let client = PirClient::new(num, size, 2048, *logt, *d);
                let galois = client.get_key();

                server.setup(&collection);
                server.set_galois_key(&galois, 0);

                let query = client.gen_query(index);
                let reply = server.gen_reply(&query, 0);

                println!(
                    "query: num {}, logt {}, d {}, size {}",
                    num,
                    *logt,
                    *d,
                    query.query.len() / 1024
                );
                println!(
                    "reply num {}, logt {}, d {}, size {}",
                    num,
                    *logt,
                    *d,
                    reply.reply.len() / 1024
                );

            }
        }
    }
}
