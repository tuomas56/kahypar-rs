fn main() {
    let mut context = kahypar::Context::from_config("examples/config.ini");
    let mut hypergraph = kahypar::Hypergraph::from_raw(
            2, 7,
            &[0, 2, 0, 1, 3, 4, 3, 4, 6, 2, 5, 6],
            &[0, 2, 6, 9, 12]
        ).edge_weights(&[
            1, 1000, 1, 1000
        ]).build();
    println!("{:?}", hypergraph.partition(&mut context, 0.03));
}