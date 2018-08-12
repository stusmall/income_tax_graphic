use std::collections::HashMap;

use dollars::Dollars;

pub struct InflationCalculator {
    lookup: HashMap<u32, f32>,
}

impl InflationCalculator {
    pub fn new() -> InflationCalculator {
        let mut lookup = HashMap::new();
        lookup.insert(1913, 9.9);
        lookup.insert(1914, 10.0);
        lookup.insert(1915, 10.1);
        lookup.insert(1916, 10.9);
        lookup.insert(1917, 12.8);
        lookup.insert(1918, 15.0);
        lookup.insert(1919, 17.3);
        lookup.insert(1920, 20.0);
        lookup.insert(1921, 17.9);
        lookup.insert(1922, 16.8);
        lookup.insert(1923, 17.1);
        lookup.insert(1924, 17.1);
        lookup.insert(1925, 17.5);
        lookup.insert(1926, 17.7);
        lookup.insert(1927, 17.4);
        lookup.insert(1928, 17.2);
        lookup.insert(1929, 17.2);
        lookup.insert(1930, 16.7);
        lookup.insert(1931, 15.2);
        lookup.insert(1932, 13.6);
        lookup.insert(1933, 12.9);
        lookup.insert(1934, 13.4);
        lookup.insert(1935, 13.7);
        lookup.insert(1936, 13.9);
        lookup.insert(1937, 14.4);
        lookup.insert(1938, 14.1);
        lookup.insert(1939, 13.9);
        lookup.insert(1940, 14.0);
        lookup.insert(1941, 14.7);
        lookup.insert(1942, 16.3);
        lookup.insert(1943, 17.3);
        lookup.insert(1944, 17.6);
        lookup.insert(1945, 18.0);
        lookup.insert(1946, 19.5);
        lookup.insert(1947, 22.3);
        lookup.insert(1948, 24.0);
        lookup.insert(1949, 23.8);
        lookup.insert(1950, 24.1);
        lookup.insert(1951, 26.0);
        lookup.insert(1952, 26.6);
        lookup.insert(1953, 26.8);
        lookup.insert(1954, 26.9);
        lookup.insert(1955, 26.8);
        lookup.insert(1956, 27.2);
        lookup.insert(1957, 28.1);
        lookup.insert(1958, 28.9);
        lookup.insert(1959, 29.2);
        lookup.insert(1960, 29.6);
        lookup.insert(1961, 29.9);
        lookup.insert(1962, 30.3);
        lookup.insert(1963, 30.6);
        lookup.insert(1964, 31.0);
        lookup.insert(1965, 31.5);
        lookup.insert(1966, 32.5);
        lookup.insert(1967, 33.4);
        lookup.insert(1968, 34.8);
        lookup.insert(1969, 36.7);
        lookup.insert(1970, 38.8);
        lookup.insert(1971, 40.5);
        lookup.insert(1972, 41.8);
        lookup.insert(1973, 44.4);
        lookup.insert(1974, 49.3);
        lookup.insert(1975, 53.8);
        lookup.insert(1976, 56.9);
        lookup.insert(1977, 60.6);
        lookup.insert(1978, 65.2);
        lookup.insert(1979, 72.6);
        lookup.insert(1980, 82.4);
        lookup.insert(1981, 90.9);
        lookup.insert(1982, 96.5);
        lookup.insert(1983, 99.6);
        lookup.insert(1984, 103.9);
        lookup.insert(1985, 107.6);
        lookup.insert(1986, 109.6);
        lookup.insert(1987, 113.6);
        lookup.insert(1988, 118.3);
        lookup.insert(1989, 124.0);
        lookup.insert(1990, 130.7);
        lookup.insert(1991, 136.2);
        lookup.insert(1992, 140.3);
        lookup.insert(1993, 144.5);
        lookup.insert(1994, 148.2);
        lookup.insert(1995, 152.4);
        lookup.insert(1996, 156.9);
        lookup.insert(1997, 160.5);
        lookup.insert(1998, 163.0);
        lookup.insert(1999, 166.6);
        lookup.insert(2000, 172.2);
        lookup.insert(2001, 177.1);
        lookup.insert(2002, 179.9);
        lookup.insert(2003, 184.0);
        lookup.insert(2004, 188.9);
        lookup.insert(2005, 195.3);
        lookup.insert(2006, 201.6);
        lookup.insert(2007, 207.3);
        lookup.insert(2008, 215.3);
        lookup.insert(2009, 214.5);
        lookup.insert(2010, 218.1);
        lookup.insert(2011, 224.9);
        lookup.insert(2012, 229.6);
        lookup.insert(2013, 233.0);
        lookup.insert(2014, 236.7);
        lookup.insert(2015, 237.0);
        lookup.insert(2016, 240.0);
        lookup.insert(2017, 245.1);
        lookup.insert(2018, 250.5);
        InflationCalculator { lookup }
    }

    pub fn adjust_for_inflation(&self, amount: &Dollars, to: u32) -> Dollars {
        let ratio = self.lookup.get(&to).unwrap() / self.lookup.get(&amount.year).unwrap();
        Dollars {
            cents: (ratio * amount.cents as f32) as u64,
            year: to,
        }
    }
}

#[test]
fn test_inflation_conversion() {
    let i = InflationCalculator::new();
    let d = Dollars::new(100, 00, 1984);
    assert_eq!(
        i.adjust_for_inflation(&d, 2016),
        Dollars::new(230, 99, 2016)
    );
}
