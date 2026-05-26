#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Address, Env, Symbol, Vec,
};

#[contract]
pub struct ETrikePayContract;

#[contracttype]
#[derive(Clone)]
pub struct Payment {
    pub passenger: Address,
    pub driver: Address,
    pub amount: i128,
    pub timestamp: u64,
}

const PAYMENTS: Symbol = symbol_short!("PAYMENTS");

#[contractimpl]
impl ETrikePayContract {

    // Save fare payment transaction
    pub fn pay_fare(
        env: Env,
        passenger: Address,
        driver: Address,
        amount: i128,
    ) {

        // Require passenger authorization
        passenger.require_auth();

        // Get existing payment list
        let mut payments: Vec<Payment> =
            env.storage().instance().get(&PAYMENTS)
            .unwrap_or(Vec::new(&env));

        // Create payment object
        let payment = Payment {
            passenger,
            driver,
            amount,
            timestamp: env.ledger().timestamp(),
        };

        // Store payment
        payments.push_back(payment);

        env.storage().instance().set(&PAYMENTS, &payments);
    }

    // Retrieve all fare payments
    pub fn get_payments(env: Env) -> Vec<Payment> {
        env.storage()
            .instance()
            .get(&PAYMENTS)
            .unwrap_or(Vec::new(&env))
    }
}