import React from 'react';
import { useStore } from '../store';
import { motion } from 'framer-motion';
import { Play, Flame, Trophy, Target, ArrowRight, Award } from 'lucide-react';
import { 
  Radar, RadarChart, PolarGrid, PolarAngleAxis, PolarRadiusAxis, ResponsiveContainer,
  LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip
} from 'recharts';

const radarData = [
  { subject: '词汇', A: 80, fullMark: 100 },
  { subject: '语法', A: 65, fullMark: 100 },
  { subject: '听力', A: 90, fullMark: 100 },
  { subject: '口语', A: 50, fullMark: 100 },
  { subject: '阅读', A: 85, fullMark: 100 },
];

const activityData = [
  { name: '周一', hours: 1.5 },
  { name: '周二', hours: 2 },
  { name: '周三', hours: 0.5 },
  { name: '周四', hours: 3 },
  { name: '周五', hours: 2.5 },
  { name: '周六', hours: 4 },
  { name: '周日', hours: 3.5 },
];

const Dashboard = () => {
  const user = useStore((state) => state.user);

  return (
    <div className="p-8 max-w-7xl mx-auto space-y-8 pb-24 md:pb-8">
      {/* Welcome Section */}
      <div className="flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
        <div>
          <h2 className="text-3xl font-bold text-slate-800">欢迎回来, {user?.name} 👋</h2>
          <p className="text-slate-500 mt-1">今天是你连续学习的第 <span className="font-bold text-amber-500">14</span> 天！继续保持！</p>
        </div>
        <button className="btn-primary flex items-center gap-2">
          <Play className="w-4 h-4 fill-current" />
          继续学习: 中级{user?.currentLanguage}口语
        </button>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Left Column - Stats */}
        <div className="lg:col-span-2 space-y-6">
          {/* Quick Stats */}
          <div className="grid grid-cols-2 sm:grid-cols-4 gap-4">
            {[
              { label: '本周时长', value: '17h', icon: Target, color: 'text-blue-500', bg: 'bg-blue-50' },
              { label: '掌握词汇', value: '1,240', icon: Trophy, color: 'text-amber-500', bg: 'bg-amber-50' },
              { label: '连续打卡', value: '14天', icon: Flame, color: 'text-red-500', bg: 'bg-red-50' },
              { label: '当前等级', value: 'B2', icon: Award, color: 'text-purple-500', bg: 'bg-purple-50' },
            ].map((stat, i) => (
              <motion.div 
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: i * 0.1 }}
                key={stat.label} 
                className="bg-white rounded-2xl p-4 shadow-sm border border-slate-100 flex flex-col items-center justify-center text-center gap-2"
              >
                <div className={`p-3 rounded-xl ${stat.bg}`}>
                  <stat.icon className={`w-6 h-6 ${stat.color}`} />
                </div>
                <div>
                  <div className="text-2xl font-bold text-slate-800">{stat.value}</div>
                  <div className="text-xs text-slate-500 font-medium">{stat.label}</div>
                </div>
              </motion.div>
            ))}
          </div>

          {/* Activity Chart */}
          <div className="bg-white rounded-2xl p-6 shadow-sm border border-slate-100">
            <div className="flex justify-between items-center mb-6">
              <h3 className="text-lg font-bold text-slate-800">学习时长趋势</h3>
              <select className="text-sm border-slate-200 rounded-lg text-slate-600 outline-none focus:ring-2 focus:ring-blue-500">
                <option>最近一周</option>
                <option>最近一月</option>
              </select>
            </div>
            <div className="h-64 w-full">
              <ResponsiveContainer width="100%" height="100%">
                <LineChart data={activityData}>
                  <CartesianGrid strokeDasharray="3 3" vertical={false} stroke="#f1f5f9" />
                  <XAxis dataKey="name" axisLine={false} tickLine={false} tick={{fill: '#94a3b8', fontSize: 12}} dy={10} />
                  <YAxis axisLine={false} tickLine={false} tick={{fill: '#94a3b8', fontSize: 12}} dx={-10} />
                  <Tooltip 
                    contentStyle={{ borderRadius: '12px', border: 'none', boxShadow: '0 4px 6px -1px rgb(0 0 0 / 0.1)' }}
                  />
                  <Line type="monotone" dataKey="hours" stroke="#3b82f6" strokeWidth={4} dot={{r: 4, strokeWidth: 2}} activeDot={{r: 6}} />
                </LineChart>
              </ResponsiveContainer>
            </div>
          </div>
        </div>

        {/* Right Column - Radar & Path */}
        <div className="space-y-6">
          {/* Radar Chart */}
          <div className="bg-white rounded-2xl p-6 shadow-sm border border-slate-100">
            <h3 className="text-lg font-bold text-slate-800 mb-2">能力雷达图</h3>
            <div className="h-64 w-full">
              <ResponsiveContainer width="100%" height="100%">
                <RadarChart cx="50%" cy="50%" outerRadius="70%" data={radarData}>
                  <PolarGrid stroke="#e2e8f0" />
                  <PolarAngleAxis dataKey="subject" tick={{fill: '#64748b', fontSize: 12, fontWeight: 600}} />
                  <PolarRadiusAxis angle={30} domain={[0, 100]} tick={false} axisLine={false} />
                  <Radar name="能力值" dataKey="A" stroke="#3b82f6" fill="#3b82f6" fillOpacity={0.3} />
                </RadarChart>
              </ResponsiveContainer>
            </div>
            <div className="text-center mt-2 text-sm text-slate-500">
              你的<span className="font-bold text-blue-600">听力</span>表现优异，建议加强<span className="font-bold text-amber-500">口语</span>练习。
            </div>
          </div>

          {/* Recommended Path */}
          <div className="bg-gradient-to-br from-slate-800 to-slate-900 rounded-2xl p-6 shadow-lg text-white relative overflow-hidden">
            <div className="absolute top-0 right-0 w-32 h-32 bg-white/5 rounded-full blur-2xl -mr-10 -mt-10"></div>
            <h3 className="text-lg font-bold mb-4 flex items-center gap-2">
              <Target className="w-5 h-5 text-blue-400" />
              个性化推荐路径
            </h3>
            
            <div className="space-y-4 relative z-10">
              <div className="bg-white/10 rounded-xl p-4 border border-white/10 hover:bg-white/20 transition cursor-pointer">
                <div className="text-xs text-blue-300 font-semibold mb-1">下一步</div>
                <div className="font-medium text-lg flex justify-between items-center">
                  场景对话：在餐厅点餐
                  <ArrowRight className="w-5 h-5" />
                </div>
                <div className="w-full bg-slate-700 h-1.5 rounded-full mt-3">
                  <div className="bg-blue-400 h-1.5 rounded-full" style={{ width: '0%' }}></div>
                </div>
              </div>
              
              <div className="bg-white/5 rounded-xl p-4 border border-white/5 opacity-70">
                <div className="text-xs text-slate-400 font-semibold mb-1">待解锁</div>
                <div className="font-medium text-slate-300">
                  语法精讲：虚拟语气基础
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Dashboard;
