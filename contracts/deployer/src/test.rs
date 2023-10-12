// In order to run the test file. You need to put values 
// that has been left blank in the test file.

#![cfg(test)]
extern crate alloc;
extern crate std;

use crate::{Deployer, DeployerClient};
use alloc::vec;
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    xdr::{self, ContractIdPreimage, ContractIdPreimageFromAddress, CreateContractArgs, Uint256},
    Address, BytesN, Env, IntoVal, Val, Vec,
};

// Replace the file path with Equity contract Wasm.
mod contract {
    soroban_sdk::contractimport!(
        file =
            "<Equity Token WASM path>"
    );
}

#[test]
fn test_deploy_from_contract() {
    let env = Env::default();
    let deployer_client = DeployerClient::new(&env, &env.register_contract(None, Deployer));

    // Upload the Wasm to be deployed from the deployer contract.
    // This can also be called from within a contract if needed.
    let wasm_hash = env.deployer().upload_contract_wasm(contract::WASM);

    // Deploy contract using deployer, and include an init function to call.
    let salt = BytesN::from_array(&env, &[0; 32]);
    let init_fn = symbol_short!("init");


    // Set the values for the variables according to the Equity contract
    // This is important to make sure that the contract is initialized properly

    let url: String = ...; // Company URL
    let ipfs: String = ...; // IPFS hash
    let desc: String = ...; // Description of the company
    let equity_diluted: u32 = ...; // Equity Diluted
    let total_tokens: i128 = ...; // Total Tokens to supply
    let admin: Address = ...; // Admin Address
    let decimal: u32 = ...; // initialize the decimal variable
    let name: String = ...; // Company Name
    let symbol: String = ...; // Company Symbol
    
    let mut init_args: Vec<Val> = Vec::new();
    init_args.push(Val::from(env)); // push the Env variable to the vector
    init_args.push(Val::from(url)); // push the url variable to the vector
    init_args.push(Val::from(ipfs)); // push the ipfs variable to the vector
    init_args.push(Val::from(desc)); // push the desc variable to the vector
    init_args.push(Val::from(equity_diluted)); // push the equity_diluted variable to the vector
    init_args.push(Val::from(total_tokens)); // push the total_tokens variable to the vector
    init_args.push(Val::from(admin)); // push the admin variable to the vector
    init_args.push(Val::from(decimal)); // push the decimal variable to the vector
    init_args.push(Val::from(name)); // push the name variable to the vector
    init_args.push(Val::from(symbol)); // push the symbol variable to the vector

    env.mock_all_auths();
    let (contract_id, init_result) = deployer_client.deploy(
        &deployer_client.address,
        &wasm_hash,
        &salt,
        &init_fn,
        &init_fn_args,
    );

    assert!(init_result.is_void());
    // No authorizations needed - the contract acts as a factory.
    assert_eq!(env.auths(), vec![]);

    // Invoke equity contract to check that it is initialized.
    let client = contract::Client::new(&env, &contract_id);
    let sum = client.value();
    assert_eq!(sum, 5);
}
