use rusqlite::{Connection, Result};

use crate::DB_PATH;

pub fn initialise_db() -> Result<()> {
    let conn = Connection::open(DB_PATH)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS presets (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            game_path TEXT,
            created_at TEXT NOT NULL,
            xml_path TEXT NOT NULL,
            section_name TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS preset_opts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            preset_id TEXT NOT NULL,
            name TEXT NOT NULL,
            selected TEXT NOT NULL,
            choices TEXT NOT NULL,
            FOREIGN KEY (preset_id) REFERENCES presets (id)
        )",
        [],
    )?;


    conn.execute(
    "CREATE TABLE IF NOT EXISTS game_dirs (
        name TEXT PRIMARY KEY,
        path TEXT NOT NULL
    )",
    [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
        [],
    )?;



    Ok(())
}