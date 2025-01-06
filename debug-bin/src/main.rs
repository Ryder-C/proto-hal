#![no_std]
#![no_main]

use cortex_m_rt::entry;
use fixed::types::I1F31;
use {defmt_rtt as _, panic_probe as _};

use cortex_m_spa::nvic;
use defmt::info;
use g4::{
    common::{cordic, rcc},
    interrupt,
};

#[interrupt]
fn CORDIC() {
    info!("hiii");
}

#[entry]
unsafe fn main() -> ! {
    let rcc = unsafe { rcc::Reset::conjure() };
    let cordic = unsafe { cordic::Reset::conjure() };
    let nvic = unsafe { nvic::Reset::conjure() };

    let cordicen = rcc
        .ahb1enr
        .build_state()
        .cordicen()
        .enabled()
        .finish()
        .cordicen;

    cortex_m::asm::delay(1);

    let cordic = cordic
        .attach(cordicen.into())
        .csr(|state| state.func().sqrt().ien().enabled());

    info!("hello!");

    let _nvic = nvic.iser3(|state| state.setena100().enabled());

    cordic
        .wdata
        .write(|w| w.arg(I1F31::from_num(0.25).to_bits() as _));
    assert_eq!(
        I1F31::from_bits(cordic.rdata.read(|r| r.res()) as _).to_num::<f32>(),
        0.4999994
    );

    loop {}
}
