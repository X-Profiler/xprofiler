import { useEffect, useMemo, useState } from 'react'
import { useParams } from 'react-router-dom'
import { apiFetch } from '@/api/http'
import type { ActivityFull, ActivityStatus } from '@/types'
import { actorHeaderValue, useAuthStore } from '@/stores/auth'
import ActivityNav from '@/components/ActivityNav'
import Button from '@/components/ui/Button'
import Input from '@/components/ui/Input'
import Textarea from '@/components/ui/Textarea'
import Badge from '@/components/ui/Badge'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card'
import { Save, Rocket, FlaskConical, PauseCircle, Archive, RefreshCcw } from 'lucide-react'

function toneByStatus(s: ActivityStatus) {
  if (s === 'online') return 'green'
  if (s === 'testing') return 'cyan'
  if (s === 'offline') return 'amber'
  if (s === 'archived') return 'rose'
  return 'neutral'
}

function nextButtons(status: ActivityStatus) {
  if (status === 'draft') return [{ next: 'testing', label: '进入测试', icon: FlaskConical }]
  if (status === 'testing')
    return [
      { next: 'online', label: '上线', icon: Rocket },
      { next: 'draft', label: '退回草稿', icon: RefreshCcw },
    ]
  if (status === 'online') return [{ next: 'offline', label: '下线', icon: PauseCircle }]
  if (status === 'offline')
    return [
      { next: 'online', label: '重新上线', icon: Rocket },
      { next: 'archived', label: '归档', icon: Archive },
    ]
  return []
}

export default function ActivityOverview() {
  const { id = '' } = useParams()
  const auth = useAuthStore()
  const actor = actorHeaderValue(auth)

  const [data, setData] = useState<ActivityFull | null>(null)
  const [name, setName] = useState('')
  const [description, setDescription] = useState('')
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const completeness = useMemo(() => {
    if (!data) return null
    const audience = data.config.audience
    const rules = data.config.rules
    const benefits = data.config.benefits
    const orchestration = data.config.orchestration
    return {
      audience:
        !!audience?.labels || !!audience?.behaviors || !!audience?.whitelist?.userIds?.length,
      rules: !!rules?.threshold || !!rules?.limit || !!rules?.stacking,
      benefits:
        !!benefits?.couponPacks?.length || !!benefits?.pointsRules?.length || !!benefits?.others?.length,
      orchestration: !!orchestration?.steps?.length,
    }
  }, [data])

  const load = async () => {
    const a = await apiFetch<ActivityFull>(`/api/activities/${id}`, { actor })
    setData(a)
    setName(a.name)
    setDescription(a.description)
  }

  useEffect(() => {
    load().catch(() => {})
  }, [id])

  if (!data) {
    return (
      <div className="rounded-2xl border border-zinc-900 bg-zinc-950/20 p-10 text-center text-sm text-zinc-400">
        加载中…
      </div>
    )
  }

  return (
    <div className="space-y-6">
      <div className="flex flex-wrap items-start justify-between gap-4">
        <div className="min-w-0">
          <div className="text-xs font-semibold tracking-[0.2em] text-zinc-400">
            ACTIVITY
          </div>
          <div className="mt-2 flex flex-wrap items-center gap-3">
            <div className="text-2xl font-semibold">{data.name}</div>
            <Badge tone={toneByStatus(data.status) as any}>{data.status}</Badge>
          </div>
          <div className="mt-2 text-xs text-zinc-500">{data.id}</div>
          <div className="mt-4">
            <ActivityNav id={data.id} />
          </div>
        </div>
        <div className="flex flex-wrap items-center gap-2">
          {nextButtons(data.status).map((b) => (
            <Button
              key={b.next}
              variant={b.next === 'online' ? 'primary' : 'secondary'}
              onClick={async () => {
                setError(null)
                try {
                  await apiFetch(`/api/activities/${data.id}/lifecycle`, {
                    method: 'POST',
                    actor,
                    body: JSON.stringify({ next: b.next }),
                  })
                  await load()
                } catch (e: any) {
                  setError(e?.message ?? '操作失败')
                }
              }}
            >
              <b.icon className="h-4 w-4" />
              {b.label}
            </Button>
          ))}
        </div>
      </div>

      <div className="grid grid-cols-1 gap-4 lg:grid-cols-2">
        <Card>
          <CardHeader>
            <CardTitle>基本信息</CardTitle>
          </CardHeader>
          <CardContent className="space-y-3">
            <div className="space-y-2">
              <div className="text-xs font-medium text-zinc-300">名称</div>
              <Input value={name} onChange={(e) => setName(e.target.value)} />
            </div>
            <div className="space-y-2">
              <div className="text-xs font-medium text-zinc-300">描述</div>
              <Textarea
                className="min-h-[110px]"
                value={description}
                onChange={(e) => setDescription(e.target.value)}
              />
            </div>
            {error ? <div className="text-sm text-rose-200">{error}</div> : null}
            <div className="flex items-center gap-2">
              <Button
                variant="primary"
                disabled={saving || !name.trim()}
                onClick={async () => {
                  setSaving(true)
                  setError(null)
                  try {
                    await apiFetch(`/api/activities/${data.id}`, {
                      method: 'PATCH',
                      actor,
                      body: JSON.stringify({ name, description }),
                    })
                    await load()
                  } catch (e: any) {
                    setError(e?.message ?? '保存失败')
                  } finally {
                    setSaving(false)
                  }
                }}
              >
                <Save className="h-4 w-4" />
                保存
              </Button>
              <Button variant="ghost" onClick={() => load()}>
                刷新
              </Button>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>配置就绪度</CardTitle>
          </CardHeader>
          <CardContent className="space-y-3">
            <div className="grid grid-cols-2 gap-3">
              <div className="rounded-2xl border border-zinc-900 bg-zinc-950/20 p-4">
                <div className="text-xs font-medium text-zinc-400">人群</div>
                <div className="mt-2">
                  <Badge tone={completeness?.audience ? 'green' : 'neutral'}>
                    {completeness?.audience ? '已配置' : '未配置'}
                  </Badge>
                </div>
              </div>
              <div className="rounded-2xl border border-zinc-900 bg-zinc-950/20 p-4">
                <div className="text-xs font-medium text-zinc-400">权益</div>
                <div className="mt-2">
                  <Badge tone={completeness?.benefits ? 'green' : 'neutral'}>
                    {completeness?.benefits ? '已配置' : '未配置'}
                  </Badge>
                </div>
              </div>
              <div className="rounded-2xl border border-zinc-900 bg-zinc-950/20 p-4">
                <div className="text-xs font-medium text-zinc-400">规则</div>
                <div className="mt-2">
                  <Badge tone={completeness?.rules ? 'green' : 'neutral'}>
                    {completeness?.rules ? '已配置' : '未配置'}
                  </Badge>
                </div>
              </div>
              <div className="rounded-2xl border border-zinc-900 bg-zinc-950/20 p-4">
                <div className="text-xs font-medium text-zinc-400">触达</div>
                <div className="mt-2">
                  <Badge tone={completeness?.orchestration ? 'green' : 'neutral'}>
                    {completeness?.orchestration ? '已配置' : '未配置'}
                  </Badge>
                </div>
              </div>
            </div>
            <div className="text-xs leading-5 text-zinc-400">
              上线态默认锁定配置；建议先在测试态用白名单做小流量验证，然后再上线。
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}

