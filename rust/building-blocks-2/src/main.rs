use bson;
use ron;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::str;

#[derive(Serialize, Deserialize, Debug)]
struct Move {
    distance: i32,
    horizontal: bool,
}

fn main() {
    let a = Move {
        distance: 2,
        horizontal: true,
    };

    // 1. Exercise: Serialize and deserialize a data structure with serde (JSON).
    {
        println!("Before Serialize: {:?}", a);
        let f = File::create("move.json").unwrap();
        serde_json::to_writer(f, &a).unwrap();

        let f = File::open("move.json").unwrap();
        let b: Move = serde_json::from_reader(f).unwrap();
        assert!(b.distance == 2);
        assert!(b.horizontal);
        println!("After Deserialize: {:?}", b);
    }

    // 2. Exercise: Serialize and deserialize a data structure to a buffer with serde (RON).
    {
        let v = ron::ser::to_string(&a).unwrap().into_bytes();
        println!("See in RON: {:?}", str::from_utf8(&v).unwrap());
    }

    // 1000 different Move values
    let n = 1000;
    let mut va: Vec<Move> = Vec::new();
    for _ in 0..n {
        va.push(Move {
            distance: rand::random::<i32>(),
            horizontal: rand::random::<bool>(),
        });
    }

    // 3. Exercise: Serialize and deserialize 1000 data structures with serde (BSON).
    {
        let mut f = File::create("move.bson").unwrap();
        for m in &va {
            if let Ok(bson::Bson::Document(document)) = bson::to_bson(&m) {
                bson::encode_document(&mut f, &document).unwrap();
            }
        }

        let mut vb: Vec<Move> = Vec::new();
        let mut f = File::open("move.bson").unwrap();
        for i in 0..n {
            let doc = bson::decode_document(&mut f).unwrap();
            let m: Move = bson::from_bson(bson::Bson::Document(doc)).unwrap();
            vb.push(m);

            assert!(va[i].distance == vb[i].distance);
            assert!(va[i].horizontal == vb[i].horizontal);
        }
        assert!(va.len() == vb.len());
    }

    // 3.1 Serializing and deserializing multiple values to a Vec<u8>
    {
        // Serializing is OK
        let mut v: Vec<u8> = Vec::new();
        for m in &va {
            if let Ok(bson::Bson::Document(document)) = bson::to_bson(&m) {
                bson::encode_document(&mut v, &document).unwrap();
            }
        }

        // Not support deserializing directly
        // let doc = bson::decode_document(&mut v[..]).unwrap();

        // Convert to &[u8], which supports std::io::Read
        let mut u = &v[..];
        let mut vb: Vec<Move> = Vec::new();
        for i in 0..n {
            let doc = bson::decode_document(&mut u).unwrap();
            let m: Move = bson::from_bson(bson::Bson::Document(doc)).unwrap();
            vb.push(m);

            assert!(va[i].distance == vb[i].distance);
            assert!(va[i].horizontal == vb[i].horizontal);
        }
        assert!(va.len() == vb.len());
    }
}
