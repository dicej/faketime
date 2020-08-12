use fake_clock::FakeClock as FakeInstant;
use futures::future::{self, BoxFuture, FutureExt};
use std::{
    convert::TryInto,
    time::{Duration, Instant},
};

trait Time {
    fn elapsed(&self) -> Duration;
}

trait Clock {
    fn now(&self) -> Box<dyn Time>;
    fn delay_for(&self, duration: Duration) -> BoxFuture<'static, ()>;
}

struct FakeTime(FakeInstant);

impl Time for FakeTime {
    fn elapsed(&self) -> Duration {
        self.0.elapsed()
    }
}

struct FakeClock;

impl Clock for FakeClock {
    fn now(&self) -> Box<dyn Time> {
        Box::new(FakeTime(FakeInstant::now()))
    }

    fn delay_for(&self, duration: Duration) -> BoxFuture<'static, ()> {
        FakeInstant::advance_time(duration.as_millis().try_into().unwrap());
        future::ready(()).boxed()
    }
}

struct RealTime(Instant);

impl Time for RealTime {
    fn elapsed(&self) -> Duration {
        self.0.elapsed()
    }
}

struct RealClock;

impl Clock for RealClock {
    fn now(&self) -> Box<dyn Time> {
        Box::new(RealTime(Instant::now()))
    }

    fn delay_for(&self, duration: Duration) -> BoxFuture<'static, ()> {
        tokio::time::delay_for(duration).boxed()
    }
}

#[tokio::main]
async fn main() {
    println!("fake sleep for 5 seconds");
    let fake_clock = Box::new(FakeClock) as Box<dyn Clock>;
    let then = fake_clock.now();
    fake_clock.delay_for(Duration::from_secs(5)).await;
    println!("elapsed: {}ms", then.elapsed().as_millis());

    println!("real sleep for 5 seconds");
    let real_clock = Box::new(RealClock) as Box<dyn Clock>;
    let then = real_clock.now();
    real_clock.delay_for(Duration::from_secs(5)).await;
    println!("elapsed: {}ms", then.elapsed().as_millis());
}
