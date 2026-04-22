"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const express_1 = require("express");
const db_1 = require("../db");
const router = (0, express_1.Router)();
// GET /api/courses
router.get('/', async (req, res) => {
    try {
        const { language, level } = req.query;
        const where = {};
        if (language) {
            where.language = String(language);
        }
        if (level) {
            where.level = String(level);
        }
        const courses = await db_1.prisma.course.findMany({
            where,
            include: {
                _count: {
                    select: { chapters: true }
                }
            }
        });
        res.json(courses);
    }
    catch (error) {
        console.error('Error fetching courses:', error);
        res.status(500).json({ error: 'Internal server error' });
    }
});
// GET /api/courses/:id
router.get('/:id', async (req, res) => {
    try {
        const { id } = req.params;
        const courseId = parseInt(String(id), 10);
        if (isNaN(courseId)) {
            res.status(400).json({ error: 'Invalid course ID' });
            return;
        }
        const course = await db_1.prisma.course.findUnique({
            where: { id: courseId },
            include: {
                chapters: {
                    include: {
                        lessons: true,
                    },
                    orderBy: {
                        order: 'asc',
                    }
                },
            },
        });
        if (!course) {
            res.status(404).json({ error: 'Course not found' });
            return;
        }
        res.json(course);
    }
    catch (error) {
        console.error('Error fetching course by id:', error);
        res.status(500).json({ error: 'Internal server error' });
    }
});
exports.default = router;
