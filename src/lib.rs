use near_sdk::{
    env::{self, current_account_id},
    log, near, require,
    store::{LookupMap, Vector},
    AccountId,
};
//, near_bindgen, env, store::{LookupMap, Vector}};
//use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

const COMMIT_PROPOSAL_DURATION_BLOCKS: u64 = 5; //REMEMBER EPOCH
const REVEAL_PROPOSAL_DURATION_BLOCKS: u64 = 1;

// Define the contract structure
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

// //To initialize the contract
// impl Default for Contract{
//     fn default() -> Self {
//         Self {
//             creator: env::current_account_id(),
//             guess_deadline: env::block_height() + COMMIT_PROPOSAL_DURATION_BLOCKS,
//             reveal_deadline: env::block_height() + COMMIT_PROPOSAL_DURATION_BLOCKS + REVEAL_PROPOSAL_DURATION_BLOCKS,
//             total_prize: 100, //esta variable creo que no se usaría
//             commitments: LookupMap::new(b"map_com".to_vec()),  //verificar los ultimos 3 valores
//             winners: Vector::new(b"vec_com".to_vec()),
//             claimed: LookupMap::new(b"map_claim".to_vec()),
//         }
//     }
// }

#[near]
impl Contract {
    #[init]
    pub fn new(commitment_offchain : String) -> Self {
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

    //constructor recibe el commitment del creador
    pub fn set_commit_creator(&mut self, correct_answer: String) {
        self.commitments
            .insert(env::current_account_id(), correct_answer);
    }

    //Agrega el commitment de un participante
    pub fn new_commitment(user: AccountId, answer: String) -> String {
        let mut concat_values = user.to_string();
        concat_values.push_str(&answer);
        let concat_values = concat_values.as_bytes();
        let hash_value = env::keccak256(concat_values);
        let hash_string = String::from_utf8_lossy(&hash_value);
        return hash_string.to_string();
    }

    //agregar los commitment, pero verifica que no sea el creador o alguien que ya habia participado
    pub fn guess(&mut self, user: AccountId, answer: String) {
        //verifica que está dentro del tiempo permitido
        require!(
            env::block_height() < self.guess_deadline,
            "Close proposal time"
        );

        //predecessor_account_id es el accountId que llamó a este método
        require!(
            env::predecessor_account_id() != self.creator,
            "You are the creator"
        );
        self.commitments.insert(user, answer);
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

        //Verificar que el commitment ya existia en la lista
        //TODO la funcion new proposal regresa un vector y el commitment es una LookupMap
        require!(
            Contract::new_commitment(env::predecessor_account_id(), answer)
                == self.commitments.get(&self.creator)
        );

        //Verificar que el commitment del creator tambien está en la lista
        //TODO, buscar el commitment del creator
        require!(Contract::new_proposal_commit(self.creator, answer) == self.commitments[creator]);

        // Si el commitment del jugador y del creador existen,  y el jugador aun no está en
        // la lista de ganadores
        require!(!Contract::is_winner(self, user));
    }

    pub fn is_winner(&mut self, user: AccountId) -> bool {
        let winner = false;

        //TODO: corregir el iterador para ver si ya existe en la lista de ganadores
        for user in self.winners {
            winner = true;
            break;
        }
        return winner;
    }

    pub fn claim(&mut self, user: AccountId) {
        require!(env::block_height() > REVEAL_PROPOSAL_DURATION_BLOCKS);

        //buscar en la lista de claimed que no lo reclamara aun
        require!(self.claimed == false);

        //que la dirección que quiere claimear está en la lista de ganadores
        require!(Contract::is_winner(self, user));

        let payout = self.total_prize / self.winners.len();

        //TODO: buscar en el map de claimed y cambiar el valor a true
        self.claimed[user] = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_self() {
        let contract = Contract::new();
        // this test did not call set_greeting so should return the default "Hello" greeting
        let creator = assert_eq!(contract.get_greeting(), "Hello");
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(contract.get_greeting(), "howdy");
    }
}
