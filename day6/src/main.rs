

fn main() {
    let content = read_txt_file("input.txt");
    let rows = content
        .lines()
        .map( |line| line.split_whitespace().collect::<Vec<&str>>() )
        .collect::<Vec<Vec<&str>>>();

    let operations = rows.last().unwrap();


    // Convert rows to Vec<Vec<char>> excluding the last row to integers
    let rows = &rows[..rows.len()-1].iter().map(|r| {
        r.iter().map(|c| c.parse::<u64>().unwrap() ).collect::<Vec<u64>>()
    }).collect::<Vec<Vec<u64>>>();

    let results = rows.iter().skip(1).fold(rows[0].clone(), |acc, r| {
        acc.iter().zip(r.iter()).zip(operations.iter()).map(|((a, b), op)| {
            match *op {
                "+" => a + b,
                "*" => a * b,
                _ => panic!("Unsupported operation"),
            }
        }).collect::<Vec<u64>>()
    });
    println!("first part: {:?}", results.iter().sum::<u64>());


    let rows = content
        .lines()
        .map( |line| line.chars().collect::<Vec<char>>() )
        .collect::<Vec<Vec<char>>>();

    let operations = rows.last().unwrap().iter().filter(|c| !c.is_whitespace()).collect::<Vec<&char>>();
    let rows = &rows[..rows.len()-1];

    println!("operations: {:?}", operations);

    // transport rows to columns
    let cols = (0..rows[0].len()).map( |i| {
        rows.iter().map( |r| r[i] ).collect::<Vec<char>>()
    }).collect::<Vec<Vec<char>>>();

    // char columns into integers grouping by full column white spaces
    let column_numbers = cols.iter().filter( |col| {
        col.iter().all( |c| c.is_whitespace() || c.is_numeric() )
    }).map( |col| {
        let mut current_number = String::new();
        for c in col {
            if c.is_numeric() {
                current_number.push(*c);
            } else if !current_number.is_empty() {
                return Some(current_number.parse::<u64>().unwrap());
            }
        }
        if !current_number.is_empty() {
            return Some(current_number.parse::<u64>().unwrap());
        }
        None
    }).collect::<Vec<Option<u64>>>();
    println!("column numbers: {:?}", column_numbers);

    let mut op_iter = operations.iter();

    let mut current_op = op_iter.next();

    let mut current_number = column_numbers.iter();

    let mut current_result: u64 = 0;

    let mut result = 0;

    while let Some(op) = current_op {
        let number = current_number.next();

        match number {
            Some(Some(n)) => {
                match *op {
                    '+' => {
                        current_result = n + current_result;
                    },                    
                    '*' => {
                        if current_result == 0 {
                            current_result = *n;
                        } else {
                            current_result = n * current_result;
                        }
                    },
                    _ => panic!("Unsupported operation"),
                }
            },
            Some(None) => {
                current_op = op_iter.next();
                result += current_result;
                current_result = 0;
            },
            None => break,
        }        
    }
    result += current_result;
    println!("second part: {:?}", result);
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
