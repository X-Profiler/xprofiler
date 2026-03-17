import { useEffect, useMemo, useState } from 'react'
import { useParams } from 'react-router-dom'
import { apiFetch } from '@/api/http'
import type { ActivityFull, ConditionGroup, RuleSet } from '@/types'
import { actorHeaderValue, useAuthStore } from '@/stores/auth'
import ActivityNav from '@/components/ActivityNav'
import ConditionBuilder from '@/components/conditions/ConditionBuilder'
import Button from '@/components/ui/Button'
import Input from '@/components/ui/Input'
import Select from '@/components/ui/Select'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card'
import JsonPreview from '@/components/JsonPreview'
import { Gavel, Save, ShieldAlert, Wand2 } from 'lucide-react'

const THRESHOLD_FIELDS = [
  { key: 'consumeAmount', label: '消费金额', valueType: 'number' as const, placeholder: '200' },
  { key: 'consumeCount', label: '消费次数', valueType: 'number' as const, placeholder: '2' },
  { key: 'isNewUser', label: '是否新客', valueType: 'boolean' as const, placeholder: 'true/false' },
]

function emptyGroup(): ConditionGroup {
  return { logic: 'AND', children: [] }
}

function summarize(r: RuleSet): string {
  const parts: string[] = []
  if (r.threshold?.children?.length) parts.push('已配置门槛')
  if (r.limit) parts.push(`限次: ${r.limit.perUser}/${r.limit.period}`)
  if (r.stacking) parts.push(`叠加: ${r.stacking.mode}`)
  return parts.length ? parts.join(' · ') : '未配置规则'
}

export default function Rules() {
  const { id = '' } = useParams()
  const auth = useAuthStore()
  const actor = actorHeaderValue(auth)

  const [activity, setActivity] = useState<ActivityFull | null>(null)
  const [rules, setRules] = useState<RuleSet>({})
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [validateMsg, setValidateMsg] = useState<string | null>(null)

  const load = async () => {
    const a = await apiFetch<ActivityFull>(`/api/activities/${id}`, { actor })
    setActivity(a)
    setRules((a.config.rules ?? {}) as RuleSet)
  }

  useEffect(() => {
    load().catch(() => {})
  }, [id])

  const locked = activity?.status === 'online'

  const normalized = useMemo(() => {
    return {
      threshold: rules.threshold,
      stacking: rules.stacking ?? { mode: 'exclusive' as const, priority: 100 },
      limit: rules.limit ?? { perUser: 1, period: 'lifecycle' as const },
    } satisfies RuleSet
  }, [rules])

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
          <div className="text-xs font-semibold tracking-[0.2em] text-zinc-400">RULES</div>
          <div className="mt-2 flex items-center gap-2 text-2xl font-semibold">
            <Gavel className="h-6 w-6 text-cyan-200" />
            规则引擎
          </div>
          <div className="mt-2 text-sm text-zinc-400">{summarize(rules)}</div>
          <div className="mt-4">
            <ActivityNav id={activity.id} />
          </div>
        </div>
        <div className="flex flex-wrap items-center gap-2">
          <Button
            variant="secondary"
            disabled={!normalized.threshold}
            onClick={async () => {
              setValidateMsg(null)
              setError(null)
              try {
                if (!normalized.threshold) return
                await apiFetch('/api/rules/validate', {
                  method: 'POST',
                  actor,
                  body: JSON.stringify(normalized.threshold),
                })
                setValidateMsg('校验通过')
              } catch (e: any) {
                setError(e?.message ?? '校验失败')
              }
            }}
          >
            <Wand2 className="h-4 w-4" />
            校验
          </Button>
          <Button
            variant="primary"
            disabled={locked || saving}
            onClick={async () => {
              setSaving(true)
              setError(null)
              setValidateMsg(null)
              try {
                const updated = await apiFetch<ActivityFull>(`/api/activities/${activity.id}/rules`, {
                  method: 'PUT',
                  actor,
                  body: JSON.stringify(normalized),
                })
                setActivity(updated)
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
        </div>
      </div>

      {locked ? (
        <div className="rounded-2xl border border-amber-400/20 bg-amber-400/10 p-4 text-sm text-amber-100">
          活动处于上线态，配置已锁定。如需修改，请先下线。
        </div>
      ) : null}

      {error ? (
        <div className="flex items-start gap-2 rounded-2xl border border-rose-400/20 bg-rose-400/10 p-4 text-sm text-rose-100">
          <ShieldAlert className="mt-0.5 h-4 w-4" />
          <div>{error}</div>
        </div>
      ) : null}
      {validateMsg ? (
        <div className="rounded-2xl border border-emerald-400/20 bg-emerald-400/10 p-4 text-sm text-emerald-100">
          {validateMsg}
        </div>
      ) : null}

      <Card>
        <CardHeader>
          <CardTitle>门槛规则</CardTitle>
        </CardHeader>
        <CardContent className="space-y-3">
          <label className="flex items-center gap-2 text-sm text-zinc-200">
            <input
              type="checkbox"
              checked={!!rules.threshold}
              onChange={(e) =>
                setRules((s) => ({
                  ...s,
                  threshold: e.target.checked ? { logic: 'AND', children: [] } : undefined,
                }))
              }
            />
            启用门槛
          </label>
          {rules.threshold ? (
            <ConditionBuilder
              fields={THRESHOLD_FIELDS}
              value={rules.threshold ?? emptyGroup()}
              onChange={(v) => setRules((s) => ({ ...s, threshold: v }))}
            />
          ) : (
            <div className="text-sm text-zinc-400">未启用</div>
          )}
        </CardContent>
      </Card>

      <div className="grid grid-cols-1 gap-4 lg:grid-cols-2">
        <Card>
          <CardHeader>
            <CardTitle>叠加规则</CardTitle>
          </CardHeader>
          <CardContent className="space-y-3">
            <div className="grid grid-cols-12 gap-2">
              <div className="col-span-7">
                <div className="text-xs text-zinc-500">模式</div>
                <Select
                  disabled={locked}
                  value={normalized.stacking?.mode ?? 'exclusive'}
                  onChange={(e) =>
                    setRules((s) => ({
                      ...s,
                      stacking: { ...(s.stacking ?? {}), mode: e.target.value as any },
                    }))
                  }
                >
                  <option value="exclusive">互斥（默认）</option>
                  <option value="allow">允许叠加</option>
                </Select>
              </div>
              <div className="col-span-5">
                <div className="text-xs text-zinc-500">优先级</div>
                <Input
                  disabled={locked}
                  inputMode="numeric"
                  value={String(normalized.stacking?.priority ?? 100)}
                  onChange={(e) =>
                    setRules((s) => ({
                      ...s,
                      stacking: { ...(s.stacking ?? { mode: 'exclusive' }), priority: Number(e.target.value || 0) },
                    }))
                  }
                />
              </div>
            </div>
            <div className="text-xs leading-5 text-zinc-400">
              互斥：命中优先级更高者；允许叠加：多活动可同时命中（需配合限次与权益冲突策略）。
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>限次规则</CardTitle>
          </CardHeader>
          <CardContent className="space-y-3">
            <div className="grid grid-cols-12 gap-2">
              <div className="col-span-5">
                <div className="text-xs text-zinc-500">次数</div>
                <Input
                  disabled={locked}
                  inputMode="numeric"
                  value={String(normalized.limit?.perUser ?? 1)}
                  onChange={(e) =>
                    setRules((s) => ({
                      ...s,
                      limit: { ...(s.limit ?? { period: 'lifecycle' }), perUser: Number(e.target.value || 0) },
                    }))
                  }
                />
              </div>
              <div className="col-span-7">
                <div className="text-xs text-zinc-500">周期</div>
                <Select
                  disabled={locked}
                  value={normalized.limit?.period ?? 'lifecycle'}
                  onChange={(e) =>
                    setRules((s) => ({
                      ...s,
                      limit: { ...(s.limit ?? { perUser: 1 }), period: e.target.value as any },
                    }))
                  }
                >
                  <option value="day">日</option>
                  <option value="week">周</option>
                  <option value="month">月</option>
                  <option value="lifecycle">活动周期</option>
                </Select>
              </div>
            </div>
            <div className="text-xs leading-5 text-zinc-400">
              建议将限次与 AB 分桶共同纳入口径，避免用户跨版本重复触达。
            </div>
          </CardContent>
        </Card>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>规则 JSON 预览</CardTitle>
        </CardHeader>
        <CardContent>
          <JsonPreview value={normalized} />
        </CardContent>
      </Card>
    </div>
  )
}

