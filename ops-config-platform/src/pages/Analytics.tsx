import { useEffect, useMemo, useState } from 'react'
import { apiFetch } from '@/api/http'
import { actorHeaderValue, useAuthStore } from '@/stores/auth'
import type { Activity } from '@/types'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card'
import Select from '@/components/ui/Select'
import Badge from '@/components/ui/Badge'
import { BarChart3 } from 'lucide-react'

type Overview = {
  participants: number
  conversions: number
  touchSuccess: number
  touchFail: number
}

export default function Analytics() {
  const auth = useAuthStore()
  const actor = actorHeaderValue(auth)

  const [overview, setOverview] = useState<Overview | null>(null)
  const [activities, setActivities] = useState<Activity[]>([])
  const [activityId, setActivityId] = useState<string>('')
  const [series, setSeries] = useState<any[]>([])

  const load = async () => {
    const [o, a] = await Promise.all([
      apiFetch<Overview>('/api/analytics/overview', { actor }),
      apiFetch<Activity[]>('/api/activities', { actor }),
    ])
    setOverview(o)
    setActivities(a)
    if (!activityId && a[0]) setActivityId(a[0].id)
  }

  useEffect(() => {
    load().catch(() => {})
  }, [])

  useEffect(() => {
    if (!activityId) return
    apiFetch<any[]>(`/api/analytics/activities/${activityId}`, { actor })
      .then(setSeries)
      .catch(() => setSeries([]))
  }, [activityId])

  const derived = useMemo(() => {
    const p = overview?.participants ?? 0
    const c = overview?.conversions ?? 0
    const rate = p > 0 ? c / p : 0
    return { rate }
  }, [overview])

  return (
    <div className="space-y-6">
      <div>
        <div className="text-xs font-semibold tracking-[0.2em] text-zinc-400">ANALYTICS</div>
        <div className="mt-2 flex items-center gap-2 text-2xl font-semibold">
          <BarChart3 className="h-6 w-6 text-cyan-200" />
          效果数据看板
        </div>
        <div className="mt-2 text-sm text-zinc-400">
          MVP 指标为演示口径，后续可接埋点/数据仓库补齐真实参与与转化。
        </div>
      </div>

      <div className="grid grid-cols-1 gap-4 lg:grid-cols-4">
        <Card>
          <CardHeader>
            <CardTitle>参与</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-3xl font-semibold">{overview?.participants ?? 0}</div>
          </CardContent>
        </Card>
        <Card>
          <CardHeader>
            <CardTitle>转化</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-3xl font-semibold">{overview?.conversions ?? 0}</div>
            <div className="mt-2 text-xs text-zinc-400">
              转化率: <span className="text-zinc-200">{(derived.rate * 100).toFixed(2)}%</span>
            </div>
          </CardContent>
        </Card>
        <Card>
          <CardHeader>
            <CardTitle>触达成功</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-3xl font-semibold">{overview?.touchSuccess ?? 0}</div>
          </CardContent>
        </Card>
        <Card>
          <CardHeader>
            <CardTitle>触达失败</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-3xl font-semibold">{overview?.touchFail ?? 0}</div>
          </CardContent>
        </Card>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>活动维度</CardTitle>
        </CardHeader>
        <CardContent className="space-y-3">
          <div className="grid grid-cols-12 gap-2">
            <div className="col-span-12 lg:col-span-6">
              <div className="text-xs text-zinc-500">选择活动</div>
              <Select value={activityId} onChange={(e) => setActivityId(e.target.value)}>
                {activities.map((a) => (
                  <option key={a.id} value={a.id}>
                    {a.name}
                  </option>
                ))}
              </Select>
            </div>
            <div className="col-span-12 lg:col-span-6">
              <div className="text-xs text-zinc-500">状态</div>
              <div className="mt-2">
                <Badge tone="neutral">
                  {activities.find((x) => x.id === activityId)?.status ?? '—'}
                </Badge>
              </div>
            </div>
          </div>

          <div className="overflow-hidden rounded-2xl border border-zinc-900">
            <table className="w-full text-left text-sm">
              <thead className="bg-zinc-950/40 text-xs text-zinc-400">
                <tr>
                  <th className="px-4 py-3">日期</th>
                  <th className="px-4 py-3">参与</th>
                  <th className="px-4 py-3">转化</th>
                  <th className="px-4 py-3">触达成功</th>
                  <th className="px-4 py-3">触达失败</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-zinc-900 bg-zinc-950/20">
                {series.map((r) => (
                  <tr key={r.date}>
                    <td className="px-4 py-3 text-zinc-200">{r.date}</td>
                    <td className="px-4 py-3 text-zinc-200">{r.participants}</td>
                    <td className="px-4 py-3 text-zinc-200">{r.conversions}</td>
                    <td className="px-4 py-3 text-zinc-200">{r.touch_success}</td>
                    <td className="px-4 py-3 text-zinc-200">{r.touch_fail}</td>
                  </tr>
                ))}
                {series.length === 0 ? (
                  <tr>
                    <td className="px-4 py-8 text-center text-zinc-400" colSpan={5}>
                      暂无指标数据（可在触达干跑后手工补齐演示数据）
                    </td>
                  </tr>
                ) : null}
              </tbody>
            </table>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}

