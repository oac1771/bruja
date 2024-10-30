#[cfg(test)]
mod tests {
    use utils::p2p::NodeBuilder;

    #[tokio::test]
    async fn foo() {
        let node_1 = NodeBuilder::build().unwrap();
        let node_2 = NodeBuilder::build().unwrap();
    }
}
