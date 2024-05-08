use std::collections::HashMap;

struct Service {
    ip: String, // ??
    key: String,
}

impl Service {
    pub fn new(ip: &str, key: &str) -> Self {
        Self{
            ip: ip.to_string(),
            key: key.to_string(),
        }
    }
}

pub static SERVICES: HashMap<&'static str, &'static Service> = HashMap::from ([
    ("MAILER/", Service::new("MailerIP", "MailerKey")),
    ("AUTH/", Service::new("AuthIp", "AuthKey")),
    ("DASHBOARD/", Service::new("DashIp", "DashKey")),
]);