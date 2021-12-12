use futures::Future;

// this function
async fn foo(x: &u8) -> u8 {
     *x
}

// is equivalent to this function:
fn foo_expanded<'a>(x: &'a u8) -> impl Future<Output = u8> + 'a {
    async move { *x }
}

async fn blocks() {
    let my_string = "foo".to_string();
    let future_one = async {};
}