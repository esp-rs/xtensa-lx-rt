

/*
--------------------------------------------------------------------------------
User Exception (including Level 1 Interrupt from user mode).
--------------------------------------------------------------------------------
*/

    .begin      literal_prefix .UserExceptionVector
    .section    .UserExceptionVector.text, "ax"
    .global     _UserExceptionVector
    .type       _UserExceptionVector,@function
    .align      4

_UserExceptionVector:

    wsr     a0, EXCSAVE_1                   /* preserve a0 */
    call0   _rust_user_exc                    /* user exception handler */
    /* never returns here - call0 is used as a jump (see note at top) */

    .end        literal_prefix