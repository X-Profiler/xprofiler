"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const express_1 = __importDefault(require("express"));
const cors_1 = __importDefault(require("cors"));
const dotenv_1 = __importDefault(require("dotenv"));
const db_1 = require("./db");
const auth_1 = __importDefault(require("./routes/auth"));
const courses_1 = __importDefault(require("./routes/courses"));
const lessons_1 = __importDefault(require("./routes/lessons"));
const progress_1 = __importDefault(require("./routes/progress"));
const recommendations_1 = __importDefault(require("./routes/recommendations"));
const community_1 = __importDefault(require("./routes/community"));
const badges_1 = __importDefault(require("./routes/badges"));
const authMiddleware_1 = require("./middleware/authMiddleware");
dotenv_1.default.config();
const app = (0, express_1.default)();
const port = process.env.PORT || 3000;
app.use((0, cors_1.default)());
app.use(express_1.default.json());
// Routes
app.use('/api/auth', auth_1.default);
app.use('/api/courses', courses_1.default);
app.use('/api/lessons', lessons_1.default);
app.use('/api/progress', progress_1.default);
app.use('/api/recommendations', recommendations_1.default);
app.use('/api/community', community_1.default);
app.use('/api/badges', badges_1.default);
// Protected route example
app.get('/api/auth/me', authMiddleware_1.authMiddleware, (req, res) => {
    res.json({ message: 'Authenticated', user: req.user });
});
// Health check route
app.get('/health', async (req, res) => {
    try {
        // Check database connection
        await db_1.prisma.$queryRaw `SELECT 1`;
        res.status(200).json({ status: 'ok', database: 'connected' });
    }
    catch (error) {
        console.error('Health check failed:', error);
        res.status(500).json({ status: 'error', database: 'disconnected', error: String(error) });
    }
});
// A simple API to fetch users
app.get('/users', async (req, res) => {
    try {
        const users = await db_1.prisma.user.findMany({
            select: { id: true, email: true, username: true, createdAt: true, updatedAt: true }
        });
        res.json(users);
    }
    catch (error) {
        res.status(500).json({ error: 'Could not fetch users' });
    }
});
app.listen(port, () => {
    console.log(`Server is running at http://localhost:${port}`);
});
