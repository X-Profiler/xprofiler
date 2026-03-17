import { useEffect, useMemo, useState } from 'react'
import { useParams } from 'react-router-dom'
import { apiFetch } from '@/api/http'
import type { ActivityFull, OrchestrationPlan, TouchChannel, TouchStep } from '@/types'
import { actorHeaderValue, useAuthStore } from '@/stores/auth'
import ActivityNav from '@/components/ActivityNav'
import Button from '@/components/ui/Button'
import Input from '@/components/ui/Input'
import Select from '@/components/ui/Select'
import Textarea from '@/components/ui/Textarea'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card'
import JsonPreview from '@/components/JsonPreview'
import { Send, Plus, Save, Trash2, FlaskConical } from 'lucide-react'

function uid(prefix: string) {
  return `${prefix}_${crypto.randomUUID()}`
}

function defaultStep(): TouchStep {
  return {
    id: uid('step'),
    channel: 'inbox',
    templateId: 'tmpl_inbox_001',
    schedule: { type: 'immediate' },
    frequency: { maxPerUser: 1, period: 'day' },
    ab: { enabled: false, buckets: [{ name: 'A', ratio: 50, templateId: 'tmpl_inbox_001' }, { name: 'B', ratio: 50, templateId: 'tmpl_inbox_002' }] },
  }
}

export default function Orchestration() {
  const { id = '' } = useParams()
  const auth = useAuthStore()
  const actor = actorHeaderValue(auth)

  const [activity, setActivity] = useState<ActivityFull | null>(null)
  const [plan, setPlan] = useState<OrchestrationPlan>({ steps: [] })
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [dryRunResult, setDryRunResult] = useState<any>(null)
  const [sampleUserJson, setSampleUserJson] = useState(
    JSON.stringify({ consumeAmount: 200, consumeCount: 2, isNewUser: true }, null, 2),
  )

  const load = async () => {
    const a = await apiFetch<ActivityFull>(`/api/activities/${id}`, { actor })
    setActivity(a)
    const cfg = (a.config.orchestration ?? {}) as OrchestrationPlan
    setPlan({ steps: cfg.steps ?? [] })
  }

  useEffect(() => {
    load().catch(() => {})
  }, [id])

  const locked = activity?.status === 'online'

  const normalized = useMemo(() => {
    return { steps: plan.steps ?? [] } satisfies OrchestrationPlan
  }, [plan])

  const updateStep = (idx: number, patch: Partial<TouchStep>) => {
    setPlan((s) => {
      const next = structuredClone(s) as OrchestrationPlan
      next.steps[idx] = { ...next.steps[idx], ...patch }
      return next
    })
  }

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
          <div className="text-xs font-semibold tracking-[0.2em] text-zinc-400">
            ORCHESTRATION
          </div>
          <div className="mt-2 flex items-center gap-2 text-2xl font-semibold">
            <Send className="h-6 w-6 text-cyan-200" />
            触达编排
          </div>
          <div className="mt-4">
            <ActivityNav id={activity.id} />
          </div>
        </div>
        <div className="flex flex-wrap items-center gap-2">
          <Button
            variant="secondary"
            onClick={() =>
              setPlan((s) => ({ steps: [...(s.steps ?? []), defaultStep()] }))
            }
            disabled={locked}
          >
            <Plus className="h-4 w-4" />
            添加步骤
          </Button>
          <Button
            variant="primary"
            disabled={locked || saving}
            onClick={async () => {
              setSaving(true)
              setError(null)
              try {
                const updated = await apiFetch<ActivityFull>(`/api/activities/${activity.id}/orchestration`, {
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

      {error ? <div className="text-sm text-rose-200">{error}</div> : null}

      <div className="space-y-3">
        {(normalized.steps ?? []).map((st, idx) => (
          <Card key={st.id}>
            <CardHeader>
              <CardTitle>步骤 {idx + 1}</CardTitle>
            </CardHeader>
            <CardContent className="space-y-3">
              <div className="grid grid-cols-12 gap-2">
                <div className="col-span-4">
                  <div className="text-xs text-zinc-500">渠道</div>
                  <Select
                    disabled={locked}
                    value={st.channel}
                    onChange={(e) => updateStep(idx, { channel: e.target.value as TouchChannel })}
                  >
                    <option value="inbox">站内信</option>
                    <option value="sms">短信</option>
                    <option value="im">IM（企微/钉钉）</option>
                  </Select>
                </div>
                <div className="col-span-8">
                  <div className="text-xs text-zinc-500">模板 ID</div>
                  <Input
                    disabled={locked}
                    value={st.templateId}
                    onChange={(e) => updateStep(idx, { templateId: e.target.value })}
                  />
                </div>
              </div>

              <div className="grid grid-cols-12 gap-2">
                <div className="col-span-5">
                  <div className="text-xs text-zinc-500">发送时间</div>
                  <Select
                    disabled={locked}
                    value={st.schedule.type}
                    onChange={(e) =>
                      updateStep(idx, {
                        schedule: { type: e.target.value as any, value: '' },
                      })
                    }
                  >
                    <option value="immediate">立即</option>
                    <option value="time_window">时间窗</option>
                    <option value="cron">Cron</option>
                  </Select>
                </div>
                <div className="col-span-7">
                  <div className="text-xs text-zinc-500">参数</div>
                  <Input
                    disabled={locked || st.schedule.type === 'immediate'}
                    placeholder={
                      st.schedule.type === 'time_window'
                        ? '09:00-21:00'
                        : st.schedule.type === 'cron'
                          ? '0 10 * * *'
                          : ''
                    }
                    value={st.schedule.value ?? ''}
                    onChange={(e) =>
                      updateStep(idx, { schedule: { ...st.schedule, value: e.target.value } })
                    }
                  />
                </div>
              </div>

              <div className="grid grid-cols-12 gap-2">
                <div className="col-span-5">
                  <div className="text-xs text-zinc-500">单用户频次</div>
                  <Input
                    disabled={locked}
                    inputMode="numeric"
                    value={String(st.frequency.maxPerUser)}
                    onChange={(e) =>
                      updateStep(idx, {
                        frequency: { ...st.frequency, maxPerUser: Number(e.target.value || 0) },
                      })
                    }
                  />
                </div>
                <div className="col-span-7">
                  <div className="text-xs text-zinc-500">周期</div>
                  <Select
                    disabled={locked}
                    value={st.frequency.period}
                    onChange={(e) =>
                      updateStep(idx, {
                        frequency: { ...st.frequency, period: e.target.value as any },
                      })
                    }
                  >
                    <option value="day">日</option>
                    <option value="week">周</option>
                    <option value="month">月</option>
                  </Select>
                </div>
              </div>

              <div className="rounded-2xl border border-zinc-900 bg-zinc-950/20 p-4">
                <div className="flex flex-wrap items-center justify-between gap-2">
                  <label className="flex items-center gap-2 text-sm text-zinc-200">
                    <input
                      type="checkbox"
                      checked={!!st.ab?.enabled}
                      onChange={(e) =>
                        updateStep(idx, {
                          ab: { ...(st.ab ?? { buckets: [] }), enabled: e.target.checked },
                        })
                      }
                      disabled={locked}
                    />
                    AB 测试
                  </label>
                  <Button
                    size="sm"
                    variant="ghost"
                    disabled={locked}
                    onClick={() =>
                      setPlan((s) => ({
                        steps: (s.steps ?? []).filter((x) => x.id !== st.id),
                      }))
                    }
                  >
                    <Trash2 className="h-4 w-4" />
                    删除步骤
                  </Button>
                </div>

                {st.ab?.enabled ? (
                  <div className="mt-3 space-y-2">
                    {(st.ab.buckets ?? []).map((b, bi) => (
                      <div key={`${st.id}_${bi}`} className="grid grid-cols-12 gap-2">
                        <div className="col-span-3">
                          <Input
                            disabled={locked}
                            value={b.name}
                            onChange={(e) =>
                              updateStep(idx, {
                                ab: {
                                  enabled: true,
                                  buckets: st.ab!.buckets.map((x, i) =>
                                    i === bi ? { ...x, name: e.target.value } : x,
                                  ),
                                },
                              })
                            }
                          />
                        </div>
                        <div className="col-span-3">
                          <Input
                            disabled={locked}
                            inputMode="numeric"
                            value={String(b.ratio)}
                            onChange={(e) =>
                              updateStep(idx, {
                                ab: {
                                  enabled: true,
                                  buckets: st.ab!.buckets.map((x, i) =>
                                    i === bi ? { ...x, ratio: Number(e.target.value || 0) } : x,
                                  ),
                                },
                              })
                            }
                          />
                        </div>
                        <div className="col-span-6">
                          <Input
                            disabled={locked}
                            value={b.templateId}
                            onChange={(e) =>
                              updateStep(idx, {
                                ab: {
                                  enabled: true,
                                  buckets: st.ab!.buckets.map((x, i) =>
                                    i === bi ? { ...x, templateId: e.target.value } : x,
                                  ),
                                },
                              })
                            }
                          />
                        </div>
                      </div>
                    ))}
                    <div className="flex items-center gap-2">
                      <Button
                        size="sm"
                        variant="ghost"
                        disabled={locked}
                        onClick={() =>
                          updateStep(idx, {
                            ab: {
                              enabled: true,
                              buckets: [
                                ...(st.ab?.buckets ?? []),
                                { name: `B${(st.ab?.buckets?.length ?? 0) + 1}`, ratio: 0, templateId: st.templateId },
                              ],
                            },
                          })
                        }
                      >
                        <Plus className="h-4 w-4" />
                        添加桶
                      </Button>
                      <div className="text-xs text-zinc-400">
                        建议桶比例合计为 100
                      </div>
                    </div>
                  </div>
                ) : (
                  <div className="mt-3 text-xs text-zinc-400">
                    关闭 AB 时使用步骤模板 ID；开启 AB 后按桶模板 ID 渲染。
                  </div>
                )}
              </div>
            </CardContent>
          </Card>
        ))}

        {normalized.steps.length === 0 ? (
          <div className="rounded-2xl border border-zinc-900 bg-zinc-950/20 p-10 text-center text-sm text-zinc-400">
            暂无触达步骤
          </div>
        ) : null}
      </div>

      <div className="grid grid-cols-1 gap-4 lg:grid-cols-2">
        <Card>
          <CardHeader>
            <CardTitle>测试触达（干跑）</CardTitle>
          </CardHeader>
          <CardContent className="space-y-3">
            <div className="text-xs leading-5 text-zinc-400">
              MVP 仅做规则解释执行与触达配置渲染验证，不会发送真实消息。
            </div>
            <Textarea
              className="min-h-[150px]"
              value={sampleUserJson}
              onChange={(e) => setSampleUserJson(e.target.value)}
            />
            <div className="flex items-center gap-2">
              <Button
                variant="secondary"
                onClick={async () => {
                  setError(null)
                  try {
                    const sampleUser = JSON.parse(sampleUserJson)
                    const r = await apiFetch(`/api/activities/${activity.id}/dry-run`, {
                      method: 'POST',
                      actor,
                      body: JSON.stringify({ sampleUser }),
                    })
                    setDryRunResult(r)
                  } catch (e: any) {
                    setError(e?.message ?? '干跑失败')
                  }
                }}
              >
                <FlaskConical className="h-4 w-4" />
                运行干跑
              </Button>
              <Button variant="ghost" onClick={() => setDryRunResult(null)}>
                清空结果
              </Button>
            </div>
            {dryRunResult ? <JsonPreview value={dryRunResult} /> : null}
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>编排 JSON 预览</CardTitle>
          </CardHeader>
          <CardContent>
            <JsonPreview value={normalized} />
          </CardContent>
        </Card>
      </div>
    </div>
  )
}

