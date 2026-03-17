import React from 'react';
import { useStore } from '../store';
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  BarChart,
  Bar,
  Legend
} from 'recharts';
import { Users, CreditCard, Activity, TrendingUp } from 'lucide-react';
import clsx from 'clsx';

const StatCard = ({ title, value, icon: Icon, change, changeType }: any) => (
  <div className="bg-white p-6 rounded-xl shadow-sm border border-gray-100 flex items-center justify-between">
    <div>
      <p className="text-sm font-medium text-gray-500 mb-1">{title}</p>
      <h3 className="text-2xl font-bold text-gray-900">{value}</h3>
      <div className={clsx("flex items-center gap-1 mt-2 text-xs font-medium", 
        changeType === 'positive' ? "text-green-600" : "text-red-600"
      )}>
        {changeType === 'positive' ? <TrendingUp size={14} /> : <TrendingUp size={14} className="rotate-180" />}
        <span>{change}</span>
      </div>
    </div>
    <div className="w-12 h-12 bg-blue-50 rounded-lg flex items-center justify-center text-blue-600">
      <Icon size={24} />
    </div>
  </div>
);

const data = [
  { name: 'Mon', uv: 4000, pv: 2400, amt: 2400 },
  { name: 'Tue', uv: 3000, pv: 1398, amt: 2210 },
  { name: 'Wed', uv: 2000, pv: 9800, amt: 2290 },
  { name: 'Thu', uv: 2780, pv: 3908, amt: 2000 },
  { name: 'Fri', uv: 1890, pv: 4800, amt: 2181 },
  { name: 'Sat', uv: 2390, pv: 3800, amt: 2500 },
  { name: 'Sun', uv: 3490, pv: 4300, amt: 2100 },
];

export const Dashboard = () => {
  const { campaigns } = useStore();

  return (
    <div className="space-y-6">
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <StatCard title="今日触达用户" value="12,543" icon={Users} change="+12.5%" changeType="positive" />
        <StatCard title="权益发放总额" value="¥ 45,230" icon={CreditCard} change="+8.2%" changeType="positive" />
        <StatCard title="活跃活动数" value={campaigns.filter(c => c.status === 'online').length} icon={Activity} change="-1" changeType="negative" />
        <StatCard title="整体转化率" value="3.2%" icon={TrendingUp} change="+0.4%" changeType="positive" />
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div className="lg:col-span-2 bg-white p-6 rounded-xl shadow-sm border border-gray-100">
          <div className="flex items-center justify-between mb-6">
            <h3 className="text-lg font-semibold text-gray-800">触达效果趋势</h3>
            <select className="text-sm border-gray-300 rounded-md shadow-sm focus:border-blue-300 focus:ring focus:ring-blue-200 focus:ring-opacity-50">
              <option>近 7 天</option>
              <option>近 30 天</option>
            </select>
          </div>
          <div className="h-80">
            <ResponsiveContainer width="100%" height="100%">
              <LineChart data={data}>
                <CartesianGrid strokeDasharray="3 3" vertical={false} stroke="#f0f0f0" />
                <XAxis dataKey="name" axisLine={false} tickLine={false} tick={{fill: '#9ca3af', fontSize: 12}} dy={10} />
                <YAxis axisLine={false} tickLine={false} tick={{fill: '#9ca3af', fontSize: 12}} />
                <Tooltip 
                  contentStyle={{borderRadius: '8px', border: 'none', boxShadow: '0 4px 6px -1px rgba(0, 0, 0, 0.1)'}}
                  cursor={{stroke: '#e5e7eb', strokeWidth: 1}}
                />
                <Legend />
                <Line type="monotone" dataKey="pv" stroke="#3b82f6" strokeWidth={3} dot={{r: 4, fill: '#3b82f6', strokeWidth: 2, stroke: '#fff'}} activeDot={{r: 6}} name="浏览量" />
                <Line type="monotone" dataKey="uv" stroke="#10b981" strokeWidth={3} dot={{r: 4, fill: '#10b981', strokeWidth: 2, stroke: '#fff'}} name="参与人数" />
              </LineChart>
            </ResponsiveContainer>
          </div>
        </div>

        <div className="bg-white p-6 rounded-xl shadow-sm border border-gray-100">
          <h3 className="text-lg font-semibold text-gray-800 mb-6">渠道转化对比</h3>
          <div className="h-80">
            <ResponsiveContainer width="100%" height="100%">
              <BarChart data={data}>
                <CartesianGrid strokeDasharray="3 3" vertical={false} stroke="#f0f0f0" />
                <XAxis dataKey="name" axisLine={false} tickLine={false} tick={{fill: '#9ca3af', fontSize: 12}} dy={10} />
                <YAxis axisLine={false} tickLine={false} tick={{fill: '#9ca3af', fontSize: 12}} />
                <Tooltip cursor={{fill: '#f9fafb'}} contentStyle={{borderRadius: '8px', border: 'none', boxShadow: '0 4px 6px -1px rgba(0, 0, 0, 0.1)'}} />
                <Bar dataKey="amt" fill="#8b5cf6" radius={[4, 4, 0, 0]} name="转化金额" />
              </BarChart>
            </ResponsiveContainer>
          </div>
        </div>
      </div>
    </div>
  );
};
