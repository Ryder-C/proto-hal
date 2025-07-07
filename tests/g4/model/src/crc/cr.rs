pub mod polysize;
pub mod reset;
pub mod rev_in;
pub mod rev_out;

use proto_hal_build::ir::structures::register::Register;

pub fn generate() -> Register {
    Register::new(
        "cr",
        8,
        [
            reset::generate(),
            polysize::generate(),
            rev_in::generate(),
            rev_out::generate(),
        ],
    )
}
