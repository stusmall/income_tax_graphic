use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::fs::File;
use std::io::BufReader;

use csv::ReaderBuilder;


use inflation::InflationCalculator;
use dollars::Dollars;




#[derive(Debug)]
pub struct TaxBracket {
    top: Option<u32>,
    bottom: u32,
    rate: f32
}


#[derive(Debug,Deserialize)]
struct CSVRecord{
    year: u32,
    rate: f32,
    bottom: u32,
    top: Option<u32>
}





pub struct TaxCode{
    inflation_calc: InflationCalculator,
    brackets_by_year: BTreeMap<u32, Vec<TaxBracket>>
}

impl TaxCode{
    pub fn new(inflation_calc: InflationCalculator) -> Self{
        let file = File::open("data/headofhousehold.csv").unwrap();
        let mut buf_reader = BufReader::new(file);


        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(buf_reader);


        let records: BTreeMap<u32,Vec<TaxBracket>> = rdr.deserialize()
            .fold(BTreeMap::new(), |mut collection,parsed|{
                if let Ok(x) = parsed {
                    let y :CSVRecord = x;
                    let tax_bracket = TaxBracket{
                        top: y.top,
                        bottom: y.bottom,
                        rate: y.rate * 0.01
                    };

                    match collection.entry(y.year){
                        Entry::Occupied(mut entry) => {
                            let mut vec = entry.get_mut();
                            match vec.binary_search_by(|probe| probe.bottom.cmp(&tax_bracket.bottom)){
                                Ok(_) => {
                                    error!("Duplicate entry for {:?}", tax_bracket);
                                }
                                Err(index) => {
                                    vec.insert(index, tax_bracket);
                                }
                            }
                        },
                        Entry::Vacant(x) => {
                            x.insert(vec![tax_bracket]);
                        }
                    }
                }else{
                    error!("Failed to parse: {:?}", parsed);
                }
                collection
            });
        //println!("We have {:#?}", records);
        TaxCode{
            inflation_calc,
            brackets_by_year: records
        }

    }

    pub fn calculate_tax_rate_over_the_years(&self, current_day_income: &Dollars) -> Vec<(u32,f32)> {

        self.brackets_by_year.iter().map(|entry|{
            let year = entry.0;
            let brackets = entry.1;
            let adjusted_income = self.inflation_calc.adjust_for_inflation(current_day_income, year.clone());
            let tax_bill = calculate_tax(&adjusted_income, brackets);
            let actual_tax_rate = tax_bill.cents as f32 / adjusted_income.cents as f32;
            println!("In {}, someone earning {:?} paid a rate of {:.2?}", year, adjusted_income, actual_tax_rate);
            (year.clone(), actual_tax_rate)
        }).collect()
    }
}


fn calculate_tax(income: &Dollars, brackets: &Vec<TaxBracket>) -> Dollars {
    let mut income_left = income.clone();
    let mut tax_so_far = Dollars::new(0, 0, income.year);
    for bracket in brackets {
        if let Some(bracket_top) = bracket.top {
            let bracket_top = Dollars::new(bracket_top, 0, income.year);
            if income_left.cents > bracket_top.cents {
                tax_so_far = tax_so_far + Dollars::new(0, (bracket_top.cents as f32 * bracket.rate) as u32, income.year );
                income_left = income_left - bracket_top;
            } else {
                return tax_so_far + Dollars::new(0,(income_left.cents as f32 * bracket.rate) as u32, income.year);
            }
        } else {
            return tax_so_far + Dollars::new(0,(income_left.cents as f32 * bracket.rate) as u32, income.year);
        }
    }
    tax_so_far
}