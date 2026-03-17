import fs from 'fs'
import path from 'path'
import Database from 'better-sqlite3'
import { fileURLToPath } from 'url'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

const dataDir = path.join(__dirname, '..', 'data')
fs.mkdirSync(dataDir, { recursive: true })

const dbPath = path.join(dataDir, 'ops-config-platform.sqlite')

export const db = new Database(dbPath)

export function initDb(): void {
  db.pragma('journal_mode = WAL')
  db.exec(`
    CREATE TABLE IF NOT EXISTS activities (
      id TEXT PRIMARY KEY,
      name TEXT NOT NULL,
      description TEXT NOT NULL DEFAULT '',
      status TEXT NOT NULL,
      created_at TEXT NOT NULL,
      updated_at TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS activity_configs (
      activity_id TEXT PRIMARY KEY,
      audience_json TEXT NOT NULL,
      benefits_json TEXT NOT NULL,
      rules_json TEXT NOT NULL,
      orchestration_json TEXT NOT NULL,
      FOREIGN KEY(activity_id) REFERENCES activities(id)
    );

    CREATE TABLE IF NOT EXISTS audit_logs (
      id TEXT PRIMARY KEY,
      activity_id TEXT NOT NULL,
      actor TEXT NOT NULL,
      action TEXT NOT NULL,
      diff_json TEXT NOT NULL,
      created_at TEXT NOT NULL,
      FOREIGN KEY(activity_id) REFERENCES activities(id)
    );

    CREATE TABLE IF NOT EXISTS snapshots (
      id TEXT PRIMARY KEY,
      activity_id TEXT NOT NULL,
      config_json TEXT NOT NULL,
      created_at TEXT NOT NULL,
      FOREIGN KEY(activity_id) REFERENCES activities(id)
    );

    CREATE TABLE IF NOT EXISTS metric_snapshots (
      activity_id TEXT NOT NULL,
      date TEXT NOT NULL,
      participants INTEGER NOT NULL DEFAULT 0,
      conversions INTEGER NOT NULL DEFAULT 0,
      touch_success INTEGER NOT NULL DEFAULT 0,
      touch_fail INTEGER NOT NULL DEFAULT 0,
      PRIMARY KEY(activity_id, date),
      FOREIGN KEY(activity_id) REFERENCES activities(id)
    );

    CREATE INDEX IF NOT EXISTS idx_audit_logs_activity_id_created_at
      ON audit_logs(activity_id, created_at);

    CREATE INDEX IF NOT EXISTS idx_snapshots_activity_id_created_at
      ON snapshots(activity_id, created_at);
  `)
}

