#![no_std]
#![no_main]

use core::ptr::{read_volatile, write_volatile};

use cortex_m_rt::entry;
use fixed::types::I1F31;
use {defmt_rtt as _, panic_probe as _};

use defmt::info;
use g4::common::{cordic, gpio::gpioa, rcc};

#[no_mangle]
#[inline(never)]
fn configure_cordic(
    cordic: cordic::Block<
        cordic::csr::Reset,
        proto_hal::stasis::Entitlement<rcc::ahb1enr::cordicen::Enabled>,
    >,
) -> cordic::Block<
    cordic::csr::Register<
        cordic::csr::func::Sqrt,
        cordic::csr::precision::Reset,
        cordic::csr::scale::Reset,
        cordic::csr::ien::Reset,
        cordic::csr::dmaren::Reset,
        cordic::csr::dmawen::Reset,
        cordic::csr::nres::Reset,
        cordic::csr::nargs::Reset,
        cordic::csr::ressize::Reset,
        cordic::csr::argsize::Reset,
        cordic::csr::rrdy::Reset,
    >,
    proto_hal::stasis::Entitlement<rcc::ahb1enr::cordicen::Enabled>,
> {
    cordic.csr(|state| state.func().sqrt())
}

#[entry]
unsafe fn main() -> ! {
    info!("hi!");

    // write_volatile(0x4002_104c as *mut u32, 1);

    let rcc: rcc::Reset = unsafe { core::mem::transmute(()) };
    let gpioa: gpioa::Reset = unsafe { core::mem::transmute(()) };
    let cordic: cordic::Reset = unsafe { core::mem::transmute(()) };

    let gpioaen = rcc
        .ahb2enr
        .build_state()
        .gpioaen()
        .enabled()
        .finish()
        .gpioaen;

    let gpioa = gpioa.attach(gpioaen.into());

    let gpioa = gpioa.moder(|state| state.mode5().output());

    gpioa.odr.write(|w| w.od5(true));

    info!("moder: {:#032b}", read_volatile(0x4800_0000 as *const u32));
    info!("otyper: {:#032b}", read_volatile(0x4800_0004 as *const u32));
    info!(
        "ospeedr: {:#032b}",
        read_volatile(0x4800_0008 as *const u32)
    );
    info!("pupdr: {:#032b}", read_volatile(0x4800_000c as *const u32));
    info!("idr: {:#032b}", read_volatile(0x4800_0010 as *const u32));
    info!("odr: {:#032b}", read_volatile(0x4800_0014 as *const u32));

    assert!(gpioa.idr.read(|r| r.id5()));
    assert!(!gpioa.idr.read(|r| r.id4()));

    let cordicen = rcc
        .ahb1enr
        .build_state()
        .cordicen()
        .enabled()
        .finish()
        .cordicen;

    cortex_m::asm::delay(1);

    let cordic = configure_cordic(cordic.attach(cordicen.into()));

    info!("csr: {:#032b}", read_volatile(0x4002_0c00 as *const u32));

    cordic
        .wdata
        .write(|w| w.arg(I1F31::from_num(0.25).to_bits() as _));
    info!(
        "{}",
        I1F31::from_bits(cordic.rdata.read(|r| r.res()) as _).to_num::<f32>()
    );

    loop {}
}
