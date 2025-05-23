use datamind_ide::domain::model::{DatabaseConnection, DatabaseType, ConnectionStatus};

#[test]
fn test_create_valid_connection() {
    // Arrange & Act
    let connection = DatabaseConnection::new(
        "Test DB".to_string(),
        DatabaseType::PostgreSQL,
        "localhost".to_string(),
        5432,
        "testdb".to_string(),
        "testuser".to_string(),
    );

    // Assert
    assert!(connection.is_ok());
    let conn = connection.unwrap();
    assert_eq!(conn.name(), "Test DB");
    assert_eq!(conn.database_type(), &DatabaseType::PostgreSQL);
    assert_eq!(conn.host(), "localhost");
    assert_eq!(conn.port(), 5432);
    assert_eq!(conn.database_name(), "testdb");
    assert_eq!(conn.username(), "testuser");
    assert_eq!(conn.status(), &ConnectionStatus::Disconnected);
}

#[test]
fn test_create_connection_with_empty_name() {
    // Arrange & Act
    let connection = DatabaseConnection::new(
        "".to_string(),
        DatabaseType::PostgreSQL,
        "localhost".to_string(),
        5432,
        "testdb".to_string(),
        "testuser".to_string(),
    );

    // Assert
    assert!(connection.is_err());
    assert_eq!(connection.unwrap_err(), "Connection name cannot be empty");
}

#[test]
fn test_connection_state_transitions() {
    // Arrange
    let mut connection = DatabaseConnection::new(
        "Test DB".to_string(),
        DatabaseType::PostgreSQL,
        "localhost".to_string(),
        5432,
        "testdb".to_string(),
        "testuser".to_string(),
    ).unwrap();

    // Act & Assert - Connecting
    connection.mark_as_connecting();
    assert_eq!(connection.status(), &ConnectionStatus::Connecting);

    // Act & Assert - Connected
    connection.mark_as_connected();
    assert_eq!(connection.status(), &ConnectionStatus::Connected);

    // Act & Assert - Failed
    connection.mark_as_failed("Connection timeout".to_string());
    match connection.status() {
        ConnectionStatus::Failed(error) => assert_eq!(error, "Connection timeout"),
        _ => panic!("Expected Failed status"),
    }

    // Act & Assert - Disconnected
    connection.disconnect();
    assert_eq!(connection.status(), &ConnectionStatus::Disconnected);
}