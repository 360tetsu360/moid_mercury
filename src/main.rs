use std::fs::File;
use std::io::Read;
use std::os::raw::c_double;

use csv::Writer;

extern "C" {
    fn moid_(
        saxisA: *mut c_double,
        eccenA: *mut c_double,
        argpeA: *mut c_double,
        omegaA: *mut c_double,
        incliA: *mut c_double,
        saxisB: *mut c_double,
        eccenB: *mut c_double,
        argpeB: *mut c_double,
        omegaB: *mut c_double,
        incliB: *mut c_double,
    ) -> c_double;
}

fn main() {
    let mut mercury_a: f64 = 0.3870983098;
    let mut mercury_e: f64 = 0.2056;
    let mut mercury_peri: f64 = 77.4561;
    let mut mercury_node: f64 = 48.3309;
    let mut mercury_incl: f64 = 7.0050;

    let url = "https://minorplanetcenter.net/iau/Ephemerides/Comets/Soft03Cmt.txt";
    let mut response = reqwest::blocking::get(url).unwrap();

    if response.status() == reqwest::StatusCode::OK {
        let mut buffer = String::new();
        response.read_to_string(&mut buffer).unwrap();

        let mut comet_moids = vec![];

        for line in buffer.lines() {
            if line.starts_with('#') {
                continue;
            }

            let comet: Vec<&str> = line.split(',').collect();

            if comet.len() < 2 {
                continue;
            }

            if comet[1] != "e" {
                continue;
            }

            let comet_name = comet[0];
            let mut comet_a: f64 = comet[5].parse::<f64>().unwrap();
            let mut comet_e: f64 = comet[7].parse::<f64>().unwrap();
            let mut comet_peri: f64 = comet[4].parse::<f64>().unwrap();
            let mut comet_node: f64 = comet[3].parse::<f64>().unwrap();
            let mut comet_incl: f64 = comet[2].parse::<f64>().unwrap();

            let result = unsafe {
                moid_(
                    &mut mercury_a,
                    &mut mercury_e,
                    &mut mercury_peri,
                    &mut mercury_node,
                    &mut mercury_incl,
                    &mut comet_a,
                    &mut comet_e,
                    &mut comet_peri,
                    &mut comet_node,
                    &mut comet_incl,
                )
            };

            comet_moids.push((comet_name.to_owned(), result));
            println!("Comet name: {}, MOID: {}", comet_name, result);
        }
        comet_moids.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let file = File::create("mercury_comet_moid.csv").unwrap();
        let mut writer = Writer::from_writer(file);

        writer.write_record(["Name", "MOID(AU)"]).unwrap();

        for (name, moid) in comet_moids {
            writer.write_record(&[name, moid.to_string()]).unwrap();
        }

        writer.flush().unwrap();
    } else {
        eprintln!("HTTP error: {}", response.status());
    }
}
