#[test]
fn endpoint_map_is_consistent_for_pilot_endpoints() {
    let map = postmark::api::meta::canonical_endpoint_map();

    assert!(
        map.iter()
            .any(|e| e.path_template == "/servers" && e.method == "GET")
    );
    assert!(
        map.iter()
            .any(|e| e.path_template == "/webhooks" && e.method == "GET")
    );
    assert!(
        map.iter()
            .any(|e| e.path_template == "/messages/outbound" && e.method == "GET")
    );
}

#[test]
fn endpoint_map_has_auth_and_query_body_flags() {
    let map = postmark::api::meta::canonical_endpoint_map();
    let servers = map
        .iter()
        .find(|m| m.path_template == "/servers")
        .expect("/servers should exist");

    assert_eq!(servers.auth, "account");
    assert!(servers.has_query);
    assert!(!servers.has_body);
}
