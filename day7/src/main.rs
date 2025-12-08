use std::collections::HashSet;
use std::collections::HashMap;




struct Puzzle {
    grid: HashSet<(i32, i32)>,
    height: i32,
    width: i32,
    start: (i32, i32),
}


impl From<&str> for Puzzle {
    fn from( input: &str ) -> Self {
        let mut grid = HashSet::new();

        let mut start = (0, 0);

        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '^' {
                    grid.insert( (x as i32, y as i32) );
                }
                if ch == 'S' {
                    start = (x as i32, y as i32);
                }
            }
        }

        Puzzle { grid, start, height: input.lines().count() as i32, width: input.lines().next().unwrap().len() as i32}
    }
}


impl Puzzle {
    fn display( &self ) {
        let max_x = *self.grid.iter().map( |(x, _)| x ).max().unwrap();
        let max_y = *self.grid.iter().map( |(_, y)| y ).max().unwrap();

        for y in 0..=max_y {
            for x in 0..=max_x {
                let ch = if self.grid.contains( &(x, y) ) { '^' } else { ' ' };
                print!( "{}", ch );
            }
            println!();
        }

        println!("START: {:?}", self.start);
    }

    fn count_splits( &self ) -> usize {
        let mut count: usize = 0;
        let mut current_beams: HashSet<i32> = vec![ self.start.0 ].into_iter().collect();


        for i in 0..(self.height-1) {
            let mut next_beams: HashSet<i32> = HashSet::new();
            while let Some( beam ) = current_beams.iter().next().cloned() {
                current_beams.remove( &beam );

                if !self.grid.contains(&(beam, i)) {
                    next_beams.insert( beam );
                    continue;
                }

                let below_left = beam - 1;
                let below_right = beam + 1;
                next_beams.insert( below_left );
                next_beams.insert( below_right );
                count += 1;

            }
            current_beams = next_beams.into_iter().collect();
        }

        count
        

    }

    fn count_timelines( &self ) -> usize {
        let mut count: usize = 0;
        let mut current_beams: HashMap<i32, usize> = vec![ (self.start.0, 1) ].into_iter().collect();


        for i in 0..(self.height + 10) {

            let mut next_beams: HashMap<i32, usize> = HashMap::new();
            while let Some( beam ) = current_beams.keys().next().cloned() {
                let beam_count = current_beams.remove( &beam ).unwrap();

                if !self.grid.contains(&(beam, i)) {
                    if next_beams.contains_key( &beam ) {
                        *next_beams.get_mut( &beam ).unwrap() += beam_count;
                    } else {
                        next_beams.insert( beam, beam_count );
                    }
                    continue;
                }

                let below_left = beam - 1;
                let below_right = beam + 1;

                if next_beams.contains_key( &below_left ) {
                    *next_beams.get_mut( &below_left ).unwrap() += beam_count;
                } else {
                    next_beams.insert( below_left, beam_count );
                }

                if next_beams.contains_key( &below_right ) {
                    *next_beams.get_mut( &below_right ).unwrap() += beam_count;
                } else {
                    next_beams.insert( below_right, beam_count );
                }
            }
            current_beams = next_beams.into_iter().collect();
        }


        for (_, beam_count) in current_beams.iter() {
            count += beam_count;
        }
        count
        

    }
}

fn main() {
    let input = read_txt_file( "input.txt" );
    let puzzle = Puzzle::from( input.as_str() );

    puzzle.display();

    let splits = puzzle.count_splits();
    println!( "Number of splits: {}", splits );

    let timelines = puzzle.count_timelines();
    println!( "Number of timelines: {}", timelines );
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
