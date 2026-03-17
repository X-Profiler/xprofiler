import { db } from './db.js'
import { newId, nowIso } from './ids.js'

export type ActivityStatus = 'draft' | 'testing' | 'online' | 'offline' | 'archived'

export type ActivityRow = {
  id: string
  name: string
  description: string
  status: ActivityStatus
  created_at: string
  updated_at: string
}

export type ActivityConfigRow = {
  activity_id: string
  audience_json: string
  benefits_json: string
  rules_json: string
  orchestration_json: string
}

export type ActivityFull = ActivityRow & {
  config: {
    audience: unknown
    benefits: unknown
    rules: unknown
    orchestration: unknown
  }
}

function parseJson<T>(s: string): T {
  return JSON.parse(s) as T
}

function stringifyJson(v: unknown): string {
  return JSON.stringify(v ?? {})
}

export function listActivities(): ActivityRow[] {
  return db.prepare('SELECT * FROM activities ORDER BY updated_at DESC').all() as any
}

export function getActivity(id: string): ActivityFull | null {
  const a = db.prepare('SELECT * FROM activities WHERE id = ?').get(id) as ActivityRow | undefined
  if (!a) return null
  const c = db
    .prepare('SELECT * FROM activity_configs WHERE activity_id = ?')
    .get(id) as ActivityConfigRow | undefined
  if (!c) return null
  return {
    ...a,
    config: {
      audience: parseJson(c.audience_json),
      benefits: parseJson(c.benefits_json),
      rules: parseJson(c.rules_json),
      orchestration: parseJson(c.orchestration_json),
    },
  }
}

export function createActivity(input: {
  name: string
  description?: string
  actor: string
}): ActivityFull {
  const id = newId('act')
  const now = nowIso()
  const status: ActivityStatus = 'draft'
  const name = input.name.trim()
  const description = (input.description ?? '').trim()

  const tx = db.transaction(() => {
    db.prepare(
      'INSERT INTO activities (id, name, description, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)',
    ).run(id, name, description, status, now, now)

    db.prepare(
      'INSERT INTO activity_configs (activity_id, audience_json, benefits_json, rules_json, orchestration_json) VALUES (?, ?, ?, ?, ?)',
    ).run(id, '{}', '{}', '{}', '{}')

    insertSnapshot(id)
    insertAuditLog({
      activityId: id,
      actor: input.actor,
      action: 'activity.create',
      diff: { changed: ['activity', 'config'] },
    })
  })

  tx()

  const a = getActivity(id)
  if (!a) throw new Error('create failed')
  return a
}

export function updateActivityBase(input: {
  id: string
  patch: Partial<Pick<ActivityRow, 'name' | 'description'>>
  actor: string
}): ActivityFull {
  const before = getActivity(input.id)
  if (!before) throw new Error('not found')

  const name =
    typeof input.patch.name === 'string' ? input.patch.name.trim() : before.name
  const description =
    typeof input.patch.description === 'string'
      ? input.patch.description.trim()
      : before.description
  const now = nowIso()

  const tx = db.transaction(() => {
    db.prepare('UPDATE activities SET name = ?, description = ?, updated_at = ? WHERE id = ?').run(
      name,
      description,
      now,
      input.id,
    )
    insertSnapshot(input.id)
    insertAuditLog({
      activityId: input.id,
      actor: input.actor,
      action: 'activity.updateBase',
      diff: { changed: ['activity'] },
    })
  })
  tx()

  const after = getActivity(input.id)
  if (!after) throw new Error('update failed')
  return after
}

export function updateActivityStatus(input: {
  id: string
  next: ActivityStatus
  actor: string
}): ActivityFull {
  const before = getActivity(input.id)
  if (!before) throw new Error('not found')
  const allowed = isTransitionAllowed(before.status, input.next)
  if (!allowed) throw new Error('invalid transition')
  const now = nowIso()

  const tx = db.transaction(() => {
    db.prepare('UPDATE activities SET status = ?, updated_at = ? WHERE id = ?').run(
      input.next,
      now,
      input.id,
    )
    insertSnapshot(input.id)
    insertAuditLog({
      activityId: input.id,
      actor: input.actor,
      action: 'activity.lifecycle',
      diff: { changed: ['status'], from: before.status, to: input.next },
    })
  })
  tx()

  const after = getActivity(input.id)
  if (!after) throw new Error('update failed')
  return after
}

function isTransitionAllowed(from: ActivityStatus, to: ActivityStatus): boolean {
  if (from === to) return true
  const map: Record<ActivityStatus, ActivityStatus[]> = {
    draft: ['testing', 'archived'],
    testing: ['online', 'draft', 'archived'],
    online: ['offline'],
    offline: ['online', 'archived'],
    archived: [],
  }
  return map[from].includes(to)
}

export function updateActivityConfigSection(input: {
  id: string
  section: 'audience' | 'benefits' | 'rules' | 'orchestration'
  value: unknown
  actor: string
}): ActivityFull {
  const before = getActivity(input.id)
  if (!before) throw new Error('not found')
  if (before.status === 'online') throw new Error('activity is online')

  const col =
    input.section === 'audience'
      ? 'audience_json'
      : input.section === 'benefits'
        ? 'benefits_json'
        : input.section === 'rules'
          ? 'rules_json'
          : 'orchestration_json'

  const tx = db.transaction(() => {
    db.prepare(`UPDATE activity_configs SET ${col} = ? WHERE activity_id = ?`).run(
      stringifyJson(input.value),
      input.id,
    )
    db.prepare('UPDATE activities SET updated_at = ? WHERE id = ?').run(nowIso(), input.id)
    insertSnapshot(input.id)
    insertAuditLog({
      activityId: input.id,
      actor: input.actor,
      action: `config.update.${input.section}`,
      diff: { changed: [input.section] },
    })
  })
  tx()

  const after = getActivity(input.id)
  if (!after) throw new Error('update failed')
  return after
}

export function listAuditLogs(activityId: string): any[] {
  return db
    .prepare('SELECT * FROM audit_logs WHERE activity_id = ? ORDER BY created_at DESC')
    .all(activityId) as any
}

export function listSnapshots(activityId: string): any[] {
  return db
    .prepare('SELECT id, activity_id, created_at FROM snapshots WHERE activity_id = ? ORDER BY created_at DESC')
    .all(activityId) as any
}

export function getSnapshot(activityId: string, snapshotId: string): any | null {
  const row = db
    .prepare('SELECT * FROM snapshots WHERE activity_id = ? AND id = ?')
    .get(activityId, snapshotId) as any
  return row ?? null
}

export function rollbackToSnapshot(input: { activityId: string; snapshotId: string; actor: string }): ActivityFull {
  const snap = getSnapshot(input.activityId, input.snapshotId)
  if (!snap) throw new Error('snapshot not found')
  const config = parseJson<any>(snap.config_json)

  const tx = db.transaction(() => {
    db.prepare(
      'UPDATE activity_configs SET audience_json = ?, benefits_json = ?, rules_json = ?, orchestration_json = ? WHERE activity_id = ?',
    ).run(
      stringifyJson(config.audience),
      stringifyJson(config.benefits),
      stringifyJson(config.rules),
      stringifyJson(config.orchestration),
      input.activityId,
    )
    db.prepare('UPDATE activities SET updated_at = ? WHERE id = ?').run(nowIso(), input.activityId)
    insertSnapshot(input.activityId)
    insertAuditLog({
      activityId: input.activityId,
      actor: input.actor,
      action: 'snapshot.rollback',
      diff: { changed: ['audience', 'benefits', 'rules', 'orchestration'], snapshotId: input.snapshotId },
    })
  })
  tx()

  const after = getActivity(input.activityId)
  if (!after) throw new Error('rollback failed')
  return after
}

export function upsertMetricDelta(input: {
  activityId: string
  date: string
  participants?: number
  conversions?: number
  touchSuccess?: number
  touchFail?: number
}): void {
  const p = input.participants ?? 0
  const c = input.conversions ?? 0
  const s = input.touchSuccess ?? 0
  const f = input.touchFail ?? 0
  db.prepare(
    `
      INSERT INTO metric_snapshots (activity_id, date, participants, conversions, touch_success, touch_fail)
      VALUES (?, ?, ?, ?, ?, ?)
      ON CONFLICT(activity_id, date) DO UPDATE SET
        participants = participants + excluded.participants,
        conversions = conversions + excluded.conversions,
        touch_success = touch_success + excluded.touch_success,
        touch_fail = touch_fail + excluded.touch_fail
    `,
  ).run(input.activityId, input.date, p, c, s, f)
}

export function getActivityMetrics(activityId: string): any[] {
  return db
    .prepare(
      'SELECT date, participants, conversions, touch_success, touch_fail FROM metric_snapshots WHERE activity_id = ? ORDER BY date ASC',
    )
    .all(activityId) as any
}

export function getOverviewMetrics(): any {
  const row = db
    .prepare(
      `
        SELECT
          SUM(participants) as participants,
          SUM(conversions) as conversions,
          SUM(touch_success) as touchSuccess,
          SUM(touch_fail) as touchFail
        FROM metric_snapshots
      `,
    )
    .get() as any
  return {
    participants: row?.participants ?? 0,
    conversions: row?.conversions ?? 0,
    touchSuccess: row?.touchSuccess ?? 0,
    touchFail: row?.touchFail ?? 0,
  }
}

function insertAuditLog(input: {
  activityId: string
  actor: string
  action: string
  diff: unknown
}): void {
  db.prepare(
    'INSERT INTO audit_logs (id, activity_id, actor, action, diff_json, created_at) VALUES (?, ?, ?, ?, ?, ?)',
  ).run(newId('log'), input.activityId, input.actor, input.action, stringifyJson(input.diff), nowIso())
}

function insertSnapshot(activityId: string): void {
  const full = getActivity(activityId)
  if (!full) throw new Error('snapshot: activity not found')
  db.prepare('INSERT INTO snapshots (id, activity_id, config_json, created_at) VALUES (?, ?, ?, ?)').run(
    newId('snap'),
    activityId,
    stringifyJson(full.config),
    nowIso(),
  )
}

