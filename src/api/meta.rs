use http::Method;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthKind {
    Server,
    Account,
    None,
}

impl AuthKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            AuthKind::Server => "server",
            AuthKind::Account => "account",
            AuthKind::None => "none",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndpointMeta {
    pub method: Method,
    pub path_template: &'static str,
    pub auth: AuthKind,
    pub has_query: bool,
    pub has_body: bool,
    pub request_type: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EndpointMetaRow {
    pub method: &'static str,
    pub path_template: &'static str,
    pub auth: &'static str,
    pub has_query: bool,
    pub has_body: bool,
    pub request_type: &'static str,
}

impl From<EndpointMeta> for EndpointMetaRow {
    fn from(value: EndpointMeta) -> Self {
        EndpointMetaRow {
            method: match value.method {
                Method::GET => "GET",
                Method::POST => "POST",
                Method::PUT => "PUT",
                Method::PATCH => "PATCH",
                Method::DELETE => "DELETE",
                _ => "OTHER",
            },
            path_template: value.path_template,
            auth: value.auth.as_str(),
            has_query: value.has_query,
            has_body: value.has_body,
            request_type: value.request_type,
        }
    }
}

pub const LIST_SERVERS_META: EndpointMeta = EndpointMeta {
    method: Method::GET,
    path_template: "/servers",
    auth: AuthKind::Account,
    has_query: true,
    has_body: false,
    request_type: "api::server::ListServersRequest",
};

pub const LIST_WEBHOOKS_META: EndpointMeta = EndpointMeta {
    method: Method::GET,
    path_template: "/webhooks",
    auth: AuthKind::Server,
    has_query: true,
    has_body: false,
    request_type: "api::webhooks::ListWebhooksRequest",
};

pub const OUTBOUND_SEARCH_META: EndpointMeta = EndpointMeta {
    method: Method::GET,
    path_template: "/messages/outbound",
    auth: AuthKind::Server,
    has_query: true,
    has_body: false,
    request_type: "api::messages::OutboundSearchRequest",
};

pub fn canonical_endpoint_map() -> Vec<EndpointMetaRow> {
    [
        LIST_SERVERS_META.clone(),
        LIST_WEBHOOKS_META.clone(),
        OUTBOUND_SEARCH_META.clone(),
    ]
    .into_iter()
    .map(EndpointMetaRow::from)
    .collect()
}
