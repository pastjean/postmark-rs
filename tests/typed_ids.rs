#[test]
fn typed_ids_roundtrip_serde() {
    let id = postmark::api::server::ServerId::new(42);
    let json = serde_json::to_string(&id).expect("serialize server id");
    assert_eq!(json, "42");

    let back: postmark::api::server::ServerId =
        serde_json::from_str(&json).expect("deserialize server id");
    assert_eq!(back.get(), 42);
}
