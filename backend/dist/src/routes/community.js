"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const express_1 = __importDefault(require("express"));
const db_1 = require("../db");
const authMiddleware_1 = require("../middleware/authMiddleware");
const router = express_1.default.Router();
// Get list of posts
router.get('/', authMiddleware_1.authMiddleware, async (req, res) => {
    try {
        const posts = await db_1.prisma.post.findMany({
            orderBy: { createdAt: 'desc' },
            include: {
                author: {
                    select: { id: true, username: true, email: true },
                },
                comments: {
                    include: {
                        author: {
                            select: { id: true, username: true, email: true },
                        },
                    },
                },
            },
        });
        res.json(posts);
    }
    catch (error) {
        console.error('Error fetching posts:', error);
        res.status(500).json({ error: 'Failed to fetch posts' });
    }
});
// Create a new post
router.post('/', authMiddleware_1.authMiddleware, async (req, res) => {
    try {
        const userId = req.user?.userId;
        if (!userId) {
            res.status(401).json({ error: 'Unauthorized' });
            return;
        }
        const { title, content } = req.body;
        if (!title || !content) {
            res.status(400).json({ error: 'Title and content are required' });
            return;
        }
        const newPost = await db_1.prisma.post.create({
            data: {
                title,
                content,
                authorId: userId,
            },
            include: {
                author: {
                    select: { id: true, username: true, email: true },
                },
            },
        });
        res.status(201).json(newPost);
    }
    catch (error) {
        console.error('Error creating post:', error);
        res.status(500).json({ error: 'Failed to create post' });
    }
});
exports.default = router;
