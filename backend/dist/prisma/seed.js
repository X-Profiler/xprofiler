"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
const client_1 = require("@prisma/client");
const adapter_libsql_1 = require("@prisma/adapter-libsql");
const dotenv = __importStar(require("dotenv"));
dotenv.config();
const adapter = new adapter_libsql_1.PrismaLibSql({
    url: process.env.DATABASE_URL || 'file:./dev.db',
});
const prisma = new client_1.PrismaClient({ adapter });
async function main() {
    console.log('Start seeding...');
    // Delete existing data
    await prisma.exercise.deleteMany();
    await prisma.lesson.deleteMany();
    await prisma.chapter.deleteMany();
    await prisma.course.deleteMany();
    // English Course
    const englishCourse = await prisma.course.create({
        data: {
            language: 'English',
            level: 'Beginner',
            title: 'Basic English Greetings',
            description: 'Learn how to greet people in English.',
            chapters: {
                create: [
                    {
                        title: 'Chapter 1: Hello and Goodbye',
                        order: 1,
                        lessons: {
                            create: [
                                {
                                    title: 'Lesson 1: Saying Hello',
                                    content: 'Hello, Hi, Good morning.',
                                    order: 1,
                                    exercises: {
                                        create: [
                                            {
                                                type: 'VOCAB',
                                                question: 'How do you say "Hello"?',
                                                options: JSON.stringify(['Goodbye', 'Hi', 'Thanks', 'Yes']),
                                                correctAnswer: 'Hi',
                                            },
                                            {
                                                type: 'LISTENING',
                                                question: 'What does "Good morning" mean?',
                                                options: JSON.stringify(['Goodbye', 'Good morning', 'Good night', 'Good evening']),
                                                correctAnswer: 'Good morning',
                                            },
                                        ],
                                    },
                                },
                                { title: 'Lesson 2: Saying Goodbye', content: 'Goodbye, Bye, See you later.', order: 2 },
                            ],
                        },
                    },
                ],
            },
        },
    });
    // Japanese Course
    const japaneseCourse = await prisma.course.create({
        data: {
            language: 'Japanese',
            level: 'Beginner',
            title: 'Basic Japanese Greetings',
            description: 'Learn how to greet people in Japanese.',
            chapters: {
                create: [
                    {
                        title: 'Chapter 1: Greetings (Aisatsu)',
                        order: 1,
                        lessons: {
                            create: [
                                {
                                    title: 'Lesson 1: Good Morning',
                                    content: 'Ohayou gozaimasu (おはようございます)',
                                    order: 1,
                                    exercises: {
                                        create: [
                                            {
                                                type: 'VOCAB',
                                                question: 'How do you say "Good Morning" in Japanese?',
                                                options: JSON.stringify(['Konnichiwa', 'Ohayou gozaimasu', 'Arigatou', 'Sayounara']),
                                                correctAnswer: 'Ohayou gozaimasu',
                                            },
                                        ],
                                    },
                                },
                                { title: 'Lesson 2: Hello', content: 'Konnichiwa (こんにちは)', order: 2 },
                            ],
                        },
                    },
                ],
            },
        },
    });
    // Korean Course
    const koreanCourse = await prisma.course.create({
        data: {
            language: 'Korean',
            level: 'Intermediate',
            title: 'Korean Daily Conversations',
            description: 'Learn daily conversations in Korean.',
            chapters: {
                create: [
                    {
                        title: 'Chapter 1: At the Restaurant',
                        order: 1,
                        lessons: {
                            create: [
                                { title: 'Lesson 1: Ordering Food', content: 'Jumunhalgeyo (주문할게요)', order: 1 },
                                { title: 'Lesson 2: Asking for the Bill', content: 'Gyesanhaejuseyo (계산해주세요)', order: 2 },
                            ],
                        },
                    },
                ],
            },
        },
    });
    console.log('Seeding finished.');
}
main()
    .then(async () => {
    await prisma.$disconnect();
})
    .catch(async (e) => {
    console.error(e);
    await prisma.$disconnect();
    throw e;
});
