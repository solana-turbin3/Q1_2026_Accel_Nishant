use pinocchio::{
    entrypoint,
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
};
pub mod state;
pub mod instructions;

entrypoint!(process_instruction);

use pinocchio_pubkey::declare_id;
declare_id!("4xMZozu4pZ1xEVW6giiRoRmtMWi1YGL8MsXBLaztwx5h");


fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {

    let (ix_disc, _ix_data) = instruction_data
    .split_first()
    .ok_or(ProgramError::InvalidInstructionData)?;


    match ix_disc {
        0 =>  todo!(),
        _ =>  Err(ProgramError::InvalidInstructionData)
    }
}
