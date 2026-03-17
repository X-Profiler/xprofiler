import { useEffect, useMemo, useState } from 'react'
import { useParams } from 'react-router-dom'
import { apiFetch } from '@/api/http'
import type { ActivityFull } from '@/types'
import { actorHeaderValue, useAuthStore } from '@/stores/auth'
import ActivityNav from '@/components/ActivityNav'
import Button from '@/components/ui/Button'
import Badge from '@/components/ui/Badge'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card'
import JsonPreview from '@/components/JsonPreview'
import { History, RotateCcw, FileClock } from 'lucide-react'

function safeParse(s: string): any {
  try {
    return JSON.parse(s)
  } catch {
    return s
  }
}

export default function Logs() {
  const { id = '' } = useParams()
  const auth = useAuthStore()
  const actor = actorHeaderValue(auth)

  const [activity, setActivity] = useState<ActivityFull | null>(null)
  const [logs, setLogs] = useState<any[]>([])
  const [snaps, setSnaps] = useState<any[]>([])
  const [error, setError] = useState<string | null>(null)
  const [selected, setSelected] = useState<any | null>(null)

  const load = async () => {
    const a = await apiFetch<ActivityFull>(`/api/activities/${id}`, { actor })
    const l = await apiFetch<any[]>(`/api/activities/${id}/audit-logs`, { actor })
    const s = await apiFetch<any[]>(`/api/activities/${id}/snapshots`, { actor })
    setActivity(a)
    setLogs(l)
    setSnaps(s)
  }

  useEffect(() => {
    load().catch(() => {})
  }, [id])

  const selectedJson = useMemo(() => {
    if (!selected) return null
    return {
      ...selected,
      diff: typeof selected.diff_json === 'string' ? safeParse(selected.diff_json) : selected.diff_json,
    }
  }, [selected])

  if (!activity) {
    return (
      <div className="rounded-2xl border border-zinc-900 bg-zinc-950/20 p-10 text-center text-sm text-zinc-400">
        加载中…
      </div>
    )
  }

  return (
    <div className="space-y-6">
      <div className="flex flex-wrap items-start justify-between gap-4">
        <div>
          <div className="text-xs font-semibold tracking-[0.2em] text-zinc-400">HISTORY</div>
          <div className="mt-2 flex items-center gap-2 text-2xl font-semibold">
            <History className="h-6 w-6 text-cyan-200" />
            状态与历史
          </div>
          <div className="mt-4">
            <ActivityNav id={activity.id} />
          </div>
        </div>
        <div className="flex items-center gap-2">
          <Button variant="ghost" onClick={() => load()}>
            刷新
          </Button>
        </div>
      </div>

      {error ? <div className="text-sm text-rose-200">{error}</div> : null}

      <div className="grid grid-cols-1 gap-4 lg:grid-cols-3">
        <Card className="lg:col-span-2">
          <CardHeader>
            <CardTitle>操作日志</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-2">
              {logs.map((l) => (
                <button
                  key={l.id}
                  className="w-full rounded-2xl border border-zinc-900 bg-zinc-950/20 p-4 text-left transition hover:border-zinc-800 hover:bg-zinc-950/30"
                  onClick={() => setSelected(l)}
                >
                  <div className="flex flex-wrap items-center justify-between gap-2">
                    <div className="text-sm font-medium text-zinc-100">{l.action}</div>
                    <Badge tone="neutral">{l.actor}</Badge>
                  </div>
                  <div className="mt-2 text-xs text-zinc-500">{l.created_at}</div>
                </button>
              ))}
              {logs.length === 0 ? (
                <div className="rounded-2xl border border-zinc-900 bg-zinc-950/20 p-10 text-center text-sm text-zinc-400">
                  暂无日志
                </div>
              ) : null}
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>配置快照</CardTitle>
          </CardHeader>
          <CardContent className="space-y-2">
            {snaps.map((s) => (
              <div
                key={s.id}
                className="rounded-2xl border border-zinc-900 bg-zinc-950/20 p-4"
              >
                <div className="flex items-center justify-between gap-2">
                  <div className="min-w-0">
                    <div className="flex items-center gap-2 text-sm font-medium text-zinc-100">
                      <FileClock className="h-4 w-4 text-zinc-400" />
                      <span className="truncate">{s.created_at}</span>
                    </div>
                    <div className="mt-1 truncate text-xs text-zinc-500">{s.id}</div>
                  </div>
                  <Button
                    size="sm"
                    variant="ghost"
                    onClick={async () => {
                      setError(null)
                      try {
                        await apiFetch(`/api/activities/${activity.id}/rollback`, {
                          method: 'POST',
                          actor,
                          body: JSON.stringify({ snapshotId: s.id }),
                        })
                        await load()
                      } catch (e: any) {
                        setError(e?.message ?? '回滚失败')
                      }
                    }}
                  >
                    <RotateCcw className="h-4 w-4" />
                  </Button>
                </div>
              </div>
            ))}
            {snaps.length === 0 ? (
              <div className="rounded-2xl border border-zinc-900 bg-zinc-950/20 p-10 text-center text-sm text-zinc-400">
                暂无快照
              </div>
            ) : null}
          </CardContent>
        </Card>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>日志详情</CardTitle>
        </CardHeader>
        <CardContent>
          {selectedJson ? (
            <JsonPreview value={selectedJson} />
          ) : (
            <div className="rounded-2xl border border-zinc-900 bg-zinc-950/20 p-10 text-center text-sm text-zinc-400">
              选择一条日志查看 diff
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  )
}

