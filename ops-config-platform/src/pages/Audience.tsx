import { useEffect, useMemo, useState } from 'react'
import { useParams } from 'react-router-dom'
import { apiFetch } from '@/api/http'
import type { ActivityFull, AudienceDefinition, ConditionGroup } from '@/types'
import { actorHeaderValue, useAuthStore } from '@/stores/auth'
import ActivityNav from '@/components/ActivityNav'
import ConditionBuilder from '@/components/conditions/ConditionBuilder'
import Button from '@/components/ui/Button'
import Badge from '@/components/ui/Badge'
import Textarea from '@/components/ui/Textarea'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card'
import JsonPreview from '@/components/JsonPreview'
import { Users, Calculator, Save } from 'lucide-react'
import * as XLSX from 'xlsx'

const LABEL_FIELDS = [
  { key: 'gender', label: '性别', valueType: 'string' as const, placeholder: 'male/female' },
  { key: 'age', label: '年龄', valueType: 'number' as const, placeholder: '18' },
  { key: 'region', label: '地域', valueType: 'string' as const, placeholder: 'CN-SH' },
  { key: 'vipLevel', label: '会员等级', valueType: 'number' as const, placeholder: '3' },
]

const BEHAVIOR_FIELDS = [
  { key: 'viewCount7d', label: '近7天浏览次数', valueType: 'number' as const, placeholder: '5' },
  { key: 'clickCount7d', label: '近7天点击次数', valueType: 'number' as const, placeholder: '3' },
  { key: 'purchaseAmount30d', label: '近30天消费金额', valueType: 'number' as const, placeholder: '200' },
  { key: 'purchaseCount30d', label: '近30天消费次数', valueType: 'number' as const, placeholder: '2' },
]

function emptyGroup(): ConditionGroup {
  return { logic: 'AND', children: [] }
}

function parseIds(raw: string): string[] {
  return raw
    .split(/[\s,，]+/g)
    .map((x) => x.trim())
    .filter(Boolean)
}

async function readExcelUserIds(file: File): Promise<string[]> {
  const buf = await file.arrayBuffer()
  const wb = XLSX.read(buf)
  const sheetName = wb.SheetNames[0]
  const sheet = wb.Sheets[sheetName]
  const rows = XLSX.utils.sheet_to_json<any[]>(sheet, { header: 1 }) as any[][]
  return rows
    .map((r) => (r?.[0] ?? '').toString().trim())
    .filter(Boolean)
}

export default function Audience() {
  const { id = '' } = useParams()
  const auth = useAuthStore()
  const actor = actorHeaderValue(auth)

  const [activity, setActivity] = useState<ActivityFull | null>(null)
  const [audience, setAudience] = useState<AudienceDefinition>({
    labels: { logic: 'AND', children: [] },
    behaviors: { logic: 'AND', children: [] },
    whitelist: { enabled: false, source: 'manual', userIds: [] },
  })
  const [whitelistText, setWhitelistText] = useState('')
  const [estimate, setEstimate] = useState<number | null>(null)
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const effective = useMemo(() => {
    const wl = parseIds(whitelistText)
    return {
      ...audience,
      whitelist: audience.whitelist
        ? { ...audience.whitelist, userIds: wl }
        : { enabled: false, source: 'manual' as const, userIds: wl },
    }
  }, [audience, whitelistText])

  const load = async () => {
    const a = await apiFetch<ActivityFull>(`/api/activities/${id}`, { actor })
    setActivity(a)
    const cfg = (a.config.audience ?? {}) as AudienceDefinition
    setAudience({
      labels: cfg.labels ?? { logic: 'AND', children: [] },
      behaviors: cfg.behaviors ?? { logic: 'AND', children: [] },
      whitelist: cfg.whitelist ?? { enabled: false, source: 'manual', userIds: [] },
    })
    setWhitelistText((cfg.whitelist?.userIds ?? []).join('\n'))
  }

  useEffect(() => {
    load().catch(() => {})
  }, [id])

  if (!activity) {
    return (
      <div className="rounded-2xl border border-zinc-900 bg-zinc-950/20 p-10 text-center text-sm text-zinc-400">
        加载中…
      </div>
    )
  }

  const locked = activity.status === 'online'

  return (
    <div className="space-y-6">
      <div className="flex flex-wrap items-start justify-between gap-4">
        <div>
          <div className="text-xs font-semibold tracking-[0.2em] text-zinc-400">
            AUDIENCE
          </div>
          <div className="mt-2 flex items-center gap-2 text-2xl font-semibold">
            <Users className="h-6 w-6 text-cyan-200" />
            人群圈选
          </div>
          <div className="mt-4">
            <ActivityNav id={activity.id} />
          </div>
        </div>
        <div className="flex flex-wrap items-center gap-2">
          <Button
            variant="secondary"
            disabled={locked}
            onClick={async () => {
              setError(null)
              try {
                const r = await apiFetch<{ estimate: number }>(
                  `/api/activities/${activity.id}/audience/estimate`,
                  { method: 'POST', actor },
                )
                setEstimate(r.estimate)
              } catch (e: any) {
                setError(e?.message ?? '预估失败')
              }
            }}
          >
            <Calculator className="h-4 w-4" />
            预估人数
          </Button>
          <Button
            variant="primary"
            disabled={locked || saving}
            onClick={async () => {
              setSaving(true)
              setError(null)
              try {
                const updated = await apiFetch<ActivityFull>(`/api/activities/${activity.id}/audience`, {
                  method: 'PUT',
                  actor,
                  body: JSON.stringify(effective),
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
            <CardTitle>标签筛选</CardTitle>
          </CardHeader>
          <CardContent>
            <ConditionBuilder
              fields={LABEL_FIELDS}
              value={audience.labels ?? emptyGroup()}
              onChange={(v) => setAudience((s) => ({ ...s, labels: v }))}
            />
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>行为筛选</CardTitle>
          </CardHeader>
          <CardContent>
            <ConditionBuilder
              fields={BEHAVIOR_FIELDS}
              value={audience.behaviors ?? emptyGroup()}
              onChange={(v) => setAudience((s) => ({ ...s, behaviors: v }))}
            />
          </CardContent>
        </Card>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>白名单管理</CardTitle>
        </CardHeader>
        <CardContent className="space-y-3">
          <label className="flex items-center gap-2 text-sm text-zinc-200">
            <input
              type="checkbox"
              checked={!!audience.whitelist?.enabled}
              onChange={(e) =>
                setAudience((s) => ({
                  ...s,
                  whitelist: {
                    enabled: e.target.checked,
                    source: 'manual',
                    userIds: s.whitelist?.userIds ?? [],
                  },
                }))
              }
            />
            启用白名单（与标签/行为条件同时生效）
          </label>
          <div className="flex flex-wrap items-center gap-2">
            <div className="text-xs leading-5 text-zinc-400">
              支持 Excel(.xlsx) 导入或手动粘贴 ID（换行/空格/逗号分隔）。
            </div>
            <label className="inline-flex cursor-pointer items-center gap-2 rounded-xl border border-zinc-900 bg-zinc-950/20 px-3 py-2 text-xs text-zinc-200 transition hover:border-zinc-800 hover:bg-zinc-950/30">
              导入 Excel
              <input
                type="file"
                accept=".xlsx"
                className="hidden"
                onChange={async (e) => {
                  const f = e.target.files?.[0]
                  if (!f) return
                  setError(null)
                  try {
                    const ids = await readExcelUserIds(f)
                    setWhitelistText(ids.join('\n'))
                    setAudience((s) => ({
                      ...s,
                      whitelist: {
                        enabled: true,
                        source: 'excel',
                        userIds: ids,
                      },
                    }))
                  } catch (err: any) {
                    setError(err?.message ?? '导入失败')
                  } finally {
                    e.target.value = ''
                  }
                }}
              />
            </label>
          </div>
          <Textarea
            placeholder="user_001&#10;user_002&#10;user_003"
            value={whitelistText}
            onChange={(e) => setWhitelistText(e.target.value)}
          />
          <div className="flex flex-wrap items-center gap-2">
            <Badge tone="neutral">去重后: {parseIds(whitelistText).length} 个</Badge>
            {estimate !== null ? <Badge tone="cyan">预估: {estimate}</Badge> : null}
          </div>
        </CardContent>
      </Card>

      {error ? <div className="text-sm text-rose-200">{error}</div> : null}

      <div className="grid grid-cols-1 gap-4 lg:grid-cols-2">
        <Card>
          <CardHeader>
            <CardTitle>人群 JSON 预览</CardTitle>
          </CardHeader>
          <CardContent>
            <JsonPreview value={effective} />
          </CardContent>
        </Card>
        <Card>
          <CardHeader>
            <CardTitle>口径提示</CardTitle>
          </CardHeader>
          <CardContent className="space-y-2 text-sm text-zinc-300">
            <div className="text-zinc-400 text-xs leading-5">
              MVP 的人数预估为演示口径：白名单优先，未启用白名单时返回一个带随机扰动的估值。
            </div>
            <div className="rounded-2xl border border-zinc-900 bg-zinc-950/20 p-4 text-xs text-zinc-400">
              真实落地时，建议将圈选定义保存为 AST，然后由离线计算引擎输出人群包与分层采样。
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
