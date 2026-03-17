import { useEffect, useMemo, useState } from 'react'
import { useParams } from 'react-router-dom'
import { apiFetch } from '@/api/http'
import type { ActivityFull, BenefitConfig } from '@/types'
import { actorHeaderValue, useAuthStore } from '@/stores/auth'
import ActivityNav from '@/components/ActivityNav'
import Button from '@/components/ui/Button'
import Input from '@/components/ui/Input'
import Select from '@/components/ui/Select'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card'
import JsonPreview from '@/components/JsonPreview'
import { Gift, Plus, Save, Trash2 } from 'lucide-react'

function uid(prefix: string) {
  return `${prefix}_${crypto.randomUUID()}`
}

export default function Benefits() {
  const { id = '' } = useParams()
  const auth = useAuthStore()
  const actor = actorHeaderValue(auth)

  const [activity, setActivity] = useState<ActivityFull | null>(null)
  const [benefits, setBenefits] = useState<BenefitConfig>({})
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const load = async () => {
    const a = await apiFetch<ActivityFull>(`/api/activities/${id}`, { actor })
    setActivity(a)
    setBenefits((a.config.benefits ?? {}) as BenefitConfig)
  }

  useEffect(() => {
    load().catch(() => {})
  }, [id])

  const locked = activity?.status === 'online'

  const normalized = useMemo(() => {
    return {
      couponPacks: benefits.couponPacks ?? [],
      pointsRules: benefits.pointsRules ?? [],
      others: benefits.others ?? [],
    } satisfies BenefitConfig
  }, [benefits])

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
            BENEFITS
          </div>
          <div className="mt-2 flex items-center gap-2 text-2xl font-semibold">
            <Gift className="h-6 w-6 text-amber-200" />
            权益配置中心
          </div>
          <div className="mt-4">
            <ActivityNav id={activity.id} />
          </div>
        </div>
        <div className="flex items-center gap-2">
          <Button
            variant="primary"
            disabled={locked || saving}
            onClick={async () => {
              setSaving(true)
              setError(null)
              try {
                const updated = await apiFetch<ActivityFull>(`/api/activities/${activity.id}/benefits`, {
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

      <div className="grid grid-cols-1 gap-4 lg:grid-cols-2">
        <Card>
          <CardHeader>
            <CardTitle>券包管理</CardTitle>
          </CardHeader>
          <CardContent className="space-y-3">
            <div className="flex items-center gap-2">
              <Button
                size="sm"
                variant="secondary"
                disabled={locked}
                onClick={() =>
                  setBenefits((s) => ({
                    ...s,
                    couponPacks: [
                      ...(s.couponPacks ?? []),
                      { id: uid('cp'), name: '新券包', coupons: [{ code: 'COUPON_X', count: 1 }] },
                    ],
                  }))
                }
              >
                <Plus className="h-4 w-4" />
                添加券包
              </Button>
            </div>
            <div className="space-y-2">
              {(normalized.couponPacks ?? []).map((p, idx) => (
                <div key={p.id} className="rounded-2xl border border-zinc-900 bg-zinc-950/20 p-4">
                  <div className="flex items-center justify-between gap-2">
                    <div className="flex-1 min-w-0">
                      <Input
                        disabled={locked}
                        value={p.name}
                        onChange={(e) =>
                          setBenefits((s) => {
                            const next = structuredClone(s) as BenefitConfig
                            next.couponPacks = next.couponPacks ?? []
                            next.couponPacks[idx].name = e.target.value
                            return next
                          })
                        }
                      />
                      <div className="mt-2 text-xs text-zinc-500">{p.id}</div>
                    </div>
                    <Button
                      size="sm"
                      variant="ghost"
                      disabled={locked}
                      onClick={() =>
                        setBenefits((s) => ({
                          ...s,
                          couponPacks: (s.couponPacks ?? []).filter((x) => x.id !== p.id),
                        }))
                      }
                    >
                      <Trash2 className="h-4 w-4" />
                    </Button>
                  </div>
                  <div className="mt-3 space-y-2">
                    {p.coupons.map((c, ci) => (
                      <div key={`${p.id}_${ci}`} className="grid grid-cols-12 gap-2">
                        <div className="col-span-8">
                          <Input
                            disabled={locked}
                            value={c.code}
                            onChange={(e) =>
                              setBenefits((s) => {
                                const next = structuredClone(s) as BenefitConfig
                                next.couponPacks = next.couponPacks ?? []
                                next.couponPacks[idx].coupons[ci].code = e.target.value
                                return next
                              })
                            }
                          />
                        </div>
                        <div className="col-span-3">
                          <Input
                            disabled={locked}
                            inputMode="numeric"
                            value={String(c.count)}
                            onChange={(e) =>
                              setBenefits((s) => {
                                const next = structuredClone(s) as BenefitConfig
                                next.couponPacks = next.couponPacks ?? []
                                next.couponPacks[idx].coupons[ci].count = Number(e.target.value || 0)
                                return next
                              })
                            }
                          />
                        </div>
                        <div className="col-span-1 flex justify-end">
                          <Button
                            size="sm"
                            variant="ghost"
                            disabled={locked}
                            onClick={() =>
                              setBenefits((s) => {
                                const next = structuredClone(s) as BenefitConfig
                                next.couponPacks = next.couponPacks ?? []
                                next.couponPacks[idx].coupons.splice(ci, 1)
                                return next
                              })
                            }
                          >
                            <Trash2 className="h-4 w-4" />
                          </Button>
                        </div>
                      </div>
                    ))}
                    <Button
                      size="sm"
                      variant="ghost"
                      disabled={locked}
                      onClick={() =>
                        setBenefits((s) => {
                          const next = structuredClone(s) as BenefitConfig
                          next.couponPacks = next.couponPacks ?? []
                          next.couponPacks[idx].coupons.push({ code: 'COUPON_Y', count: 1 })
                          return next
                        })
                      }
                    >
                      <Plus className="h-4 w-4" />
                      添加券
                    </Button>
                  </div>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>积分规则</CardTitle>
          </CardHeader>
          <CardContent className="space-y-3">
            <div className="flex items-center gap-2">
              <Button
                size="sm"
                variant="secondary"
                disabled={locked}
                onClick={() =>
                  setBenefits((s) => ({
                    ...s,
                    pointsRules: [
                      ...(s.pointsRules ?? []),
                      { id: uid('pt'), name: '新积分规则', earnPerOrder: 10, dailyCap: 200 },
                    ],
                  }))
                }
              >
                <Plus className="h-4 w-4" />
                添加规则
              </Button>
            </div>

            <div className="space-y-2">
              {(normalized.pointsRules ?? []).map((r, idx) => (
                <div key={r.id} className="rounded-2xl border border-zinc-900 bg-zinc-950/20 p-4">
                  <div className="flex items-center justify-between gap-2">
                    <div className="flex-1 min-w-0">
                      <Input
                        disabled={locked}
                        value={r.name}
                        onChange={(e) =>
                          setBenefits((s) => {
                            const next = structuredClone(s) as BenefitConfig
                            next.pointsRules = next.pointsRules ?? []
                            next.pointsRules[idx].name = e.target.value
                            return next
                          })
                        }
                      />
                      <div className="mt-2 grid grid-cols-2 gap-2">
                        <div>
                          <div className="text-xs text-zinc-500">每单获取</div>
                          <Input
                            disabled={locked}
                            inputMode="numeric"
                            value={String(r.earnPerOrder ?? 0)}
                            onChange={(e) =>
                              setBenefits((s) => {
                                const next = structuredClone(s) as BenefitConfig
                                next.pointsRules = next.pointsRules ?? []
                                next.pointsRules[idx].earnPerOrder = Number(e.target.value || 0)
                                return next
                              })
                            }
                          />
                        </div>
                        <div>
                          <div className="text-xs text-zinc-500">日上限</div>
                          <Input
                            disabled={locked}
                            inputMode="numeric"
                            value={String(r.dailyCap ?? 0)}
                            onChange={(e) =>
                              setBenefits((s) => {
                                const next = structuredClone(s) as BenefitConfig
                                next.pointsRules = next.pointsRules ?? []
                                next.pointsRules[idx].dailyCap = Number(e.target.value || 0)
                                return next
                              })
                            }
                          />
                        </div>
                      </div>
                    </div>
                    <Button
                      size="sm"
                      variant="ghost"
                      disabled={locked}
                      onClick={() =>
                        setBenefits((s) => ({
                          ...s,
                          pointsRules: (s.pointsRules ?? []).filter((x) => x.id !== r.id),
                        }))
                      }
                    >
                      <Trash2 className="h-4 w-4" />
                    </Button>
                  </div>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>其他权益（实物/虚拟）</CardTitle>
        </CardHeader>
        <CardContent className="space-y-3">
          <div className="flex items-center gap-2">
            <Button
              size="sm"
              variant="secondary"
              disabled={locked}
              onClick={() =>
                setBenefits((s) => ({
                  ...s,
                  others: [
                    ...(s.others ?? []),
                    { id: uid('bn'), name: '新权益', kind: 'virtual', stock: 100 },
                  ],
                }))
              }
            >
              <Plus className="h-4 w-4" />
              添加权益
            </Button>
          </div>
          <div className="grid grid-cols-1 gap-2 lg:grid-cols-2">
            {(normalized.others ?? []).map((b, idx) => (
              <div key={b.id} className="rounded-2xl border border-zinc-900 bg-zinc-950/20 p-4">
                <div className="flex items-center justify-between gap-2">
                  <div className="flex-1 min-w-0 space-y-2">
                    <Input
                      disabled={locked}
                      value={b.name}
                      onChange={(e) =>
                        setBenefits((s) => {
                          const next = structuredClone(s) as BenefitConfig
                          next.others = next.others ?? []
                          next.others[idx].name = e.target.value
                          return next
                        })
                      }
                    />
                    <div className="grid grid-cols-2 gap-2">
                      <Select
                        disabled={locked}
                        value={b.kind}
                        onChange={(e) =>
                          setBenefits((s) => {
                            const next = structuredClone(s) as BenefitConfig
                            next.others = next.others ?? []
                            next.others[idx].kind = e.target.value as any
                            return next
                          })
                        }
                      >
                        <option value="virtual">虚拟</option>
                        <option value="physical">实物</option>
                      </Select>
                      <Input
                        disabled={locked}
                        inputMode="numeric"
                        value={String(b.stock ?? 0)}
                        onChange={(e) =>
                          setBenefits((s) => {
                            const next = structuredClone(s) as BenefitConfig
                            next.others = next.others ?? []
                            next.others[idx].stock = Number(e.target.value || 0)
                            return next
                          })
                        }
                      />
                    </div>
                    <div className="text-xs text-zinc-500">{b.id}</div>
                  </div>
                  <Button
                    size="sm"
                    variant="ghost"
                    disabled={locked}
                    onClick={() =>
                      setBenefits((s) => ({
                        ...s,
                        others: (s.others ?? []).filter((x) => x.id !== b.id),
                      }))
                    }
                  >
                    <Trash2 className="h-4 w-4" />
                  </Button>
                </div>
              </div>
            ))}
          </div>
        </CardContent>
      </Card>

      {error ? <div className="text-sm text-rose-200">{error}</div> : null}

      <Card>
        <CardHeader>
          <CardTitle>权益 JSON 预览</CardTitle>
        </CardHeader>
        <CardContent>
          <JsonPreview value={normalized} />
        </CardContent>
      </Card>
    </div>
  )
}

