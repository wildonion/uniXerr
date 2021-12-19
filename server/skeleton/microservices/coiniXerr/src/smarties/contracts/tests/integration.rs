






#![cfg(feature="test-bpf")] //-- inner attribute - this crate only gets compiled if the feature is test-bpf


pub mod lib;
use lib::entrypoints::contract_program;
use {
    std::mem,
    super::*,
    assert_matches::*,
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    solana_program_test::*,
    solana_program::clock::Epoch,
    solana_sdk::{signature::Signer, transaction::Transaction},
    solana_validator::test_validator::*,
};








// https://docs.solana.com/developing/test-validator
#[tokio::test]
async fn test_validator_transaction() {
    let program_id = Pubkey::new_unique();

    let (test_validator, payer) = TestValidatorGenesis::default().add_program("some token contracts", program_id).start();
    let (rpc_client, recent_blockhash, _fee_calculator) = test_validator.rpc_client();

    let mut transaction = Transaction::new_with_payer(
        &[Instruction { //-- every transaction contains one or more instructions or programs and every instruction needs to be passed through the entrypoint of the bpf loader to compile into bpf bytecode - a transaction contains an instruction wich contains program_id, accounts and instruction_data means for executing a program we need to send a transaction
            program_id,
            accounts: vec![AccountMeta::new(payer.pubkey(), false)],
            data: vec![1, 2, 3],
        }],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    assert_matches!(rpc_client.send_and_confirm_transaction(&transaction), Ok(_)); //-- the program execution begins with a transaction in which one or more instructions inside the transaction will be compiled to bpf bytecode using their own entrypoint loader
}








#[tokio::test] //-- tokio test for async functions
async fn test_transaction() {
    let program_id = Pubkey::new_unique();

    let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
        "some token testing",
        program_id,
        processor!(contract_program),
    )
    .start()
    .await;

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![AccountMeta::new(payer.pubkey(), false)],
            data: vec![1, 2, 3],
        }],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
}








#[tokio::test] //-- tokio test for async functions
async fn test_contract_program_instruction(){
    
    let key = Pubkey::default();
    let mut lamports = 0;
    let mut data = vec![0; mem::size_of::<u32>()];
    let owner = Pubkey::default();
    let account = AccountInfo::new( //-- an account info to pass through the contract_program() function to compile using the entrypoint bpf loader
        &key,
        false,
        true, //-- this account is writable
        &mut lamports,
        &mut data,
        &owner,
        false,
        Epoch::default(),
    );
    let program_id = Pubkey::default();
    let instruction_data: Vec<u8> = Vec::new();
    let accounts = vec![account];


    assert_eq!(
        Contract::try_from_slice(&accounts[0].data.borrow()).unwrap().sign, 
        0
    );
    
    contract_program(&program_id, accounts, instruction_data); //-- contract sign field will be increased every time we call the contract program 
    assert_eq!(
        Contract::try_from_slice(&accounts[0].data.borrow()).unwrap().sign, 
        1
    );
    
    
    contract_program(&program_id, accounts, instruction_data); //-- contract sign field will be increased every time we call the contract program
    assert_eq!(
        Contract::try_from_slice(&accounts[0].data.borrow()).unwrap().sign, 
        2
    );

}
