use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Copy)]
pub struct ConnectionId(Uuid);

impl ConnectionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for ConnectionId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ConnectionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConnection {
    pub id: ConnectionId,  // Made public
    pub name: String,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub connection_type: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_connected: Option<chrono::DateTime<chrono::Utc>>,
    pub is_active: bool,
}

impl DatabaseConnection {
    pub fn new(
        name: String,
        host: String,
        port: u16,
        database: String,
        username: String,
        password: String,
        connection_type: String,
    ) -> Self {
        Self {
            id: ConnectionId::new(),
            name,
            host,
            port,
            database,
            username,
            password,
            connection_type,
            created_at: chrono::Utc::now(),
            last_connected: None,
            is_active: false,
        }
    }

    // Getter methods for better encapsulation (optional)
    pub fn id(&self) -> ConnectionId {
        self.id.clone()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn database(&self) -> &str {
        &self.database
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn connection_type(&self) -> &str {
        &self.connection_type
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
        if active {
            self.last_connected = Some(chrono::Utc::now());
        }
    }

    pub fn connection_string(&self) -> String {
        match self.connection_type.as_str() {
            "postgresql" => format!(
                "postgresql://{}:{}@{}:{}/{}",
                self.username, self.password, self.host, self.port, self.database
            ),
            "mysql" => format!(
                "mysql://{}:{}@{}:{}/{}",
                self.username, self.password, self.host, self.port, self.database
            ),
            "sqlite" => format!("sqlite://{}", self.database),
            _ => format!(
                "{}://{}:{}@{}:{}/{}",
                self.connection_type, self.username, self.password, self.host, self.port, self.database
            ),
        }
    }

    pub fn display_name(&self) -> String {
        format!("{} ({}:{})", self.name, self.host, self.port)
    }
}

impl PartialEq for DatabaseConnection {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for DatabaseConnection {}

impl std::hash::Hash for DatabaseConnection {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}