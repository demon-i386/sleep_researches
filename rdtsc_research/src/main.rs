use std::time::Duration;
use std::time::Instant;
use std::mem;

#[derive(Debug)]
struct TimeSpec {
    tv_sec: u64,
    tv_nsec: u64,
}



fn main() {


    unsafe {
        let mut hit = false;
        let mut time_in_future = 0;

        while true{
            let duration = TimeSpec {
                tv_sec: 100000,
                tv_nsec: 1338,
            };
            let instant: Instant;
    
            let duration_bytes: Instant = std::mem::transmute(duration);
            instant = std::mem::transmute(duration_bytes);

            let instant2 = Instant::now(); // get RDTSC (loop)
            

            let instant_bytes: [u8; mem::size_of::<Instant>()] = mem::transmute(instant2);
            let ptr = instant_bytes.as_ptr();
            let tv_sec_ptr = ptr as *const u64;

            if hit == false{
                hit = true;
                time_in_future = *tv_sec_ptr + 20; // previous captured RDTSC + 20 (seconds)
            }

            println!("Instant: {:?} - expected: {:?}", *tv_sec_ptr, time_in_future);
            if (*tv_sec_ptr) > time_in_future{
                println!("timeout...");
                break
            }

        }


    }


    // println!("{:?} {:?}", start, timeout);
}
