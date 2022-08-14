fn main() {
    let mut sorted_vec: Vec<u32> = vec![];
    let data: [u32; 8] = [1, 5, 15, 2, 6, 20, 4, 19];
    for x in data {
        match sorted_vec.binary_search(&x) {
            Ok(_) => {}
            Err(index) => {
                sorted_vec.insert(index, x);
            }
        }
    }
    println!("{:?}", sorted_vec);

    let mut sorted_two_vec: Vec<(u32, String)> = vec![];
    let data: Vec<(u32, String)> = vec![
        (1, "How".to_owned()),
        (5, "What".to_owned()),
        (15, "Where".to_owned()),
        (2, "Whom".to_owned()),
        (6, "Why".to_owned()),
        (20, "What if".to_owned()),
        (4, "Yes".to_owned()),
        (19, "No".to_owned())
    ];
    for x in data {
        match sorted_two_vec.binary_search_by(|(c,_)| c.cmp(&x.0)) {
            Ok(_) => {}
            Err(index) => {
                sorted_two_vec.insert(index, x);
            }
        }
    }

    println!("{:?}", sorted_two_vec);

}
