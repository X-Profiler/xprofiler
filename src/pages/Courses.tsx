import React, { useState } from 'react';
import { motion } from 'framer-motion';
import { Search, Filter, Star, Clock, BookOpen, ChevronRight } from 'lucide-react';
import { useNavigate } from 'react-router-dom';

const MOCK_COURSES = [
  { id: 1, title: '初级英语：日常寒暄', lang: 'EN', level: '初级', rating: 4.8, students: '12k', image: 'https://images.unsplash.com/photo-1523240795612-9a054b0db644?auto=format&fit=crop&w=800&q=80' },
  { id: 2, title: '商务英语：邮件与会议', lang: 'EN', level: '高级', rating: 4.9, students: '8.5k', image: 'https://images.unsplash.com/photo-1664575602554-2087b04935a5?auto=format&fit=crop&w=800&q=80' },
  { id: 3, title: '标准日语：五十音图', lang: 'JP', level: '初级', rating: 4.7, students: '25k', image: 'https://images.unsplash.com/photo-1528360983277-13d401cdc186?auto=format&fit=crop&w=800&q=80' },
  { id: 4, title: '韩语进阶：韩剧台词解析', lang: 'KR', level: '中级', rating: 4.9, students: '15k', image: 'https://images.unsplash.com/photo-1580247817119-c6cb496270a6?auto=format&fit=crop&w=800&q=80' },
  { id: 5, title: '雅思听力特训', lang: 'EN', level: '高级', rating: 4.6, students: '5k', image: 'https://images.unsplash.com/photo-1546410531-bea422116065?auto=format&fit=crop&w=800&q=80' },
  { id: 6, title: '日语敬语完全指南', lang: 'JP', level: '高级', rating: 4.8, students: '3.2k', image: 'https://images.unsplash.com/photo-1493976040374-85c8e12f0c0e?auto=format&fit=crop&w=800&q=80' },
];

const Courses = () => {
  const [activeLang, setActiveLang] = useState('All');
  const [activeLevel, setActiveLevel] = useState('All');
  const navigate = useNavigate();

  const filteredCourses = MOCK_COURSES.filter(course => {
    if (activeLang !== 'All' && course.lang !== activeLang) return false;
    if (activeLevel !== 'All' && course.level !== activeLevel) return false;
    return true;
  });

  return (
    <div className="p-8 max-w-7xl mx-auto pb-24 md:pb-8">
      {/* Header & Search */}
      <div className="flex flex-col md:flex-row justify-between items-start md:items-center gap-4 mb-8">
        <div>
          <h2 className="text-3xl font-bold text-slate-800">课程中心</h2>
          <p className="text-slate-500 mt-1">探索适合你的完美课程</p>
        </div>
        
        <div className="flex items-center gap-3 w-full md:w-auto">
          <div className="relative flex-1 md:w-64">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-slate-400" />
            <input 
              type="text" 
              placeholder="搜索课程名称或关键词..." 
              className="w-full pl-10 pr-4 py-2.5 rounded-xl border border-slate-200 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
            />
          </div>
          <button className="p-2.5 rounded-xl border border-slate-200 text-slate-600 hover:bg-slate-50 transition-colors">
            <Filter className="w-5 h-5" />
          </button>
        </div>
      </div>

      {/* Filters */}
      <div className="flex flex-wrap gap-6 mb-8">
        <div className="space-y-2">
          <div className="text-sm font-semibold text-slate-500">语种</div>
          <div className="flex gap-2">
            {['All', 'EN', 'JP', 'KR'].map(lang => (
              <button
                key={lang}
                onClick={() => setActiveLang(lang)}
                className={`px-4 py-1.5 rounded-lg text-sm font-medium transition-all ${
                  activeLang === lang 
                    ? 'bg-blue-600 text-white shadow-md' 
                    : 'bg-white text-slate-600 border border-slate-200 hover:bg-slate-50'
                }`}
              >
                {lang === 'All' ? '全部' : lang}
              </button>
            ))}
          </div>
        </div>
        
        <div className="space-y-2">
          <div className="text-sm font-semibold text-slate-500">难度级别</div>
          <div className="flex gap-2">
            {['All', '初级', '中级', '高级'].map(level => (
              <button
                key={level}
                onClick={() => setActiveLevel(level)}
                className={`px-4 py-1.5 rounded-lg text-sm font-medium transition-all ${
                  activeLevel === level 
                    ? 'bg-slate-800 text-white shadow-md' 
                    : 'bg-white text-slate-600 border border-slate-200 hover:bg-slate-50'
                }`}
              >
                {level === 'All' ? '全部' : level}
              </button>
            ))}
          </div>
        </div>
      </div>

      {/* Course Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {filteredCourses.map((course, index) => (
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: index * 0.1 }}
            key={course.id}
            className="group bg-white rounded-2xl overflow-hidden border border-slate-100 shadow-sm hover:shadow-xl transition-all duration-300 flex flex-col cursor-pointer"
            onClick={() => navigate(`/learn/${course.id}`)}
          >
            <div className="relative h-48 overflow-hidden">
              <div className="absolute top-3 left-3 z-10 flex gap-2">
                <span className="px-2 py-1 bg-white/90 backdrop-blur text-xs font-bold rounded-md text-slate-800">
                  {course.lang}
                </span>
                <span className="px-2 py-1 bg-blue-600/90 backdrop-blur text-xs font-bold rounded-md text-white">
                  {course.level}
                </span>
              </div>
              <img 
                src={course.image} 
                alt={course.title} 
                className="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500"
              />
            </div>
            
            <div className="p-5 flex-1 flex flex-col">
              <h3 className="text-lg font-bold text-slate-800 mb-2 line-clamp-2 group-hover:text-blue-600 transition-colors">
                {course.title}
              </h3>
              
              <div className="mt-auto pt-4 flex items-center justify-between text-sm text-slate-500">
                <div className="flex items-center gap-4">
                  <div className="flex items-center gap-1">
                    <Star className="w-4 h-4 text-amber-400 fill-amber-400" />
                    <span className="font-medium text-slate-700">{course.rating}</span>
                  </div>
                  <div className="flex items-center gap-1">
                    <BookOpen className="w-4 h-4" />
                    <span>{course.students} 人学过</span>
                  </div>
                </div>
                <div className="w-8 h-8 rounded-full bg-blue-50 flex items-center justify-center text-blue-600 group-hover:bg-blue-600 group-hover:text-white transition-colors">
                  <ChevronRight className="w-5 h-5" />
                </div>
              </div>
            </div>
          </motion.div>
        ))}
      </div>
    </div>
  );
};

export default Courses;
