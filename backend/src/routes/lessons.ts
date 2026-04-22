import { Router, Request, Response } from 'express';
import { prisma } from '../db';

const router = Router();

// GET /api/lessons/:id/exercises
router.get('/:id/exercises', async (req: Request, res: Response) => {
  try {
    const { id } = req.params;
    const lessonId = parseInt(String(id), 10);
    
    if (isNaN(lessonId)) {
      res.status(400).json({ error: 'Invalid lesson ID' });
      return;
    }

    const lesson = await prisma.lesson.findUnique({
      where: { id: lessonId },
    });

    if (!lesson) {
      res.status(404).json({ error: 'Lesson not found' });
      return;
    }

    const exercises = await prisma.exercise.findMany({
      where: { lessonId },
      orderBy: {
        id: 'asc',
      },
    });

    // Parse options for each exercise since it's stored as JSON string
    const formattedExercises = exercises.map((ex) => {
      let parsedOptions = [];
      try {
        parsedOptions = JSON.parse(ex.options);
      } catch (e) {
        parsedOptions = [];
      }
      return {
        ...ex,
        options: parsedOptions,
      };
    });

    res.json(formattedExercises);
  } catch (error) {
    console.error('Error fetching exercises:', error);
    res.status(500).json({ error: 'Internal server error' });
  }
});

export default router;
