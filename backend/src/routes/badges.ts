import express, { Response } from 'express';
import { prisma } from '../db';
import { authMiddleware, AuthRequest } from '../middleware/authMiddleware';

const router = express.Router();

router.get('/', authMiddleware, async (req: AuthRequest, res: Response) => {
  try {
    const userId = req.user?.userId;
    if (!userId) {
      res.status(401).json({ error: 'Unauthorized' });
      return;
    }

    const userBadges = await prisma.userBadge.findMany({
      where: { userId },
      include: { badge: true },
    });

    const allBadges = await prisma.badge.findMany();

    // Map all badges and indicate if user earned them
    const badgesWithStatus = allBadges.map((badge) => {
      const earned = userBadges.find((ub) => ub.badgeId === badge.id);
      return {
        ...badge,
        earned: !!earned,
        earnedAt: earned ? earned.earnedAt : null,
      };
    });

    res.json(badgesWithStatus);
  } catch (error) {
    console.error('Error fetching badges:', error);
    res.status(500).json({ error: 'Failed to fetch badges' });
  }
});

export default router;