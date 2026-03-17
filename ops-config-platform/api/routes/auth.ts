/**
 * This is a user authentication API route demo.
 * Handle user registration, login, token management, etc.
 */
import { Router, type Request, type Response } from 'express'

const router = Router()

/**
 * User Login
 * POST /api/auth/register
 */
router.post('/register', async (req: Request, res: Response): Promise<void> => {
  res.status(501).json({ success: false, error: 'not implemented' })
})

/**
 * User Login
 * POST /api/auth/login
 */
router.post('/login', async (req: Request, res: Response): Promise<void> => {
  const username = String(req.body?.username ?? '').trim()
  const role = String(req.body?.role ?? 'operator').trim()
  if (!username) {
    res.status(400).json({ success: false, error: 'username required' })
    return
  }
  res.json({
    success: true,
    data: {
      token: `demo_${role}_${username}`,
      user: { username, role },
    },
  })
})

/**
 * User Logout
 * POST /api/auth/logout
 */
router.post('/logout', async (req: Request, res: Response): Promise<void> => {
  res.json({ success: true })
})

export default router
