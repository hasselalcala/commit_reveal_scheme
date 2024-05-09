use near_sdk::{
    env::{self}, near, require,
    store::{LookupMap, Vector},
    AccountId,
};


const COMMIT_PROPOSAL_DURATION_BLOCKS: u64 = 5; 
const REVEAL_PROPOSAL_DURATION_BLOCKS: u64 = 1;


#[near(contract_state)]
pub struct Contract {
    creator: AccountId,
    guess_deadline: u64,
    reveal_deadline: u64,
    total_prize: u64,
    commitments: LookupMap<AccountId, String>,
    winners: Vector<AccountId>,
    claimed: LookupMap<AccountId, bool>,
}


impl Default for Contract {
    fn default() -> Self {
        Self {
            creator: env::current_account_id(),
            guess_deadline: env::block_height() + COMMIT_PROPOSAL_DURATION_BLOCKS,
            reveal_deadline: env::block_height()
                + COMMIT_PROPOSAL_DURATION_BLOCKS
                + REVEAL_PROPOSAL_DURATION_BLOCKS,
            total_prize: 100, 
            commitments: LookupMap::new(b"map_com".to_vec()),
            winners: Vector::new(b"vec_com".to_vec()),
            claimed: LookupMap::new(b"map_claim".to_vec()),
        }
    }
}

#[near]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            creator: env::current_account_id(),
            guess_deadline: env::block_height() + COMMIT_PROPOSAL_DURATION_BLOCKS,
            reveal_deadline: env::block_height()
                + COMMIT_PROPOSAL_DURATION_BLOCKS
                + REVEAL_PROPOSAL_DURATION_BLOCKS,
            total_prize: 100,
            commitments: LookupMap::new(b"map_com".to_vec()),
            winners: Vector::new(b"vec_com".to_vec()),
            claimed: LookupMap::new(b"map_claim".to_vec()),
        }
    }

    pub fn set_commit_creator(&mut self, correct_answer: String) {
        let caller = env::current_account_id();
        require!(caller == self.creator, "You are not the owner");
        let value = Contract::new_commitment(correct_answer);
        self.commitments.insert(caller, value);
    }

    pub fn new_commitment(answer: String) -> String {
        let answer_as_bytes = answer.as_bytes();
        let hash_value = env::keccak256(answer_as_bytes);
        let hash_string = String::from_utf8_lossy(&hash_value);
        return hash_string.to_string();
    }

    pub fn guess(&mut self, user: AccountId, answer: String) -> bool{
      
        if env::block_height() >= self.guess_deadline {
            return false;
        }
    
        if user == self.creator {
            return false;
        }
    
        let value = Contract::new_commitment(answer);
        self.commitments.insert(user, value);
        true
    }

    pub fn reveal_proposal(&mut self, answer: String) {
        require!(
            env::block_height() > self.guess_deadline,
            "Close proposal time, we are in reveal time"
        );
        require!(
            env::block_height() < self.reveal_deadline,
            "Close reveal time"
        );

        let answer_to_verify = Contract::new_commitment(answer);
        let answer_saved = self.commitments.get(&env::predecessor_account_id());

        if let Some(answer) = answer_saved {
            require!(*answer == answer_to_verify, "No answer register");
        }

        let creator = self.creator.clone();
        let answer_creator = self.commitments.get(&creator);

        require!(Some(answer_creator) == Some(answer_saved));
        require!(!Contract::is_winner(self, env::predecessor_account_id()));
        self.winners.push(env::predecessor_account_id());
    }

    pub fn is_winner(&mut self, user: AccountId) -> bool {
        let mut exist = false;
        for winner in self.winners.iter() {
            if *winner == user {
                exist = true;
                break;
            }
        }
        return exist;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_self() {
        let creator = env::current_account_id();
        let initial_block = env::block_height();
        let contract = Contract::new();

        assert_eq!(contract.creator, creator);
        assert_eq!(contract.guess_deadline, initial_block + 5);
        assert_eq!(contract.reveal_deadline, initial_block + 5 + 1);
        assert_eq!(contract.total_prize, 100);
        assert!(contract.winners.len() == 0, "Instance fail");
    }

    #[test]
    fn test_set_commit_creator() {
        let mut contract = Contract::new();
        let value =Contract::new_commitment("true".to_string());
        contract.set_commit_creator("true".to_string());

        let commit_value =contract.commitments.get(&env::current_account_id());
        match commit_value {
            Some(commitment_string) => {
                assert_eq!(*commitment_string, value);
            }
            None=> {
                panic!("value doesn't exist");
            }
        }
    }

    #[test]
    fn test_guess() {

        let mut contract = Contract::new();
        
        let creator: AccountId = "alice.near".parse().unwrap();
        let user2: AccountId = "edson.near".parse().unwrap();
        let answer = "true".to_string();

        contract.creator = creator.clone();
        contract.guess_deadline = 100; 

        let creator_call = contract.guess(creator, answer.clone());
        assert!(!creator_call, "Creator calls the function");

        let user2_call = contract.guess(user2.clone(), answer.clone());
        assert!(user2_call, "New user can't participate");
    }

    #[test]
    #[should_panic]
    fn test_guess_panic(){

        let mut contract = Contract::new();
        
        let creator: AccountId = "alice.near".parse().unwrap();
        let user1: AccountId = "hassel.near".parse().unwrap();
        let answer = "true".to_string();

        contract.creator = creator.clone();
        contract.guess_deadline = 100;
        contract.guess(user1.clone(), answer.clone());
        let user1_call = contract.guess(user1.clone(), answer.clone());
        assert!(user1_call, "Answer no register");
        let user1_call2 = contract.guess(user1.clone(), answer.clone());
        panic!("You have a register answer");

    }
}
