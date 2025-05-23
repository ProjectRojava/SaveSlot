-- Rojava SaveSlot - Database initialization script

-- Create database schema
CREATE SCHEMA IF NOT EXISTS rojava;

-- Create connections table
CREATE TABLE IF NOT EXISTS rojava.connections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    database_type VARCHAR(50) NOT NULL,
    host VARCHAR(255) NOT NULL,
    port INTEGER NOT NULL,
    database_name VARCHAR(255) NOT NULL,
    username VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'disconnected',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create queries table
CREATE TABLE IF NOT EXISTS rojava.queries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    connection_id UUID NOT NULL REFERENCES rojava.connections(id) ON DELETE CASCADE,
    sql_text TEXT NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'draft',
    execution_time_ms BIGINT,
    rows_affected INTEGER,
    error_message TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    executed_at TIMESTAMP WITH TIME ZONE
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_connections_name ON rojava.connections(name);
CREATE INDEX IF NOT EXISTS idx_queries_connection_id ON rojava.queries(connection_id);
CREATE INDEX IF NOT EXISTS idx_queries_created_at ON rojava.queries(created_at);

-- Insert sample data
INSERT INTO rojava.connections (name, database_type, host, port, database_name, username, status) 
VALUES 
    ('Local PostgreSQL', 'postgresql', 'localhost', 5432, 'rojava_db', 'postgres', 'connected'),
    ('Development MySQL', 'mysql', 'localhost', 3306, 'dev_db', 'dev_user', 'disconnected')
ON CONFLICT DO NOTHING;

-- Create a sample query
INSERT INTO rojava.queries (connection_id, sql_text, status)
SELECT id, 'SELECT * FROM rojava.connections LIMIT 10;', 'draft'
FROM rojava.connections 
WHERE name = 'Local PostgreSQL'
ON CONFLICT DO NOTHING;