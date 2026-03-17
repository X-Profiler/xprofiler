import { Router, type Request, type Response } from 'express'
import {
  evaluateConditionGroup,
  validateConditionGroup,
  type ConditionGroup,
} from '../domain/conditions.js'

const router = Router()

router.post('/validate', (req: Request, res: Response) => {
  const group = req.body as ConditionGroup
  const v = validateConditionGroup(group, 4)
  if (v.ok === false) {
    res.status(400).json({ success: false, error: `${v.path}: ${v.error}` })
    return
  }
  res.json({ success: true, data: { ok: true } })
})

router.post('/evaluate', (req: Request, res: Response) => {
  const group = req.body?.group as ConditionGroup
  const context = (req.body?.context ?? {}) as Record<string, unknown>
  const v = validateConditionGroup(group, 4)
  if (v.ok === false) {
    res.status(400).json({ success: false, error: `${v.path}: ${v.error}` })
    return
  }
  const result = evaluateConditionGroup(group, context)
  res.json({ success: true, data: { result } })
})

export default router
