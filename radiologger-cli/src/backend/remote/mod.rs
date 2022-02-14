use super::Connection;

struct RemoteConnection {
    access_token: String,
}

impl Connection for RemoteConnection {}
