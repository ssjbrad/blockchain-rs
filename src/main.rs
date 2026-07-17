use std::collections::HashMap;

fn main() {}

//Basic main data structures

//User accounts
struct User {
    balance: u64,
    nonce: u64,
}

impl User {
    fn new() -> Self {
        Self {
            balance: 0,
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
        let sender_bal = self.global_state.get(&transaction.sender_address);           //FIXING THIS NEXT TIME, NEED TO WORK THROUGH VALIDATION OF SENDER INFO/BAL ETC. 
            if let Some(address) = sender_bal {
                if address.balance >= transaction.amount {
                }
                else {
                    println!("Insufficent balance")
                }
            } 

        let receiver = self
            .global_state
            .entry(transaction.receiver_address)
            .or_insert_with(User::new);
        receiver.balance += transaction.amount;
    }
}


    if let Some(number) = maybe_number {
        println!("The number is: {}", number); // Prints: The number is: 42
    } else {
        println!("There was no number found.");
    }





//So — add_user isn't a public API. Nobody calls it from main. It's an internal step inside apply_transaction, on the receiver path.

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

//Error Enum

enum StateError {
    AccountExists,
}
