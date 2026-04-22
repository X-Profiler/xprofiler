import express, { Response } from 'express';
import { AuthRequest, authMiddleware } from '../middleware/authMiddleware';
import { prisma } from '../db';

const router = express.Router();

router.use(authMiddleware);

// POST /api/progress
router.post('/', async (req: AuthRequest, res: Response): Promise<void> => {
  try {
    const userId = req.user?.userId;
    const { lessonId, score } = req.body;

    if (!userId) {
      res.status(401).json({ error: 'Unauthorized' });
      return;
    }

    if (typeof lessonId !== 'number' || typeof score !== 'number') {
      res.status(400).json({ error: 'Invalid lessonId or score' });
      return;
    }

    const progress = await prisma.progress.create({
      data: {
        userId,
        lessonId,
        score,
      },
    });

    res.status(201).json(progress);
  } catch (error) {
    console.error('Failed to save progress:', error);
    res.status(500).json({ error: 'Failed to save progress' });
  }
});

// GET /api/progress
router.get('/', async (req: AuthRequest, res: Response): Promise<void> => {
  try {
    const userId = req.user?.userId;

    if (!userId) {
      res.status(401).json({ error: 'Unauthorized' });
      return;
    }

    const progressRecords = await prisma.progress.findMany({
      where: { userId },
      orderBy: { completedAt: 'asc' },
    });

    const completedLessonsCount = new Set(progressRecords.map(p => p.lessonId)).size;
    
    let totalScore = 0;
    progressRecords.forEach(p => {
      totalScore += p.score;
    });
    
    const averageScore = progressRecords.length > 0 
      ? totalScore / progressRecords.length 
      : 0;

    // Group by date (YYYY-MM-DD)
    const dailyRecordsMap = new Map<string, number>();
    const dailyScoresMap = new Map<string, { total: number, count: number }>();

    progressRecords.forEach(p => {
      // Create date string in YYYY-MM-DD format based on UTC
      const dateStr = p.completedAt.toISOString().split('T')[0];
      
      // We can just keep the highest score for a lesson on a given day, or average, or just return all records
      // The requirement says: "按日期的成绩记录（用于图表）"
      // Let's provide average score per day or just all records per day?
      // Typically for a chart we might want the average score per day and number of lessons completed
      if (!dailyScoresMap.has(dateStr)) {
        dailyScoresMap.set(dateStr, { total: 0, count: 0 });
      }
      
      const current = dailyScoresMap.get(dateStr)!;
      current.total += p.score;
      current.count += 1;
    });

    const dailyRecords = Array.from(dailyScoresMap.entries()).map(([date, data]) => ({
      date,
      averageScore: data.total / data.count,
      lessonsCompleted: data.count,
    }));

    res.json({
      completedLessons: completedLessonsCount,
      averageScore,
      dailyRecords,
      records: progressRecords // Might be useful to return raw records too
    });
  } catch (error) {
    console.error('Failed to fetch progress:', error);
    res.status(500).json({ error: 'Failed to fetch progress' });
  }
});

export default router;