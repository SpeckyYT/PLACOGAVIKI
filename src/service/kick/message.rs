pub mod v1 {
    #[derive(Debug, Clone, Default)]
    pub struct User {
        pub id: u64,
        pub username: String,
        pub role: String,
        pub is_super_admin: bool, // NOTE: isSuperAdmin
        pub profile_thumb: String,
        pub verified: bool,
        pub follower_badges: Vec<()>, // no idea
        pub is_subscribed: bool,
        pub is_founder: bool,
        pub months_subscribed: u64,
        pub quantity_gifted: u64,
    }
    #[derive(Debug, Clone, Default)]
    pub struct Message {
        // important
        pub id: String,
        pub content: String,
        pub typ: String, // NOTE: it's "type"
        pub chatroom_id: u64,
        pub created_at: u64,
        pub user: User,

        // flavor
        pub replied_to: Option<String>, // no idea
        pub link_preview: Option<()>, // no idea
        pub role: Option<()>, // no idea
        pub action: Option<()>, // no idea
        pub optional_message: Option<()>, // no idea
        pub months_subscribed: Option<u64>,
        pub subscriptions_count: Option<u64>, // no idea
        pub gifted_users: Option<()>, // NOTE: it's "giftedUsers"
    }
    use json::object::Object;
    impl Message {
        #[allow(dead_code)]
        pub fn new(data: &Object) -> Self {
            let message = data;
            let user = &data["user"];
            Message {
                id: message["id"].as_str().unwrap_or_default().to_string(),
                content: message["content"].as_str().unwrap_or_default().to_string(),
                typ: message["type"].as_str().unwrap_or_default().to_string(),
                chatroom_id: message["chatroom_id"].as_str().unwrap_or_default().parse().unwrap_or_default(),
                created_at: message["created_at"].as_u64().unwrap_or_default(),
                replied_to: message["replied_to"].as_str().map(|s| s.to_string()), // no idea
                link_preview: None, // no idea
                role: None, // no idea
                action: None, // no idea
                optional_message: None, // no idea
                months_subscribed: message["months_subscribed"].as_u64(),
                subscriptions_count: message["subscriptions_count"].as_u64(), // no idea
                gifted_users: None, // NOTE: it's "giftedUsers"
                user: User {
                    id: user["id"].as_u64().unwrap_or_default(),
                    username: user["username"].as_str().unwrap_or_default().to_string(),
                    role: user["role"].as_str().unwrap_or_default().to_string(),
                    is_super_admin: user["isSuperAdmin"].as_bool().unwrap_or_default(),
                    profile_thumb: user["profile_thumb"].as_str().unwrap_or_default().to_string(),
                    verified: user["verified"].as_bool().unwrap_or_default(),
                    follower_badges: Default::default(), // no idea
                    is_subscribed: user["is_subscribed"].as_bool().unwrap_or_default(),
                    is_founder: user["is_founder"].as_bool().unwrap_or_default(),
                    months_subscribed: user["months_subscribed"].as_u64().unwrap_or_default(),
                    quantity_gifted: user["quantity_gifted"].as_u64().unwrap_or_default(),
                }
            }
        }
    }
}

pub mod v2 {
    #[derive(Debug, Clone, Default)]
    pub struct Badge {
        pub typ: String, // NOTE: it's "type"
        pub text: String,
        pub count: Option<u64>,
    }
    #[derive(Debug, Clone, Default)]
    pub struct Identity {
        pub color: String,      // hex
        pub badges: Vec<Badge>,
    }
    #[derive(Debug, Clone, Default)]
    pub struct Sender {
        pub id: u64,            // user ID
        pub username: String,   // username
        pub slug: String,       // user "link"
        pub identity: Identity, // "decorations"
    }
    #[derive(Debug, Clone, Default)]
    pub struct Message {
        pub id: String,
        pub chatroom_id: u64,
        pub content: String,
        pub typ: String,        // NOTE: it's "type"
        pub created_at: String,
        pub sender: Sender,
    }
    use json::object::Object;
    impl Message {
        #[allow(dead_code)]
        pub fn new(data: &Object) -> Self {
            let sender = &data["sender"];
            Message {
                id: data["id"].as_str().unwrap_or_default().to_string(),
                chatroom_id: data["chatroom_id"].as_u64().unwrap_or_default(),
                content: data["content"].as_str().unwrap_or_default().to_string(),
                typ: data["type"].as_str().unwrap_or_default().to_string(),
                created_at: data["created_at"].as_str().unwrap_or_default().to_string(),
                sender: Sender {
                    id: sender["id"].as_u64().unwrap_or_default(),
                    username: sender["username"].as_str().unwrap_or_default().to_string(),
                    slug: sender["slug"].as_str().unwrap_or_default().to_string(),
                    identity: {
                        let identity = &sender["identity"];
                        Identity {
                            color: identity["color"].as_str().unwrap_or_default().to_string(),
                            badges: {
                                if let json::JsonValue::Array(badges) = &identity["badges"] {
                                    badges.iter().map(|badge| Badge {
                                        typ: badge["type"].as_str().unwrap_or_default().to_string(),
                                        text: badge["text"].as_str().unwrap_or_default().to_string(),
                                        count: badge["count"].as_u64(),
                                    }).collect()
                                } else {
                                    vec![]
                                }
                            },
                        }
                    },
                }
            }
        }
    }
}
