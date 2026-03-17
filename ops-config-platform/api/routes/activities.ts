import { Router, type Request, type Response } from 'express'
import {
  createActivity,
  getActivity,
  listActivities,
  listAuditLogs,
  listSnapshots,
  rollbackToSnapshot,
  upsertMetricDelta,
  updateActivityBase,
  updateActivityConfigSection,
  updateActivityStatus,
} from '../storage/activityRepo.js'
import { evaluateConditionGroup, validateConditionGroup } from '../domain/conditions.js'

const router = Router()

function actorFrom(req: Request): string {
  const v = req.header('x-actor')
  return v && v.trim() ? v.trim() : 'operator'
}

router.get('/', (req: Request, res: Response) => {
  res.json({ success: true, data: listActivities() })
})

router.post('/', (req: Request, res: Response) => {
  const name = String(req.body?.name ?? '').trim()
  if (!name) {
    res.status(400).json({ success: false, error: 'name required' })
    return
  }
  const description = typeof req.body?.description === 'string' ? req.body.description : ''
  const a = createActivity({ name, description, actor: actorFrom(req) })
  res.status(201).json({ success: true, data: a })
})

router.get('/:id', (req: Request, res: Response) => {
  const a = getActivity(req.params.id)
  if (!a) {
    res.status(404).json({ success: false, error: 'not found' })
    return
  }
  res.json({ success: true, data: a })
})

router.patch('/:id', (req: Request, res: Response) => {
  try {
    const a = updateActivityBase({
      id: req.params.id,
      actor: actorFrom(req),
      patch: {
        name: typeof req.body?.name === 'string' ? req.body.name : undefined,
        description:
          typeof req.body?.description === 'string' ? req.body.description : undefined,
      },
    })
    res.json({ success: true, data: a })
  } catch (e: any) {
    res.status(400).json({ success: false, error: e?.message ?? 'bad request' })
  }
})

router.post('/:id/lifecycle', (req: Request, res: Response) => {
  const next = String(req.body?.next ?? '')
  try {
    const a = updateActivityStatus({
      id: req.params.id,
      next: next as any,
      actor: actorFrom(req),
    })
    res.json({ success: true, data: a })
  } catch (e: any) {
    res.status(400).json({ success: false, error: e?.message ?? 'bad request' })
  }
})

router.put('/:id/audience', (req: Request, res: Response) => {
  try {
    const a = updateActivityConfigSection({
      id: req.params.id,
      section: 'audience',
      value: req.body ?? {},
      actor: actorFrom(req),
    })
    res.json({ success: true, data: a })
  } catch (e: any) {
    res.status(400).json({ success: false, error: e?.message ?? 'bad request' })
  }
})

router.post('/:id/audience/estimate', (req: Request, res: Response) => {
  const a = getActivity(req.params.id)
  if (!a) {
    res.status(404).json({ success: false, error: 'not found' })
    return
  }
  const audience: any = a.config.audience ?? {}
  const whitelistCount = Array.isArray(audience?.whitelist?.userIds)
    ? audience.whitelist.userIds.length
    : 0
  const base = whitelistCount > 0 ? whitelistCount : 12000
  const jitter = Math.floor(base * 0.12)
  const estimate = Math.max(0, base + Math.floor(Math.random() * (jitter + 1)) - Math.floor(jitter / 2))
  res.json({ success: true, data: { estimate } })
})

router.put('/:id/benefits', (req: Request, res: Response) => {
  try {
    const a = updateActivityConfigSection({
      id: req.params.id,
      section: 'benefits',
      value: req.body ?? {},
      actor: actorFrom(req),
    })
    res.json({ success: true, data: a })
  } catch (e: any) {
    res.status(400).json({ success: false, error: e?.message ?? 'bad request' })
  }
})

router.put('/:id/rules', (req: Request, res: Response) => {
  try {
    const a = updateActivityConfigSection({
      id: req.params.id,
      section: 'rules',
      value: req.body ?? {},
      actor: actorFrom(req),
    })
    res.json({ success: true, data: a })
  } catch (e: any) {
    res.status(400).json({ success: false, error: e?.message ?? 'bad request' })
  }
})

router.put('/:id/orchestration', (req: Request, res: Response) => {
  try {
    const a = updateActivityConfigSection({
      id: req.params.id,
      section: 'orchestration',
      value: req.body ?? {},
      actor: actorFrom(req),
    })
    res.json({ success: true, data: a })
  } catch (e: any) {
    res.status(400).json({ success: false, error: e?.message ?? 'bad request' })
  }
})

router.get('/:id/audit-logs', (req: Request, res: Response) => {
  res.json({ success: true, data: listAuditLogs(req.params.id) })
})

router.get('/:id/snapshots', (req: Request, res: Response) => {
  res.json({ success: true, data: listSnapshots(req.params.id) })
})

router.post('/:id/rollback', (req: Request, res: Response) => {
  const snapshotId = String(req.body?.snapshotId ?? '')
  if (!snapshotId) {
    res.status(400).json({ success: false, error: 'snapshotId required' })
    return
  }
  try {
    const a = rollbackToSnapshot({
      activityId: req.params.id,
      snapshotId,
      actor: actorFrom(req),
    })
    res.json({ success: true, data: a })
  } catch (e: any) {
    res.status(400).json({ success: false, error: e?.message ?? 'bad request' })
  }
})

router.post('/:id/dry-run', (req: Request, res: Response) => {
  const a = getActivity(req.params.id)
  if (!a) {
    res.status(404).json({ success: false, error: 'not found' })
    return
  }

  const sampleUser = (req.body?.sampleUser ?? {}) as Record<string, unknown>
  const rules = (a.config.rules ?? {}) as any
  const threshold = rules?.threshold

  let eligible = true
  if (threshold && typeof threshold === 'object') {
    const v = validateConditionGroup(threshold, 4)
    if (v.ok === false) {
      res.status(400).json({ success: false, error: `${v.path}: ${v.error}` })
      return
    }
    eligible = evaluateConditionGroup(threshold, sampleUser)
  }

  const today = new Date().toISOString().slice(0, 10)
  const plan: any = a.config.orchestration ?? {}
  const steps = Array.isArray(plan?.steps) ? plan.steps.length : 0
  upsertMetricDelta({
    activityId: a.id,
    date: today,
    participants: eligible ? 1 : 0,
    conversions: eligible ? (Math.random() < 0.18 ? 1 : 0) : 0,
    touchSuccess: eligible ? steps : 0,
    touchFail: 0,
  })

  res.json({
    success: true,
    data: {
      eligible,
      message: eligible ? '命中规则，可执行触达/发放' : '未命中规则',
    },
  })
})

export default router
