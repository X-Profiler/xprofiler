"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const express_1 = __importDefault(require("express"));
const db_1 = require("../db");
const authMiddleware_1 = require("../middleware/authMiddleware");
const router = express_1.default.Router();
router.get('/', authMiddleware_1.authMiddleware, async (req, res) => {
    try {
        const userId = req.user?.userId;
        if (!userId) {
            res.status(401).json({ error: 'Unauthorized' });
            return;
        }
        // Find all completed lesson IDs for the user
        const completedProgresses = await db_1.prisma.progress.findMany({
            where: { userId },
            select: { lessonId: true },
        });
        const completedLessonIds = completedProgresses.map((p) => p.lessonId);
        // Find the first lesson that is not completed
        const nextLesson = await db_1.prisma.lesson.findFirst({
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
    }
    catch (error) {
        console.error('Error fetching recommendation:', error);
        res.status(500).json({ error: 'Failed to fetch recommendation' });
    }
});
exports.default = router;
