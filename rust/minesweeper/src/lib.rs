const SPACE:u8 = 32;
const MINE:u8 = 42;

// pub fn get_mines(minefield: &[&str], position: (i32, i32)) -> u8 {
//     return 1
// }

pub fn create_res_vec(minefield: &[&str]) -> Vec<String> {
    if !minefield.is_empty() {
        let str:Vec<u8> = vec![32; minefield[0].len()];
        let res: Vec<String> = vec![String::from_utf8(str)
                                            .expect("Invalid UTF-8"); minefield.len()];
        res
    }
    else {
        vec![]
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let res: Vec<String> = create_res_vec(minefield);
    for str in &res {
        println!("str : {:?}", str);
    }

    for (_i, str) in minefield.iter().enumerate() {
        for (_j, b) in str.bytes().enumerate() {
            print!("{:?}", b);
        }
        println!();
    }
    return res
}
