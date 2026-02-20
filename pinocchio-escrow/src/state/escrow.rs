use pinocchio::{account_info::AccountInfo, pubkey::Pubkey};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Escrow {
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub maker: Pubkey,
    pub receive_amount: u64,
    pub bump: u8,
}

impl Escrow {
    pub const LEN: usize = 32 + 32 + 8 + 1;

    pub fn from_account_info(account_info: &AccountInfo) -> &mut Self {
        assert_eq!(account_info.data_len(), Self::LEN);
        assert_eq!(unsafe { account_info.owner() }, &crate::id());

        unsafe { &mut *(account_info.borrow_mut_data_unchecked().as_mut_ptr() as *mut Self) }
    }

      pub fn from_account_info_readable(account_info: &AccountInfo) -> &Self {
        assert_eq!(account_info.data_len(), Self::LEN);
        assert_eq!(unsafe { account_info.owner() }, &crate::id());

        unsafe { &*(account_info.borrow_data_unchecked().as_ptr() as *const Self) }
    }
}
