use std::{collections::{HashMap, HashSet}, hash::Hash};


#[derive(Clone, Hash, PartialEq, Eq, Copy)]
struct BoxPosition {
    x: u32,
    y: u32,
    z: u32,
}

impl From<&str> for BoxPosition {
    fn from( s: &str ) -> Self {
        let coords: Vec<u32> = s
            .split(',')
            .map( |part| part.trim().parse::<u32>().unwrap() )
            .collect();
        BoxPosition {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
    }
}

impl BoxPosition {
    fn euclidean_distance( &self, other: &BoxPosition ) -> f64 {
        let dx = ( self.x as i32 - other.x as i32 ) as f64;
        let dy = ( self.y as i32 - other.y as i32 ) as f64;
        let dz = ( self.z as i32 - other.z as i32 ) as f64;
        ( dx * dx + dy * dy + dz * dz ).sqrt()
    }
}


struct BoxGrouping {
    groups: HashMap<BoxPosition, BoxPosition>,
    sizes: HashMap<BoxPosition, usize>,
}

impl std::fmt::Debug for BoxPosition {
    fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result {
        write!( f, "({}, {}, {})", self.x, self.y, self.z )
    }
}

impl BoxGrouping {
    fn new( positions: &Vec<BoxPosition> ) -> Self {
        let groups: HashMap<BoxPosition, BoxPosition> = positions.iter().map( |pos| ( pos.clone(), pos.clone()) ).collect();
        let sizes: HashMap<BoxPosition, usize> =  positions.iter().map( |pos| ( pos.clone(), 1 ) ).collect();
        BoxGrouping { groups, sizes }
    }

    fn find_group(&self, mut x: BoxPosition) -> BoxPosition {
        while let Some(&parent) = self.groups.get(&x) {
            if parent == x {
                return x;
            }
            x = parent;
        }
        x 
    }

    fn union_groups( &mut self, x: BoxPosition, y: BoxPosition ) {
        let group_x = self.find_group( x );
        let group_y = self.find_group( y );

        if group_x != group_y {
            let size_x = *self.sizes.get( &group_x ).unwrap();
            let size_y = *self.sizes.get( &group_y ).unwrap();

            if size_x < size_y {
                self.groups.insert( group_x, group_y );
                self.sizes.insert( group_y, size_x + size_y );
            } else {
                self.groups.insert( group_y, group_x );
                self.sizes.insert( group_x, size_x + size_y );
            }
        }
    }

    fn union_groups_count(&self) -> usize {
        let unique_groups: HashSet<BoxPosition> = self.groups.iter().map( |(&pos, _)| {
            self.find_group( pos )
        } ).collect();
        unique_groups.len()
    }

    fn group_size( &self, x: BoxPosition ) -> usize {
        let group = self.find_group( x );
        *self.sizes.get( &group ).unwrap()
    }

    fn top_3_largest_groups( &self ) -> Vec<usize> {

        let unique_groups: HashSet<BoxPosition> = self.groups.iter().map( |(&pos, _)| {
            self.find_group( pos )
        } ).collect();
        let mut sizes: Vec<usize> = unique_groups.iter().map( |group| self.group_size( *group ) ).collect();
        sizes.sort_unstable_by( |a, b| b.cmp( a ) );
        sizes.truncate( 3 );
        sizes
    }
}


fn main() {
    let content = read_txt_file( "input.txt" );
    let positions: Vec<BoxPosition> = content
        .lines()
        .map( |line| BoxPosition::from( line ) )
        .collect();

    let distance_matrix: Vec<Vec<f64>> = positions
        .iter()
        .map( |pos1| {
            positions
                .iter()
                .map( |pos2| pos1.euclidean_distance( pos2 ) )
                .collect()
        } )
        .collect();

    let mut connections: Vec<((BoxPosition, BoxPosition), f64)> = {
        let mut conns = Vec::new();
        for x in 0..positions.len() {
            for y in 0..positions.len() {
                if x < y {
                    conns.push( ( ( positions[x], positions[y] ), distance_matrix[x][y] ) );
                }
            }
        }
        conns
    };

    connections.sort_by( |a, b| a.1.partial_cmp( &b.1 ).unwrap() );

    let top10_connections: Vec<((BoxPosition, BoxPosition), f64)> = connections
        .iter()
        .cloned()
        .take( 1000)
        .collect();

    let mut grouping = BoxGrouping::new( &positions );


    for ( ( pos1, pos2 ), dist ) in top10_connections {
        grouping.union_groups( pos1, pos2 );
    }

    println!("Top 3: {:?}\n", grouping.top_3_largest_groups() );

    println!("Multiply top 3: {}\n",
        grouping.top_3_largest_groups().iter().product::<usize>()
    );

    let mut last_couple = None;

    for ( ( pos1, pos2 ), dist ) in &connections {
        grouping.union_groups( *pos1, *pos2 );
        if grouping.union_groups_count() == 1 {
            last_couple = Some( ( *pos1, *pos2 )  );
            break;
        }
    }    

    println!("{}", last_couple.unwrap().0.x * last_couple.unwrap().1.x );

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
