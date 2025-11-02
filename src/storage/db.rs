use crate::models::event::Event;
use anyhow::{anyhow, Context, Result};
use chrono::{TimeZone, Utc};
use rusqlite::{types::Type, params, Connection, Error as RusqliteError};
use std::fs;
use std::path::PathBuf;

fn get_db_path() -> Result<PathBuf> {
    let data_dir = directories::ProjectDirs::from("com", "calendar-app", "calendar-app")
        .context("Failed to get data directory")?
        .data_dir()
        .to_path_buf();
    fs::create_dir_all(&data_dir)?;
    Ok(data_dir.join("events.db"))
}

pub fn initialize_db() -> Result<Connection> {
    let db_path = get_db_path()?;
    let conn = Connection::open(db_path)?;
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS events (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT,
            start_datetime INTEGER NOT NULL,
            end_datetime INTEGER NOT NULL,
            location TEXT,
            created_at INTEGER DEFAULT (strftime('%s', 'now')),
            updated_at INTEGER DEFAULT (strftime('%s', 'now'))
        );
        CREATE INDEX IF NOT EXISTS idx_events_start ON events(start_datetime);
        ",
    )?;
    Ok(conn)
}

pub fn get_events_in_range(conn: &Connection, start: i64, end: i64) -> Result<Vec<Event>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, description, start_datetime, end_datetime, location, created_at, updated_at
         FROM events
         WHERE start_datetime < ?2 AND end_datetime > ?1",
    )?;
    let event_iter = stmt.query_map([start, end], |row| {
        let start_ts: i64 = row.get(3)?;
        let end_ts: i64 = row.get(4)?;
        let created_ts: i64 = row.get(6)?;
        let updated_ts: i64 = row.get(7)?;

        Ok(Event {
            id: Some(row.get(0)?),
            title: row.get(1)?,
            description: row.get(2)?,
            start_datetime: Utc
                .timestamp_opt(start_ts, 0)
                .single()
                .ok_or_else(|| RusqliteError::InvalidColumnType(3, "Invalid timestamp".into(), Type::Integer))?,
            end_datetime: Utc
                .timestamp_opt(end_ts, 0)
                .single()
                .ok_or_else(|| RusqliteError::InvalidColumnType(4, "Invalid timestamp".into(), Type::Integer))?,
            location: row.get(5)?,
            created_at: Utc
                .timestamp_opt(created_ts, 0)
                .single()
                .ok_or_else(|| RusqliteError::InvalidColumnType(6, "Invalid timestamp".into(), Type::Integer))?,
            updated_at: Utc
                .timestamp_opt(updated_ts, 0)
                .single()
                .ok_or_else(|| RusqliteError::InvalidColumnType(7, "Invalid timestamp".into(), Type::Integer))?,
        })
    })?;

    let mut events = Vec::new();
    for event in event_iter {
        events.push(event?);
    }
    Ok(events)
}

pub fn create_event(conn: &Connection, event: &Event) -> Result<i64> {
    let mut stmt = conn.prepare(
        "INSERT INTO events (title, description, start_datetime, end_datetime, location)
         VALUES (?1, ?2, ?3, ?4, ?5)",
    )?;
    let id = stmt.insert(params![
        event.title,
        event.description,
        event.start_datetime.timestamp(),
        event.end_datetime.timestamp(),
        event.location,
    ])?;
    Ok(id)
}

pub fn update_event(conn: &Connection, event: &Event) -> Result<()> {
    let id = event
        .id
        .ok_or_else(|| anyhow!("Cannot update event without ID"))?;
    conn.execute(
        "UPDATE events SET title = ?1, description = ?2, start_datetime = ?3, end_datetime = ?4, location = ?5, updated_at = strftime('%s', 'now') WHERE id = ?6",
        params![
            event.title,
            event.description,
            event.start_datetime.timestamp(),
            event.end_datetime.timestamp(),
            event.location,
            id,
        ],
    )?;
    Ok(())
}

pub fn delete_event(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM events WHERE id = ?1", params![id])?;
    Ok(())
}
