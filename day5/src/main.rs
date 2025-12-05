use std::fmt::Debug;


struct Range {
    start: usize,
    end: usize
}

impl Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Range").field("start", &self.start).field("end", &self.end).finish()
    }
}

impl Range {
    fn contains(&self, a: &usize) -> bool {
        a >= &self.start && a <= &self.end
    }
}


fn main() {
    let file_content = read_txt_file("input.txt");
    let splits: Vec<&str> = file_content.split("\n\n").collect();
    let mut ranges: Vec<Range> = splits[0].lines().map(|l| {
        let r_list: Vec<usize> = l.split("-").map(|s| s.parse::<usize>().unwrap()).collect();
        Range {
            start: r_list[0],
            end: r_list[1]
        }

    }).collect();
    
    let ids: Vec<usize> = splits[1].lines().map(|l| l.parse::<usize>().unwrap()).collect();

    let count = ids.iter().filter(|id| ranges.iter().any(|r| r.contains(id))).count();

    println!("{}", count);

    ranges.sort_by(|r1, r2| r1.start.cmp(&r2.start));

    let mut compressed_ranges: Vec<Range> = Vec::new();

    let mut current_end = 0;
    let mut current_start = 0;

    for r in ranges {
        if current_start == 0 {
            current_start = r.start
        }
        if r.start > current_end && current_end != 0{
            // add previous range
            compressed_ranges.push(Range { start: current_start, end: current_end });
            current_start = r.start;
            current_end = r.end;
        } else {
            current_end = std::cmp::max(current_end, r.end);
        }
    }

    compressed_ranges.push(Range { start: current_start, end: current_end });

    // println!("{:?}", compressed_ranges);

    let tot: usize= compressed_ranges.iter().map(|r| r.end - r.start + 1).sum();

    println!("{}", tot)

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
