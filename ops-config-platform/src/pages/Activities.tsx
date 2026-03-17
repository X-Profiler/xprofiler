import { useEffect, useMemo, useState } from 'react'
import { Link, useNavigate } from 'react-router-dom'
import { apiFetch } from '@/api/http'
import type { Activity, ActivityStatus } from '@/types'
import { actorHeaderValue, useAuthStore } from '@/stores/auth'
import Button from '@/components/ui/Button'
import Input from '@/components/ui/Input'
import Badge from '@/components/ui/Badge'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card'
import { Plus, ArrowRight } from 'lucide-react'

function toneByStatus(s: ActivityStatus) {
  if (s === 'online') return 'green'
  if (s === 'testing') return 'cyan'
  if (s === 'offline') return 'amber'
  if (s === 'archived') return 'rose'
  return 'neutral'
}

export default function Activities() {
  const navigate = useNavigate()
  const auth = useAuthStore()
  const actor = actorHeaderValue(auth)
  const [items, setItems] = useState<Activity[]>([])
  const [q, setQ] = useState('')
  const [creating, setCreating] = useState(false)
  const [name, setName] = useState('')
  const [error, setError] = useState<string | null>(null)

  const filtered = useMemo(() => {
    const s = q.trim().toLowerCase()
    if (!s) return items
    return items.filter((x) => x.name.toLowerCase().includes(s) || x.id.toLowerCase().includes(s))
  }, [items, q])

  const load = async () => {
    const data = await apiFetch<Activity[]>('/api/activities', { actor })
    setItems(data)
  }

  useEffect(() => {
    load().catch(() => {})
  }, [])

  return (
    <div className="space-y-6">
      <div className="flex flex-wrap items-end justify-between gap-3">
        <div>
          <div className="text-xs font-semibold tracking-[0.2em] text-zinc-400">
            ACTIVITIES
          </div>
          <div className="mt-2 text-2xl font-semibold text-zinc-100">活动工作台</div>
          <div className="mt-2 text-sm text-zinc-400">
            草稿 → 测试 → 上线 → 下线 → 归档；配置变更自动留痕与快照。
          </div>
        </div>
        <div className="flex items-center gap-2">
          <Input className="w-[280px]" placeholder="搜索活动名称 / ID" value={q} onChange={(e) => setQ(e.target.value)} />
          <Button variant="primary" onClick={() => setCreating(true)}>
            <Plus className="h-4 w-4" />
            新建
          </Button>
        </div>
      </div>

      {creating ? (
        <Card>
          <CardHeader>
            <CardTitle>新建活动</CardTitle>
          </CardHeader>
          <CardContent className="flex flex-wrap items-center gap-2">
            <div className="flex-1 min-w-[260px]">
              <Input placeholder="活动名称" value={name} onChange={(e) => setName(e.target.value)} />
            </div>
            <Button
              variant="primary"
              disabled={!name.trim()}
              onClick={async () => {
                setError(null)
                try {
                  const created = await apiFetch<{ id: string }>('/api/activities', {
                    method: 'POST',
                    actor,
                    body: JSON.stringify({ name }),
                  })
                  setCreating(false)
                  setName('')
                  await load()
                  navigate(`/activities/${created.id}`)
                } catch (e: any) {
                  setError(e?.message ?? '创建失败')
                }
              }}
            >
              创建并进入
            </Button>
            <Button variant="ghost" onClick={() => setCreating(false)}>
              取消
            </Button>
            {error ? <div className="w-full text-sm text-rose-200">{error}</div> : null}
          </CardContent>
        </Card>
      ) : null}

      <div className="grid grid-cols-1 gap-3">
        {filtered.map((a) => (
          <Link
            key={a.id}
            to={`/activities/${a.id}`}
            className="group rounded-2xl border border-zinc-900 bg-zinc-950/20 p-5 transition hover:border-zinc-800 hover:bg-zinc-950/35"
          >
            <div className="flex flex-wrap items-center justify-between gap-3">
              <div className="min-w-0">
                <div className="truncate text-lg font-semibold text-zinc-100">{a.name}</div>
                <div className="mt-1 truncate text-xs text-zinc-500">{a.id}</div>
                <div className="mt-2 text-sm text-zinc-400">
                  {a.description || '—'}
                </div>
              </div>
              <div className="flex items-center gap-2">
                <Badge tone={toneByStatus(a.status) as any}>{a.status}</Badge>
                <ArrowRight className="h-4 w-4 text-zinc-500 transition group-hover:text-zinc-300" />
              </div>
            </div>
          </Link>
        ))}
        {filtered.length === 0 ? (
          <div className="rounded-2xl border border-zinc-900 bg-zinc-950/20 p-10 text-center text-sm text-zinc-400">
            暂无活动
          </div>
        ) : null}
      </div>
    </div>
  )
}
