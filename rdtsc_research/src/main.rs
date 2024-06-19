use std::time::Duration;
use std::time::Instant;
use std::mem;

fn main() {
    unsafe {
        let mut hit = false;
        let mut time_in_future = 0;

        while true{

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
}
