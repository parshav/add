use std::env;

fn main() {
    let mut add: f64 = 0.0;
    env::args().into_iter()
        .skip(1)
        .map(|num| { num.parse::<f64>().expect("failed to parse") })
        .for_each(|f| {
            add += f;
        });
    println!("Addition : {}", add);
}
