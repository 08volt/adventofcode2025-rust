

use std::collections::HashSet;

fn main() {
    let input = read_txt_file( "input.txt" );
    let mut rolls: HashSet<(i64, i64)> = input.lines().enumerate().flat_map(
        |(x, line)| 
            line.chars().enumerate().filter(move |(_, cc)| cc.clone() == '@')
                .map(move |(y, _)| 
                {
                    (x as i64, y as i64)
                })
                    
                ).collect();

    let mut total = 0;

    let mut counts = 1;

    while counts > 0 {

        let to_remove: HashSet<(i64, i64)> = rolls.iter().filter(|(x,y)| 
            (rolls.contains(&(*x,*y + 1)) as i32 + 
            rolls.contains(&(*x,*y - 1)) as i32 + 
            rolls.contains(&(*x + 1,*y)) as i32 + 
            rolls.contains(&(*x - 1,*y)) as i32 + 
            rolls.contains(&(*x+1,*y + 1))as i32 + 
            rolls.contains(&(*x+1,*y - 1)) as i32 + 
            rolls.contains(&(*x - 1,*y + 1)) as i32 + 
            rolls.contains(&(*x - 1,*y -1))as i32 )< 4)
            .copied()
            .collect();


        counts = to_remove.len();
        println!("Rmoved {}", counts);


        rolls = rolls.difference(&to_remove).copied().collect();

        total += counts
    }
    println!("Total: {}", total);
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
