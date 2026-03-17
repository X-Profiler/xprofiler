import { Router, type Request, type Response } from 'express'
import { getActivityMetrics, getOverviewMetrics } from '../storage/activityRepo.js'

const router = Router()

router.get('/overview', (req: Request, res: Response) => {
  res.json({ success: true, data: getOverviewMetrics() })
})

router.get('/activities/:id', (req: Request, res: Response) => {
  res.json({ success: true, data: getActivityMetrics(req.params.id) })
})

export default router

