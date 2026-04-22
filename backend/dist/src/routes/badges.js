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
        const userBadges = await db_1.prisma.userBadge.findMany({
            where: { userId },
            include: { badge: true },
        });
        const allBadges = await db_1.prisma.badge.findMany();
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
    }
    catch (error) {
        console.error('Error fetching badges:', error);
        res.status(500).json({ error: 'Failed to fetch badges' });
    }
});
exports.default = router;
