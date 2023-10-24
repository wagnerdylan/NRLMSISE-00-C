#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct NRLMSISEInput {
    pub year: i32,
    pub doy: i32,
    pub sec: f64,
    pub alt: f64,
    pub g_lat: f64,
    pub g_long: f64,
    pub lst: f64,
    pub f107A: f64,
    pub f107: f64,
    pub ap: f64,
    pub ap_a: [f64; 7usize],
}

pub struct NRLMSISEFlags {
    pub switches: [i32; 24usize],
    pub sw: [f64; 24usize],
    pub swc: [f64; 24usize],
}
#[derive(Debug)]
pub struct NRLMSISEOutput {
    pub TINF: f64,
    pub TG: f64,
    pub HE: f64,
    pub O: f64,
    pub N2: f64,
    pub O2: f64,
    pub AR: f64,
    pub H: f64,
    pub N: f64,
    pub ANM: f64,
    pub rho: f64,
}

pub fn gtd7_safe(input: &mut NRLMSISEInput, flags: &NRLMSISEFlags) -> NRLMSISEOutput {
    let mut input_c_form = nrlmsise_input {
        year: input.year,
        doy: input.doy,
        sec: input.sec,
        alt: input.alt,
        g_lat: input.g_lat,
        g_long: input.g_long,
        lst: input.lst,
        f107A: input.f107A,
        f107: input.f107,
        ap: input.ap,
        ap_a: &mut ap_array { a: input.ap_a } as *mut _,
    };

    let mut flags_c_form = nrlmsise_flags {
        switches: flags.switches,
        sw: flags.sw,
        swc: flags.swc,
    };

    let mut output_c_form = nrlmsise_output {
        d: [0f64; 9usize],
        t: [0f64; 2usize],
    };

    unsafe {
        gtd7(
            &mut input_c_form as *mut _,
            &mut flags_c_form as *mut _,
            &mut output_c_form as *mut _,
        );
    }

    NRLMSISEOutput {
        TINF: output_c_form.t[0],
        TG: output_c_form.t[1],
        HE: output_c_form.d[0],
        O: output_c_form.d[1],
        N2: output_c_form.d[2],
        O2: output_c_form.d[3],
        AR: output_c_form.d[4],
        H: output_c_form.d[6],
        N: output_c_form.d[7],
        ANM: output_c_form.d[8],
        rho: output_c_form.d[5],
    }
}

// TODO add more tests plus asserts
// TODO make flags interface more usable

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gtd7_01_unsafe() {
        let mut aph = ap_array {
            a: [0.0f64; 7usize],
        };

        let mut flags = nrlmsise_flags {
            switches: [1; 24usize],
            sw: [0f64; 24usize],
            swc: [0f64; 24usize],
        };

        flags.switches[0] = 0;

        let mut input = nrlmsise_input {
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

        unsafe {
            gtd7(
                &mut input as *mut _,
                &mut flags as *mut _,
                &mut output as *mut _,
            );
        }
    }

    #[test]
    fn gtd7_01_safe() {
        let mut flags = NRLMSISEFlags {
            switches: [1; 24usize],
            sw: [0f64; 24usize],
            swc: [0f64; 24usize],
        };

        flags.switches[0] = 0;

        let mut input = NRLMSISEInput {
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
            ap_a: [0f64; 7usize],
        };

        let output = gtd7_safe(&mut input, &flags);
    }
}
