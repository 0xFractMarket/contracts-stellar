#![no_std]

use soroban_sdk::{contractimpl, symbol_short, Address, Env, Symbol, Vec, Map};

#[derive(Clone)]
pub struct Proposal {
    pub id: u32,
    pub description: Symbol,
    pub votes_for: u32,
    pub votes_against: u32,
    pub executed: bool,
}

pub struct FractDAO;

#[contractimpl]
impl FractDAO {
    pub fn create_proposal(env: Env, description: Symbol) {
        let mut proposal_id: u32 = env.storage().get(&symbol_short!("last_id")).unwrap_or(0);
        proposal_id += 1;

        let proposal = Proposal {
            id: proposal_id,
            description,
            votes_for: 0,
            votes_against: 0,
            executed: false,
        };

        env.storage().set(&symbol_short!("last_id"), &proposal_id);
        env.storage().set(&Self::proposal_key(proposal_id), &proposal);
    }

    pub fn vote(env: Env, voter: Address, property_contract: Address, proposal_id: u32, in_favor: bool) {
        voter.require_auth();

        let key = Self::voted_key(&voter, proposal_id);
        if env.storage().has(&key) {
            panic!("Ya votaste en esta propuesta");
        }

        let prop = FractionalPropertyClient::new(&env, &property_contract);
        let weight = prop.get_balance(voter.clone());

        let mut proposal: Proposal = env.storage().get(&Self::proposal_key(proposal_id)).expect("Propuesta no encontrada");

        if in_favor {
            proposal.votes_for += weight;
        } else {
            proposal.votes_against += weight;
        }

        env.storage().set(&Self::proposal_key(proposal_id), &proposal);
        env.storage().set(&key, &true);
    }

    pub fn execute(env: Env, proposal_id: u32, quorum: u32) {
        let mut proposal: Proposal = env.storage().get(&Self::proposal_key(proposal_id)).expect("Propuesta no encontrada");

        if proposal.executed {
            panic!("Ya ejecutada");
        }

        let total_votes = proposal.votes_for + proposal.votes_against;
        if total_votes < quorum {
            panic!("Quórum no alcanzado");
        }

        proposal.executed = true;
        env.storage().set(&Self::proposal_key(proposal_id), &proposal);
    }

    fn proposal_key(id: u32) -> Symbol {
        Symbol::short(format!("prop_{}", id).as_str())
    }

    fn voted_key(addr: &Address, proposal_id: u32) -> Symbol {
        Symbol::short(format!("voted_{}_{}", addr.to_string(), proposal_id).as_str())
    }
}

mod external {
    use soroban_sdk::{contractclient, Address, Env};

    #[contractclient(name = "FractionalPropertyClient")]
    pub trait FractionalPropertyTrait {
        fn get_balance(env: Env, user: Address) -> u32;
    }
}
