extern crate csv;
#[macro_use]
extern crate log;
extern crate rayon;
#[macro_use]
extern crate serde_derive;

use rayon::prelude::*;

mod dollars;
mod inflation;
mod tax_brackets;

use dollars::Dollars;

pub fn run(){
    let inflation = inflation::InflationCalculator::new();
    let tax_code = tax_brackets::TaxCode::new(inflation);

    //range 10,000 to 10,000,000 in 2013 dollars.  $10k steps
    let range: Vec<Dollars> = (1..1000)
        .map(|value| {
            Dollars::new(value * 10_000, 0, 2018)
        })
        .collect::<Vec<Dollars>>();


    let result:Vec<(Dollars,Vec<(u32,f32)>)> = range
        .iter()
        .map(|income|{
            //This is one stripe of the y axis
            //tax_code
            println!("with income of {:?}", income);
            (income.to_owned(), tax_code.calculate_tax_rate_over_the_years(income))
        }).collect();

}