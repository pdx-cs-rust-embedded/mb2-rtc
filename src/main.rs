#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use microbit::{board::Board, hal::rtc};

struct Ticker<T>(rtc::Rtc<T>);

impl<T: rtc::Instance> Ticker<T> {
    fn new(rtc: T) -> Result<Self, rtc::Error> {
        let rtc = rtc::Rtc::new(rtc, 33)?;
        rtc.disable_counter();
        Ok(Ticker(rtc))
    }

    fn wait_one_second(&mut self) {
        self.0.clear_counter();
        // XXX Must wait for counter to actually clear.
        while self.0.get_counter() != 0 {
            // spin
        }
        self.0.enable_counter();
        // XXX Empirical constant. Math says 993.
        while self.0.get_counter() < 970 {
            // spin
        }
        self.0.disable_counter();
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut ticker = Ticker::new(board.RTC0).unwrap();
    for sec in 0u64.. {
        rprintln!("{}", sec);
        ticker.wait_one_second();
    }
    panic!("too many seconds");
}
