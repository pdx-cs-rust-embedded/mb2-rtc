#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use microbit::{board::Board, hal::rtc};

struct Ticker<T> {
    rtc: rtc::Rtc<T>,
    last_time: u32,
}

impl<T: rtc::Instance> Ticker<T> {
    fn new(rtc: T) -> Result<Self, rtc::Error> {
        let rtc = rtc::Rtc::new(rtc, 33)?;
        rtc.enable_counter();
        let last_time = rtc.get_counter();
        Ok(Ticker { rtc, last_time })
    }

    fn wait_for_tick(&mut self) {
        let mut now = self.rtc.get_counter();
        // XXX Empirical constant. Math says 993.
        while now - self.last_time < 970 {
            now = self.rtc.get_counter();
        }
        self.last_time = now;
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut ticker = Ticker::new(board.RTC0).unwrap();
    for sec in 0u64.. {
        rprintln!("{}", sec);
        ticker.wait_for_tick();
    }
    panic!("too many seconds");
}
