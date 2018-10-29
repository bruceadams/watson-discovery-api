pub struct ResourceController {
    /// API key for Resource Controller
    pub apikey: String,
    /// bearer token
    pub bearer: Option<String>,
    /// refresh token
    pub refresh: Option<String>,
}

pub struct CloudFoundry {
    /// Username for Cloud Foundry
    pub username: String,
    /// Password for Cloud Foundry
    pub password: String,
}

pub enum Authentication {
    ResourceController(ResourceController),
    CloudFoundry(CloudFoundry),
}

pub struct DiscoveryInstance {
    pub auth: Authentication,
    /// Base URL
    pub url: String,
}

impl DiscoveryInstance {
    pub fn new_apikey(apikey: &str, url: &str) -> DiscoveryInstance {
        DiscoveryInstance {
            auth: Authentication::ResourceController(ResourceController {
                apikey: apikey.to_string(),
                bearer: None,
                refresh: None,
            }),
            url: url.to_string(),
        }
    }

    pub fn new_username_password(username: &str, password: &str, url: &str) -> DiscoveryInstance {
        DiscoveryInstance {
            auth: Authentication::CloudFoundry(CloudFoundry {
                username: username.to_string(),
                password: password.to_string(),
            }),
            url: url.to_string(),
        }
    }
}
