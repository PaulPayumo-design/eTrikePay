#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _},
    Address, Env,
};

#[test]
fn test_happy_path_payment() {

    let env = Env::default();

    let passenger = Address::generate(&env);
    let driver = Address::generate(&env);

    ETrikePayContract::pay_fare(
        env.clone(),
        passenger.clone(),
        driver.clone(),
        20,
    );

    let payments =
        ETrikePayContract::get_payments(env);

    assert_eq!(payments.len(), 1);
}

#[test]
#[should_panic]
fn test_unauthorized_payment() {

    let env = Env::default();

    let passenger = Address::generate(&env);
    let driver = Address::generate(&env);

    ETrikePayContract::pay_fare(
        env,
        passenger,
        driver,
        -10,
    );
}

#[test]
fn test_state_verification() {

    let env = Env::default();

    let passenger = Address::generate(&env);
    let driver = Address::generate(&env);

    ETrikePayContract::pay_fare(
        env.clone(),
        passenger.clone(),
        driver.clone(),
        50,
    );

    let payments =
        ETrikePayContract::get_payments(env);

    assert_eq!(payments.get(0).unwrap().amount, 50);
}

#[test]
fn test_multiple_payments() {

    let env = Env::default();

    let passenger = Address::generate(&env);
    let driver = Address::generate(&env);

    ETrikePayContract::pay_fare(
        env.clone(),
        passenger.clone(),
        driver.clone(),
        20,
    );

    ETrikePayContract::pay_fare(
        env.clone(),
        passenger,
        driver,
        30,
    );

    let payments =
        ETrikePayContract::get_payments(env);

    assert_eq!(payments.len(), 2);
}

#[test]
fn test_zero_payment() {

    let env = Env::default();

    let passenger = Address::generate(&env);
    let driver = Address::generate(&env);

    ETrikePayContract::pay_fare(
        env.clone(),
        passenger,
        driver,
        0,
    );

    let payments =
        ETrikePayContract::get_payments(env);

    assert_eq!(payments.len(), 1);
}