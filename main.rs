mod algorithms {
    pub mod search;
}

fn main() {
    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let search = algorithms::search::Search::new(values);
    println!("{:?}", search.clone().linear_search(&5));
    println!("{:?}", search.binary_search(&9));   
}