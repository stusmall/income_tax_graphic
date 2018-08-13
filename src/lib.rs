extern crate csv;
#[macro_use]
extern crate log;
extern crate png;
extern crate rayon;
#[macro_use]
extern crate serde_derive;

mod dollars;
mod inflation;
mod tax_brackets;

use std::fs::File;
use std::io::BufWriter;

use png::HasParameters;
use rayon::prelude::*;

use dollars::Dollars;

pub fn run(){
    let inflation = inflation::InflationCalculator::new();
    let tax_code = tax_brackets::TaxCode::new(inflation);

    //range 10,000 to 20,000,000 in 2013 dollars.  $10k steps
    let range: Vec<Dollars> = (1..2_000)
        .map(|value| {
            Dollars::new(value * 10_000, 0, 2018)
        })
        .collect::<Vec<Dollars>>();

    let mut result:Vec<(Dollars,Vec<(u32,f32)>)> = range
        .par_iter()
        .map(|income|{
            //This is one stripe of the y axis
            //tax_code
            (income.to_owned(), tax_code.calculate_tax_rate_over_the_years(income))
        }).collect();

    result.reverse();

    //write_csv(&result);
    write_png(&result);

}

//In the form of income measure in 2018 buying power -> tax rate for each year
fn write_csv(info: &Vec<(Dollars,Vec<(u32,f32)>)>){
    let file = File::create("/tmp/out.csv").unwrap();
    let writer = BufWriter::new(file);

    let mut wtr = csv::Writer::from_writer(writer);

    #[derive(Debug,Serialize)]
    struct OutputRow {
        income: u64,
        year: u32,
        rate: f32
    }

    for entry in info {
        let income = entry.0.cents / 100;
        for internal_entry in &entry.1 {
            let _ = wtr.serialize(OutputRow{
                income,
                year: internal_entry.0,
                rate: internal_entry.1
            });
        }
    }
}

fn write_png(info: &Vec<(Dollars,Vec<(u32,f32)>)>){
    let file = File::create("/tmp/out.png").unwrap();
    let ref mut w = BufWriter::new(file);
    //let width = 1925 - 1913 + 1;
    let width = info.get(0).unwrap().1.len() as u32;
    let height = info.len() as u32;
    let year_width = 10 as u32;
    let mut encoder = png::Encoder::new(w, width * year_width, height);
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let mut buffer = Vec::new();

    for entry in info {
        for internal_entry in &entry.1 {
            let crossover = 0.5;
            let r:u8 = if internal_entry.1 > crossover {
                //(255.0 * (1.0-(internal_entry.1*2.0))) as u8
                (((internal_entry.1 - crossover) / (1.0 - crossover))* 255.0) as u8
            } else {
                0
            };
            //let b:u8 = 0; //(255.0 * (1.0-internal_entry.1)) as u8;
            let b:u8 = if internal_entry.1 < crossover {
                //(255.0 * (1.0-(internal_entry.1*2.0))) as u8
                (((crossover - internal_entry.1) / crossover) * 255.0) as u8
            } else {
                0
            };
            //println!("rate {}, b {}, r {}", internal_entry.1, b, r);
            for _ in  0 .. year_width {
                let mut data = vec![r, 0, b];
                buffer.append(&mut data);
            }

        }
    };
    writer.write_image_data(&buffer).unwrap(); // Save

}

