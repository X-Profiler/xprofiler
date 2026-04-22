import { useState, useEffect } from 'react';
import { useParams, Link } from 'react-router-dom';
import axios from 'axios';

interface Lesson {
  id: number;
  title: string;
  content: string | null;
  order: number;
}

interface Chapter {
  id: number;
  title: string;
  order: number;
  lessons: Lesson[];
}

interface CourseDetailData {
  id: number;
  title: string;
  description: string;
  language: string;
  level: string;
  chapters: Chapter[];
}

export default function CourseDetail() {
  const { id } = useParams<{ id: string }>();
  const [course, setCourse] = useState<CourseDetailData | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string>('');
  
  // Track open chapters (accordion state)
  const [openChapters, setOpenChapters] = useState<Set<number>>(new Set());

  useEffect(() => {
    const fetchCourseDetail = async () => {
      try {
        const response = await axios.get(`/api/courses/${id}`);
        setCourse(response.data);
        
        // Open the first chapter by default if it exists
        if (response.data.chapters && response.data.chapters.length > 0) {
          setOpenChapters(new Set([response.data.chapters[0].id]));
        }
        
        setLoading(false);
      } catch {
        setError('加载课程详情失败');
        setLoading(false);
      }
    };
    if (id) {
      fetchCourseDetail();
    }
  }, [id]);

  const toggleChapter = (chapterId: number) => {
    setOpenChapters(prev => {
      const newSet = new Set(prev);
      if (newSet.has(chapterId)) {
        newSet.delete(chapterId);
      } else {
        newSet.add(chapterId);
      }
      return newSet;
    });
  };

  if (loading) return <div className="flex justify-center items-center h-screen">加载中...</div>;
  if (error || !course) return <div className="flex justify-center items-center h-screen text-red-500">{error || '找不到课程'}</div>;

  return (
    <div className="min-h-screen bg-gray-50 p-8">
      <div className="max-w-4xl mx-auto">
        {/* Header */}
        <div className="bg-white rounded-xl shadow-md p-8 mb-8 border border-gray-100">
          <div className="flex justify-between items-start mb-4">
            <div>
              <div className="flex items-center space-x-3 mb-2">
                <span className="px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-sm font-medium">
                  {course.language}
                </span>
                <span className="px-3 py-1 bg-green-100 text-green-800 rounded-full text-sm font-medium">
                  {course.level}
                </span>
              </div>
              <h1 className="text-3xl font-bold text-gray-900">{course.title}</h1>
            </div>
            <Link to="/course" className="px-4 py-2 bg-gray-100 text-gray-700 rounded hover:bg-gray-200 transition">
              返回课程列表
            </Link>
          </div>
          <p className="text-gray-600 text-lg leading-relaxed">{course.description || '暂无描述'}</p>
        </div>

        {/* Chapters Accordion */}
        <div className="space-y-4">
          <h2 className="text-2xl font-bold text-gray-800 mb-6">课程内容</h2>
          
          {course.chapters && course.chapters.length > 0 ? (
            course.chapters
              .sort((a, b) => a.order - b.order)
              .map((chapter) => {
                const isOpen = openChapters.has(chapter.id);
                return (
                  <div key={chapter.id} className="bg-white rounded-lg shadow-sm border border-gray-200 overflow-hidden">
                    {/* Accordion Header */}
                    <button
                      onClick={() => toggleChapter(chapter.id)}
                      className="w-full flex justify-between items-center p-5 bg-white hover:bg-gray-50 transition-colors focus:outline-none"
                    >
                      <h3 className="text-lg font-semibold text-gray-800 flex items-center">
                        <span className="mr-3 text-blue-500">第 {chapter.order} 章</span>
                        {chapter.title}
                      </h3>
                      <svg 
                        className={`w-6 h-6 text-gray-400 transform transition-transform duration-200 ${isOpen ? 'rotate-180' : ''}`} 
                        fill="none" 
                        stroke="currentColor" 
                        viewBox="0 0 24 24"
                      >
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
                      </svg>
                    </button>
                    
                    {/* Accordion Content */}
                    {isOpen && (
                      <div className="border-t border-gray-100 bg-gray-50">
                        {chapter.lessons && chapter.lessons.length > 0 ? (
                          <ul className="divide-y divide-gray-100">
                            {chapter.lessons
                              .sort((a, b) => a.order - b.order)
                              .map((lesson) => (
                                <li key={lesson.id} className="p-4 hover:bg-gray-100 transition-colors flex items-center group">
                                  <div className="w-8 h-8 rounded-full bg-blue-100 text-blue-600 flex items-center justify-center mr-4 flex-shrink-0 font-medium text-sm">
                                    {lesson.order}
                                  </div>
                                  <div className="flex-grow">
                                    <h4 className="text-gray-800 font-medium group-hover:text-blue-600 transition-colors">{lesson.title}</h4>
                                    {lesson.content && (
                                      <p className="text-gray-500 text-sm mt-1 line-clamp-1">{lesson.content}</p>
                                    )}
                                  </div>
                                  <Link 
                                    to={`/lesson/${lesson.id}`}
                                    className="opacity-0 group-hover:opacity-100 ml-4 px-4 py-1.5 bg-blue-50 text-blue-600 rounded text-sm font-medium hover:bg-blue-100 transition-all block"
                                  >
                                    开始学习
                                  </Link>
                                </li>
                              ))}
                          </ul>
                        ) : (
                          <div className="p-6 text-center text-gray-500">
                            本章暂无课程内容
                          </div>
                        )}
                      </div>
                    )}
                  </div>
                );
              })
          ) : (
            <div className="bg-white rounded-lg shadow-sm p-8 text-center text-gray-500 border border-gray-200">
              该课程尚未添加章节
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
