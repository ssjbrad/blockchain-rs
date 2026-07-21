use borsh::{BorshDeserialize, BorshSerialize};
use ed25519_dalek::{Signature, SigningKey, VerifyingKey};
use ethnum::U256;
use std::collections::HashMap;

fn main() {}

//Basic main data structures

//User accounts
struct User {
    balance: U256,
    nonce: u64,
}

impl User {
    fn new() -> Self {
        Self {
            balance: U256::new(0),
            nonce: 0,
        }
    }
}

//Global state
struct State {
    global_state: HashMap<String, User>,
}

impl State {
    fn new() -> Self {
        Self {
            global_state: HashMap::new(),
        }
    }

    fn add_user(&mut self, address: String, user: User) -> Result<(), StateError> {
        if self.global_state.contains_key(&address) {
            return Err(StateError::AccountExists);
        }
        self.global_state.insert(address, user);
        Ok(())
    }

    fn apply_transaction(&mut self, transaction: &Transaction) -> Result<(), StateError> {
        // phase 1: validate — no mutations
        let sender = self
            .global_state
            .get(&transaction.sender_address)
            .ok_or(StateError::UnknownSender)?;
        if sender.balance < transaction.amount {
            return Err(StateError::InsufficientBalance);
        }
        if sender.nonce != transaction.nonce {
            return Err(StateError::InvalidNonce);
        }

        // check receiver overflow by *reading* current balance (0 if absent)
        let receiver_balance = self
            .global_state
            .get(&transaction.receiver_address)
            .map(|u| u.balance)
            .unwrap_or(U256::new(0));
        receiver_balance
            .checked_add(transaction.amount)
            .ok_or(StateError::BalanceOverflow)?;

        // phase 2: mutate — now everything's validated
        let sender = self
            .global_state
            .get_mut(&transaction.sender_address)
            .unwrap();
        sender.balance -= transaction.amount;
        sender.nonce += 1;

        let receiver = self
            .global_state
            .entry(transaction.receiver_address.clone())
            .or_insert_with(User::new);
        receiver.balance += transaction.amount;
        Ok(())
    }
}

//Transaction requests
struct Transaction {
    receiver_address: String,
    sender_address: String,
    signer: Vec<u8>,
    nonce: u64,
    amount: U256,
}

impl Transaction {
    fn verify_signature(&self) -> Result<(), StateError> {
        //Serialization of transaction info (without signer)
        //serialize transaction fields
        //Verify with signature, fields and public key?

        let fields = SignedFields {
            sender_address: self.sender_address.clone(),
            receiver_address: self.receiver_address.clone(),
            amount: self.amount.to_be_bytes(),
            nonce: self.nonce,
        };
        let message = borsh::to_vec(&fields)?;

        let public_key = &self.sender_address;
        let signature = &self.signer;

        Ok(())
    }
}

//Fields for sigature vefication
#[derive(BorshSerialize)]
struct SignedFields {
    sender_address: String,
    receiver_address: String,
    amount: [u8; 32],
    nonce: u64,
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

//Error Enum

enum StateError {
    AccountExists,
    UnknownSender,
    InsufficientBalance,
    InvalidNonce,
    BalanceOverflow,
}
