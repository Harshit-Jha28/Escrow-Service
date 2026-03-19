#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, Symbol,
};

#[derive(Clone)]
#[contracttype]
pub struct Escrow {
    pub buyer: Address,
    pub seller: Address,
    pub amount: i128,
    pub released: bool,
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {

    // Initialize escrow
    pub fn create_escrow(
        env: Env,
        buyer: Address,
        seller: Address,
        amount: i128,
    ) {
        buyer.require_auth();

        let escrow = Escrow {
            buyer: buyer.clone(),
            seller,
            amount,
            released: false,
        };

        env.storage().instance().set(&Symbol::new(&env, "ESCROW"), &escrow);
    }

    // Release funds to seller
    pub fn release(env: Env) {
        let mut escrow: Escrow = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "ESCROW"))
            .unwrap();

        escrow.buyer.require_auth();

        if escrow.released {
            panic!("Already released");
        }

        escrow.released = true;

        // Transfer logic (simplified)
        escrow.seller.require_auth();

        env.storage().instance().set(&Symbol::new(&env, "ESCROW"), &escrow);
    }

    // Refund buyer
    pub fn refund(env: Env) {
        let mut escrow: Escrow = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "ESCROW"))
            .unwrap();

        escrow.seller.require_auth();

        if escrow.released {
            panic!("Already released");
        }

        escrow.released = true;

        env.storage().instance().set(&Symbol::new(&env, "ESCROW"), &escrow);
    }

    // View escrow
    pub fn get_escrow(env: Env) -> Escrow {
        env.storage()
            .instance()
            .get(&Symbol::new(&env, "ESCROW"))
            .unwrap()
    }
}
