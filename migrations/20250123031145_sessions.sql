PRAGMA foreign_keys = ON;
CREATE TABLE IF NOT EXISTS sessions (
            session_id INTEGER PRIMARY KEY AUTOINCREMENT,
            server_id VARCHAR(250) NOT NULL,
            title VARCHAR(250) NOT NULL,
            start_time VARCHAR(250) NOT NULL,
            end_time VARCHAR(250) NOT NULL,
            owner VARCHAR(250) NOT NULL,
            game VARCHAR(250)
);
CREATE INDEX idx_server_id
ON sessions (server_id);
CREATE TABLE IF NOT EXISTS users (
            user_id VARCHAR(250) NOT NULL,
            session_id INTEGER NOT NULL,
            user_photo VARCHAR(250) NOT NULL,
            PRIMARY KEY (session_id, user_id),
            FOREIGN KEY (session_id)
                REFERENCES sessions (session_id)
                ON DELETE CASCADE
);
/*CREATE TABLE IF NOT EXISTS preferences (
            id INTEGER AUTO_INCREMENT PRIMARY KEY NOT NULL,
            user_id VARCHAR(250) NOT NULL,
            session_id INTEGER NOT NULL,
            suggested_game VARCHAR(250) NOT NULL,
            is_selected BOOL NOT NULL,
            FOREIGN KEY (user_id, session_id)
                REFERENCES users (user_id, session_id)
                ON DELETE CASCADE
);
CREATE INDEX idx_for_preferences
ON preferences (user_id, session_id);*/