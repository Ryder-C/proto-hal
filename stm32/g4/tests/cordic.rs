#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

#[defmt_test::tests]
mod tests {
    use defmt::assert_eq;
    use fixed::types::I1F31;
    use g4::common::{cordic, rcc};

    #[test]
    fn sqrt() {
        let rcc: rcc::Reset = unsafe { core::mem::transmute(()) };
        let cordic: cordic::Reset = unsafe { core::mem::transmute(()) };

        let cordicen = rcc
            .ahb1enr
            .build_transition()
            .cordicen::<rcc::ahb1enr::cordicen::Enabled>()
            .finish()
            .cordicen;

        cortex_m::asm::delay(1);

        let cordic = cordic.attach(cordicen.into()).csr(|reg| {
            reg.build_transition()
                .func::<cordic::csr::func::Sqrt>()
                .finish()
        });

        cordic
            .wdata
            .write(|w| w.arg(I1F31::from_num(0.25).to_bits() as _));
        assert_eq!(
            I1F31::from_bits(cordic.rdata.read(|r| r.res()) as _).to_num::<f32>(),
            0.4999994
        );
    }
}
