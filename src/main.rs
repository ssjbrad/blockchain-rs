use std::collections::HashMap;

fn main() {}

//Basic main data structures

//User accounts
struct User {
    balance: u64,
    nonce: u64,
}

//Global state
struct State {
    global_state: HashMap<String, User>,
}

//Transaction requests
struct Transaction {
    receiver_address: String,
    sender_address: String,
    signer: Vec<u8>,
    amount: u64,
}

//A block of transaction requests
struct Block {
    transactions: Vec<Transaction>,
    previous_hash: String,
    hash: String,
    timestamp: u64, //unix time, seconds since Jan 1 1970
}

//All transaction requests, chained.
struct Blockchain {
    blocks: Vec<Block>,
}
