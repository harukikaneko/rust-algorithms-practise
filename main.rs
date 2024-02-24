use std::time;

mod algorithms {
    pub mod search;
    pub mod stack_queue;
    pub mod tree_node;
}

fn main() {
    let values = (1..1000).collect::<Vec<i32>>();
    let search = algorithms::search::Search::new(values);

    let now = time::Instant::now();
    println!("{:?}", search.clone().linear_search(&890));
    println!("linear{:?}", now.elapsed());

    let now2 = time::Instant::now();
    println!("{:?}", search.binary_search(&890));
    println!("binary{:?}", now2.elapsed());
}
