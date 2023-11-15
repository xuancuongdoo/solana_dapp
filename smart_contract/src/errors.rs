use anchor_lang::prelude::*;

#[error_code]
pub enum TodoError{
    #[msg("You are not authorized to perform this action.")]
    Unauthorize,
    #[msg("Not Allowed")]
    NotAllowed,
    #[msg("Math Operation Overflow")]
    MathOverFlow,
    #[msg("Already Marked")]
    AlreadyMarked
}