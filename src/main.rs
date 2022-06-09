use typed_builder::TypedBuilder;

#[allow(dead_code)]
#[derive(TypedBuilder, Debug)]
struct Params {
    address: String,
    token: String,

    #[builder(default=5)]
    attempts: u8,

    #[builder(default=vec!["node1".to_string()])]
    nodes: Vec<String>

}

fn main() {
    let params = Params::builder().address("a1".into()).token("t1".into()).build();
    dbg!(params);
}
