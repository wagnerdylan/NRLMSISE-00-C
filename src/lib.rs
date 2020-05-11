#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// TODO create safe interface
// TODO add more tests plus asserts

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gtd7_01() {
        let mut aph = ap_array{
            a: [0.0f64; 7usize]
        };

        let mut flags = nrlmsise_flags {
            switches: [1; 24usize],
            sw: [0f64; 24usize],
            swc: [0f64; 24usize],
        };

        flags.switches[0] = 0;

        let mut input = nrlmsise_input{
            year: 0,
            doy: 172,
            sec: 29000f64,
            alt: 400f64,
            g_lat: 60f64,
            g_long: -70f64,
            lst: 16f64,
            f107A: 150f64,
            f107: 150f64,
            ap: 4f64,
            ap_a: &mut aph as *mut _, 
        };

        let mut output = nrlmsise_output {
            d: [0f64; 9usize],
            t: [0f64; 2usize],
        };
        
        unsafe{ gtd7(&mut input as *mut _, &mut flags as *mut _, &mut output as *mut _); }

        println!("{:?}", output);
    }
}
