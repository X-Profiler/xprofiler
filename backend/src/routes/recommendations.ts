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

    // Find all completed lesson IDs for the user
    const completedProgresses = await prisma.progress.findMany({
      where: { userId },
      select: { lessonId: true },
    });
    const completedLessonIds = completedProgresses.map((p) => p.lessonId);

    // Find the first lesson that is not completed
    const nextLesson = await prisma.lesson.findFirst({
      where: {
        id: {
          notIn: completedLessonIds,
        },
      },
      orderBy: [
        { chapter: { courseId: 'asc' } },
        { chapter: { order: 'asc' } },
        { order: 'asc' },
        { id: 'asc' },
      ],
      include: {
        chapter: {
          include: {
            course: true,
          },
        },
      },
    });

    if (!nextLesson) {
      res.json({ message: 'All lessons completed', lesson: null });
      return;
    }

    res.json(nextLesson);
  } catch (error) {
    console.error('Error fetching recommendation:', error);
    res.status(500).json({ error: 'Failed to fetch recommendation' });
  }
});

export default router;