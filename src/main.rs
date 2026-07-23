use borsh::BorshSerialize;
use ed25519_dalek::SignatureError;
use ed25519_dalek::Signer;
use ed25519_dalek::{Signature, SigningKey, VerifyingKey};
use ethnum::U256;
use hex::FromHexError;
use rand::rand_core::UnwrapErr;
use rand::rngs::SysRng;
use std::collections::HashMap;

fn main() {}

//Key data structures

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
        //serialized fields
        let message = borsh::to_vec(&fields)?;

        //signature
        let signature = Signature::try_from(&self.signer[..])?;

        let decoded = hex::decode(&self.sender_address)?;
        let key_array: [u8; 32] = decoded.try_into().map_err(|_| StateError::BadAddress)?;
        let verifying_key = VerifyingKey::from_bytes(&key_array)?;

        verifying_key
            .verify_strict(&message, &signature)
            .map_err(|_| StateError::BadSignature)?;
        Ok(())
    }

    fn sender_sign(&mut self, wallet: &Wallet) -> Result<(), StateError> {
        let fields = SignedFields {
            sender_address: self.sender_address.clone(),
            receiver_address: self.receiver_address.clone(),
            amount: self.amount.to_be_bytes(),
            nonce: self.nonce,
        };
        let secret_key_vec = hex::decode(&wallet.private_key)?;
        let secret_key_array: [u8; 32] = secret_key_vec
            .try_into()
            .map_err(|_| StateError::InvalidSecret)?;

        let signing_key: SigningKey = SigningKey::from_bytes(&secret_key_array);

        let message = borsh::to_vec(&fields)?;
        let signature = signing_key.sign(&message);
        self.signer = signature.to_bytes().to_vec();
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

//Wallet Struct

struct Wallet {
    private_key: String,
    public_key: String,
}

impl Wallet {
    fn new() -> Self {
        let mut csprng = UnwrapErr(SysRng);
        let signing_key = SigningKey::generate(&mut csprng);
        let private_key_bytes = signing_key.to_bytes(); // [u8; 32]
        let private_key = hex::encode(private_key_bytes);
        let public_key_interm = signing_key.verifying_key();
        let public_key = hex::encode(public_key_interm.to_bytes());

        Wallet {
            private_key,
            public_key,
        }
    }
}

//Error Enum

enum StateError {
    AccountExists,
    UnknownSender,
    InsufficientBalance,
    InvalidNonce,
    BalanceOverflow,
    BadSignature,
    SerializationFailed,
    BadAddress,
    DecodeError,
    InvalidSecret,
}

impl From<SignatureError> for StateError {
    fn from(err: SignatureError) -> Self {
        StateError::BadSignature
    }
}

impl From<std::io::Error> for StateError {
    fn from(err: std::io::Error) -> Self {
        StateError::SerializationFailed
    }
}

impl From<FromHexError> for StateError {
    fn from(err: FromHexError) -> Self {
        StateError::DecodeError
    }
}
