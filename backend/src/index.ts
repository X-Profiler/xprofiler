import express, { Request, Response } from 'express';
import cors from 'cors';
import dotenv from 'dotenv';
import { prisma } from './db';
import authRoutes from './routes/auth';
import coursesRoutes from './routes/courses';
import lessonsRoutes from './routes/lessons';
import progressRoutes from './routes/progress';
import recommendationsRoutes from './routes/recommendations';
import communityRoutes from './routes/community';
import badgesRoutes from './routes/badges';
import { authMiddleware, AuthRequest } from './middleware/authMiddleware';

dotenv.config();

const app = express();
const port = process.env.PORT || 3000;

app.use(cors());
app.use(express.json());

// Routes
app.use('/api/auth', authRoutes);
app.use('/api/courses', coursesRoutes);
app.use('/api/lessons', lessonsRoutes);
app.use('/api/progress', progressRoutes);
app.use('/api/recommendations', recommendationsRoutes);
app.use('/api/community', communityRoutes);
app.use('/api/badges', badgesRoutes);

// Protected route example
app.get('/api/auth/me', authMiddleware, (req: AuthRequest, res: Response) => {
  res.json({ message: 'Authenticated', user: req.user });
});

// Health check route
app.get('/health', async (req: Request, res: Response) => {

  try {
    // Check database connection
    await prisma.$queryRaw`SELECT 1`;
    res.status(200).json({ status: 'ok', database: 'connected' });
  } catch (error) {
    console.error('Health check failed:', error);
    res.status(500).json({ status: 'error', database: 'disconnected', error: String(error) });
  }
});

// A simple API to fetch users
app.get('/users', async (req: Request, res: Response) => {
  try {
    const users = await prisma.user.findMany({
      select: { id: true, email: true, username: true, createdAt: true, updatedAt: true }
    });
    res.json(users);
  } catch (error) {
    res.status(500).json({ error: 'Could not fetch users' });
  }
});

app.listen(port, () => {
  console.log(`Server is running at http://localhost:${port}`);
});
