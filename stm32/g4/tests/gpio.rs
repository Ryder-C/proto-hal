#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

#[defmt_test::tests]
mod tests {
    use defmt::assert;
    use g4::common::{gpio::gpioa, rcc};

    #[test]
    fn output_input() {
        let rcc: rcc::Reset = unsafe { core::mem::transmute(()) };
        let gpioa: gpioa::Reset = unsafe { core::mem::transmute(()) };

        let gpioaen = rcc
            .ahb2enr
            .build_transition()
            .gpioaen::<rcc::ahb2enr::gpioaen::Enabled>()
            .finish()
            .gpioaen;

        let gpioa = gpioa.attach(gpioaen.into());

        let gpioa = gpioa.moder(|reg| {
            reg.build_transition()
                .mode5::<gpioa::moder::mode5::Output>()
                .finish()
        });

        gpioa.odr.write(|w| w.od5(true));

        cortex_m::asm::delay(1);

        assert!(gpioa.idr.read(|r| r.id5()));
        assert!(!gpioa.idr.read(|r| r.id4()));
    }
}
