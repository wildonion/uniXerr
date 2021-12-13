







#![cfg(not(feature = "no-entrypoint"))] //-- inner attribute is the only way to place attributes on the crate by writing them in the root of it - this crate only gets compiled if the feature is not no-entrypoint
#[cfg(all(feature = "bloody-panic", target_arch = "bpf"))] //-- outter attribute only on bpf target and the can be on enum and struct fields
#[no_mangle]
fn custom_panic(info: &core::panic::PanicInfo<'_>) {
    solana_program::msg!("program paniced!!!");
    solana_program::msg!("{}", info);
}







use solana_program::{
    account_info::AccountInfo, 
    entrypoint::ProgramResult, 
    pubkey::Pubkey,
    entrypoint, 
};







// =======================================
//  ....... SMART CONTRACT ENTRYPOINTS
// =======================================
entrypoint!(contract_program); //-- the entrypoint of bpf loader for each instruction to compile the program defined instruction into elf shared object (.so) which contains the bpf bytecode - every program has its own loader and entrypoint which the program was deployed with - each loader provides a helper function that deserializes the program's input parameters into Rust types, the entrypoint macros automatically calls the deserialization helper








// ===========================================
//  ....... SMART CONTRACT PROCESSORS LOADING
// ===========================================
fn contract_program(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    crate::processor::contract_program(program_id, accounts, instruction_data) //-- it'll return a program to pass it through the entrypoint of the bpf loader
}