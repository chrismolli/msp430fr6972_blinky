#![no_main]
#![no_std]
#![feature(abi_msp430_interrupt)]

extern crate panic_msp430;

use msp430::asm;
use msp430_rt::entry;
use msp430fr6972::interrupt;

/*
    Subroutines
*/
fn delay(n: u16) {
    let mut i = 0;
    loop {
        asm::nop();

        i += 1;

        if i == n {
            break;
        }
    }
}

/*
    Main Function
*/
#[entry]
fn main() -> ! {
    /*
        Take peripheral singleton instance
    */
    let p = msp430fr6972::Peripherals::take().unwrap();

    /*
        Disable watchdog timer
    */
    let wd = p.WATCHDOG_TIMER;
    wd.wdtctl.write(|w| {
        unsafe { w.bits(0x5A00) } // password
        .wdthold().set_bit()
    });

    /*
        Set Output Direction
        which is located at P9-6
    */
    let p9 = p.PORT_9;
    p9.p9dir.modify( |_,w| {unsafe {w.bits(0x40)}} );

    /*
        Enable Debug LED Pin
    */
    p9.p9out.modify( |_,w| {unsafe {w.bits(0x40)}} );

    loop {
        /*
            Toggle Debug LED
        */
        p9.p9out.modify( |_,w| {unsafe {w.bits(0x40)}} );
        delay(50000);
        delay(50000);
        p9.p9out.modify( |_,w| {unsafe {w.bits(0x00)}} );
        delay(50000);
        delay(50000);
    }
}


#[interrupt]
fn TIMER0_A0() {

}

#[no_mangle]
extern "C" fn abort() -> ! {
    panic!();
}
