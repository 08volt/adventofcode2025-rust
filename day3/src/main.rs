
use std::collections::HashMap;


struct BatteryBank {
    bank: Vec<u16>,
    counter: HashMap<u16, u16>,
}

impl From<&str> for BatteryBank {
    fn from( input: &str ) -> Self {
        let bank = input.chars()
            .map( |line| line.to_digit(10).unwrap() as u16 )
            .collect::<Vec<u16>>();

        let mut counter = HashMap::new();
        for &value in &bank {
            *counter.entry(value).or_insert(0) += 1;
        }

        BatteryBank { bank, counter }
    }
}

impl BatteryBank {

    fn max_couple(&self) -> u16 {
        let m1 = self.bank[..self.bank.len()-1].iter().max().unwrap();
        let i1 = self.bank.iter().position( |x| x == m1 ).unwrap();
        let m2 = self.bank[i1 + 1..].iter().max().unwrap();
        let s1 = m1.to_string() + &m2.to_string();
        s1.parse::<u16>().unwrap()
    }


    fn max_N(&self, n:usize) -> u64 {
        
        let mut result: Vec<u16> = Vec::new();
        let mut last_index: isize = -1;

        for i in 0..n {
            let mut lookup_slice = self.bank[(last_index+1) as usize..self.bank.len()-(n-i -1)].iter();
            let m1 = lookup_slice.clone().max().unwrap();
            last_index = lookup_slice.clone().position( |x| x == m1 ).unwrap() as isize + 1 + last_index;
            result.push( *m1 );
        }

        let s1 = result.iter().map( |x| x.to_string() ).collect::<String>();

        s1.parse::<u64>().unwrap()
    }
    

}

fn main() {
    let input = read_txt_file( "input.txt" );
    let batteries = input.lines().map(|line| BatteryBank::from(line) ).collect::<Vec<BatteryBank>>();
    let result = batteries.iter().map( |bank| bank.max_couple() ).sum::<u16>();
    let result_n = batteries.iter().map( |bank| bank.max_N(12) ).sum::<u64>();

    println!( "The sum of the maximum couples is: {}", result );
    println!( "The sum of the maximum 12-digit numbers is: {}", result_n );

}

fn read_txt_file( path : &str ) -> String {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open( path )
        .expect( &format!( "Could not open file: {}", path ) );
    let mut contents = String::new();
    file.read_to_string( &mut contents ).unwrap();
    contents
}
