

fn read_txt_file( path : &str ) -> String {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open( path )
        .expect( &format!( "Could not open file: {}", path ) );
    let mut contents = String::new();
    file.read_to_string( &mut contents ).unwrap();
    contents
}

fn check_sequence( pattern: &str, checked_string: &str ) -> bool {
    if checked_string.len() < pattern.len() {
        return false;
    }

    if checked_string.len() % pattern.len() != 0 {
        return false;
    }

    if pattern == checked_string {
        return true;
    }

    if pattern == &checked_string[..pattern.len()] {
        return check_sequence(pattern, checked_string[pattern.len()..].as_ref());
    }
    return false;
}

fn is_invalid_number(num: usize) -> bool {
    let num_str = num.to_string();

    for i in 1..=(num_str.len() - 1) {
        let pattern = &num_str[..i];
        if check_sequence(pattern, &num_str) {
            return true;
        }
    }
    return false;
}

struct Sequence {
    start: usize,
    end: usize,
    // l_min: usize,
    // l_max: usize,
    // r_min: usize,
    // r_max: usize,
}

impl From<&str> for Sequence {
    fn from( s: &str ) -> Self {
        let parts: Vec<&str> = s.split( '-' ).collect();

        let mut part_0 = parts[0];
        let mut part_1 = parts[1];
        let repeated_nines = "9".repeat(part_0.len());
        let new_part0 = "1".to_string() + &"0".repeat(part_1.len() - 1);

        // if part_1.len() > part_0.len() {
        //     if part_0.len() % 2 == 0 {
        //         // smaller part1
        //         part_1 = &repeated_nines;
        //     } else {
        //         // higher part0
        //         part_0 = &new_part0;
        //     }
        // }

        print!("Parsing sequence: {} -> ", s );
        let start = part_0.parse::<usize>().unwrap();
        let end = part_1.parse::<usize>().unwrap();
        
        // let (l_min, r_min) = part_0.split_at(part_0.len() / 2);
        // let end_str = end.to_string();
        // let (l_max, r_max) = end_str.split_at(end_str.len() / 2);
        // println!("Seq: {}-{}, L: {}-{}, R: {}-{}", start, end, l_min, l_max, r_min, r_max);  
        Sequence {
            start,
            end,
            // l_min: l_min.parse::<usize>().unwrap(),
            // r_min: r_min.parse::<usize>().unwrap(),
            // l_max: l_max.parse::<usize>().unwrap(),
            // r_max: r_max.parse::<usize>().unwrap(),
        }
    }
}

impl Sequence {
    // fn invalid_ids( &self ) -> Vec<usize> {
    //     let mut invalid_ids: Vec<usize> = Vec::new();

    //     println!("{}-{}: ", self.start, self.end);

    //     if self.start.to_string().len() % 2 == 1 {
    //         println!("Odd length IDs, no invalid IDs.");
    //         return invalid_ids;
    //     }
        

    //     if self.l_max == self.l_min {

    //         if self.l_min <= self.r_max && self.l_min >= self.r_min {
    //             invalid_ids.push((self.l_min.to_string() + &self.l_min.to_string()).parse::<usize>().unwrap());
    //         }

    //         println!("Single L value, invalid IDs: {:?}", invalid_ids);
    //         return invalid_ids;
    //     }

    //     for l in (self.l_min + 1)..self.l_max {
    //         invalid_ids.push((l.to_string() + &l.to_string()).parse::<usize>().unwrap());
    //     }

    //     if self.l_min >= self.r_min {
    //         invalid_ids.push((self.l_min.to_string() + &self.l_min.to_string()).parse::<usize>().unwrap());
    //     }

    //     if self.l_max <= self.r_max {
    //         invalid_ids.push((self.l_max.to_string() + &self.l_max.to_string()).parse::<usize>().unwrap());
    //     }

    //     println!("Invalid IDs found: {:?}", invalid_ids);

    //     invalid_ids
    // }
    

    // Only sequences 
    fn broad_invalid_ids( &self ) -> Vec<usize> {

        let mut invalid_ids: Vec<usize> = Vec::new();

        println!("{}-{}: ", self.start, self.end);


        for num in self.start..=self.end + 1 {
            if is_invalid_number( num ) {
                println!( "Invalid number found: {}", num );
                invalid_ids.push( num );
            }
        }
        
        invalid_ids
    }
}

fn main() {
    let file_path = "input.txt";
    let file_contents = read_txt_file( file_path );

    let sequences_str = file_contents.split(',').collect::<Vec<&str>>();
    let sequences: Vec<Sequence> = sequences_str.iter().map( |s| Sequence::from( *s ) ).collect();

    // let invalid_ids_sum: usize = sequences.iter()
    //     .map( |seq| seq.invalid_ids() )
    //     .flatten()
    //     .sum();
    // println!( "Total sum: {}", invalid_ids_sum );

    let broad_invalid_ids_sum: usize = sequences.iter()
        .map( |seq| seq.broad_invalid_ids() )
        .flatten()
        .sum();
    println!( "Total broad sum: {}", broad_invalid_ids_sum );
}
