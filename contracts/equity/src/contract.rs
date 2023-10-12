// Dependencies
use crate::admin::{has_administrator, read_administrator, write_administrator};
use crate::allowance::{read_allowance, spend_allowance, write_allowance};
use crate::balance::{read_balance, receive_balance, spend_balance};
use crate::metadata::{read_decimal, read_name, read_symbol, write_metadata};
use crate::storage_types::{INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD};
use soroban_sdk::contracttype;
use soroban_sdk::token::{self, Interface as _};
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, String, Symbol};
use soroban_token_sdk::metadata::TokenMetadata;
use soroban_token_sdk::TokenUtils;

// Company Data (Will be stored in .env with Company Name as it's key)
// Finding companies with name instead of admin address for ease-of-use and simplicity
// Each company will have a username, providing an Ecosystem.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    equity_diluted: u32,
    total_tokens: i128,
    address: Address,
    company_url: String,
    ipfshash: String,
    company_description: String,
}
const STATE: Symbol = symbol_short!("STATE");

fn check_nonnegative_amount(amount: i128) {
    if amount < 0 {
        panic!("negative amount is not allowed: {}", amount)
    }
}

#[contract]
pub struct Token;

#[contractimpl]
impl Token {
    // Initialize the Token
    // name : Name of the Company
    // url : Website URL of the company
    // ipfs : ipfs hash of the company
    // desc : Description of the company
    // equity_diluted : Equity diluted to create token (as a Stock)
    // total_tokens: Total Stocks in form of tokens
    // admin : Adress of admin
    // symbol : Symbol of token
    pub fn initialize(
        e: Env,
        url: String,
        ipfs: String,
        desc: String,
        equity_diluted: u32,
        total_tokens: i128,
        admin: Address,
        decimal: u32,
        name: String,
        symbol: String,
    ) {
        if has_administrator(&e) {
            panic!("already initialized")
        }
        write_administrator(&e, &admin);
        if decimal > u8::MAX.into() {
            panic!("Decimal must fit in a u8");
        }

        let mut company = Self::get_state(
            e.clone(),
            admin.clone(),
            url.clone(),
            desc.clone(),
            ipfs.clone(),
        );
        company.equity_diluted = equity_diluted;
        company.total_tokens = total_tokens;

        e.storage().instance().set(&name, &company);
        e.storage().instance().bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        Self::mint(e.clone(), admin.clone(), total_tokens); // Minting the tokens

        write_metadata(
            &e,
            TokenMetadata {
                decimal,
                name,
                symbol,
            },
        )
    }

    // URL , Description and IPFS Hash is provided for the investors to know more about the company
    // and to do background check before investing.
    // Equity held can be caluclated as n*equity_diluted/total_tokens
    pub fn get_state(e: Env, add: Address, url: String, desc: String, ipfs: String) -> State {
        e.storage().instance().get(&STATE).unwrap_or(State {
            equity_diluted: 0,
            total_tokens: 0,
            address: add,
            company_url: url,
            ipfshash: ipfs,
            company_description: desc,
        })
    }

    // Returns all the information about the company
    // Will be displayed in the CompanyPage of the website
    // Users can make their decision based on the information provided
    // Companies are encouraged to put as much information as possible (including govt reg id)
    pub fn get_information(e: Env, name: String) -> State {
        let admin = read_administrator(&e);
        let company = e.storage().instance().get(&name).unwrap_or(State {
            equity_diluted: 0,
            total_tokens: 0,
            address: admin,
            company_url: name.clone(),
            ipfshash: name.clone(),
            company_description: name.clone(),
        });
        company
    }

    pub fn mint(e: Env, to: Address, amount: i128) {
        check_nonnegative_amount(amount);
        to.require_auth();

        e.storage()
            .instance()
            .bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        receive_balance(&e, to.clone(), amount);
        TokenUtils::new(&e).events().mint(to.clone(), to, amount);
    }
}

#[contractimpl]
impl token::Interface for Token {
    fn allowance(e: Env, from: Address, spender: Address) -> i128 {
        e.storage()
            .instance()
            .bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_allowance(&e, from, spender).amount
    }

    fn approve(e: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();

        check_nonnegative_amount(amount);

        e.storage()
            .instance()
            .bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        write_allowance(&e, from.clone(), spender.clone(), amount, expiration_ledger);
        TokenUtils::new(&e)
            .events()
            .approve(from, spender, amount, expiration_ledger);
    }

    fn balance(e: Env, id: Address) -> i128 {
        e.storage()
            .instance()
            .bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_balance(&e, id)
    }

    fn spendable_balance(e: Env, id: Address) -> i128 {
        e.storage()
            .instance()
            .bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_balance(&e, id)
    }

    fn transfer(e: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        check_nonnegative_amount(amount);

        e.storage()
            .instance()
            .bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_balance(&e, from.clone(), amount);
        receive_balance(&e, to.clone(), amount);
        TokenUtils::new(&e).events().transfer(from, to, amount);
    }

    fn transfer_from(e: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        check_nonnegative_amount(amount);

        e.storage()
            .instance()
            .bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_allowance(&e, from.clone(), spender, amount);
        spend_balance(&e, from.clone(), amount);
        receive_balance(&e, to.clone(), amount);
        TokenUtils::new(&e).events().transfer(from, to, amount)
    }

    fn burn(e: Env, from: Address, amount: i128) {
        from.require_auth();

        check_nonnegative_amount(amount);

        e.storage()
            .instance()
            .bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_balance(&e, from.clone(), amount);
        TokenUtils::new(&e).events().burn(from, amount);
    }

    fn burn_from(e: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();

        check_nonnegative_amount(amount);

        e.storage()
            .instance()
            .bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_allowance(&e, from.clone(), spender, amount);
        spend_balance(&e, from.clone(), amount);
        TokenUtils::new(&e).events().burn(from, amount)
    }

    fn decimals(e: Env) -> u32 {
        read_decimal(&e)
    }

    fn name(e: Env) -> String {
        read_name(&e)
    }

    fn symbol(e: Env) -> String {
        read_symbol(&e)
    }
}
