import React, { useState } from 'react';
import { motion } from 'framer-motion';
import { Heart, MessageCircle, Share2, Award, TrendingUp, Trophy, Star, Flame } from 'lucide-react';
import { useStore } from '../store';

const POSTS = [
  { id: 1, user: 'Sakura_Learn', avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Sakura', lang: 'JP', time: '2小时前', content: '今天终于突破了N3的听力部分！虽然还有很多生词，但是能听懂大意了，继续加油！💪', likes: 124, comments: 18 },
  { id: 2, user: 'John_Doe', avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=John', lang: 'EN', time: '5小时前', content: '分享一个背单词的小技巧：使用词根词缀法加上平台的闪卡功能，效率翻倍！', likes: 89, comments: 5 },
  { id: 3, user: 'Kim_Min', avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Kim', lang: 'KR', time: '1天前', content: '完成连续30天打卡！获得了"持之以恒"徽章，开心！🎉', likes: 256, comments: 42, image: 'https://images.unsplash.com/photo-1515260268569-9271009adfdb?auto=format&fit=crop&w=600&q=80' },
];

const LEADERBOARD = [
  { rank: 1, name: 'Alex_Master', exp: 15420, avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Alex' },
  { rank: 2, name: 'Lingua_Queen', exp: 14200, avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Queen' },
  { rank: 3, name: 'Poly_glot', exp: 13850, avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Poly' },
  { rank: 4, name: 'Sakura_Learn', exp: 12100, avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Sakura' },
  { rank: 5, name: 'John_Doe', exp: 11500, avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=John' },
];

const Community = () => {
  const user = useStore(state => state.user);
  const [activeTab, setActiveTab] = useState('feed');

  return (
    <div className="p-8 max-w-7xl mx-auto pb-24 md:pb-8 flex flex-col lg:flex-row gap-8">
      {/* Main Content - Feed */}
      <div className="flex-1 space-y-6">
        <div className="flex items-center gap-6 mb-6 border-b border-slate-200 pb-2">
          <button 
            onClick={() => setActiveTab('feed')}
            className={`pb-4 px-2 font-medium text-lg relative ${activeTab === 'feed' ? 'text-blue-600' : 'text-slate-500 hover:text-slate-700'}`}
          >
            最新动态
            {activeTab === 'feed' && (
              <motion.div layoutId="underline" className="absolute bottom-0 left-0 right-0 h-0.5 bg-blue-600 rounded-t-full" />
            )}
          </button>
          <button 
            onClick={() => setActiveTab('achievements')}
            className={`pb-4 px-2 font-medium text-lg relative ${activeTab === 'achievements' ? 'text-blue-600' : 'text-slate-500 hover:text-slate-700'}`}
          >
            我的成就
            {activeTab === 'achievements' && (
              <motion.div layoutId="underline" className="absolute bottom-0 left-0 right-0 h-0.5 bg-blue-600 rounded-t-full" />
            )}
          </button>
        </div>

        {activeTab === 'feed' ? (
          <div className="space-y-6">
            {/* Create Post */}
            <div className="bg-white rounded-2xl p-5 shadow-sm border border-slate-100 flex gap-4">
              <img src={user?.avatar} alt="User" className="w-12 h-12 rounded-full bg-slate-100" />
              <div className="flex-1">
                <textarea 
                  placeholder="分享你的学习心得..." 
                  className="w-full bg-slate-50 rounded-xl p-4 border-none focus:ring-2 focus:ring-blue-100 resize-none h-24 text-slate-700 placeholder:text-slate-400"
                ></textarea>
                <div className="flex justify-between items-center mt-3">
                  <div className="flex gap-2">
                    <button className="text-slate-400 hover:text-blue-500 transition-colors p-2"><TrendingUp className="w-5 h-5" /></button>
                  </div>
                  <button className="btn-primary py-1.5 px-6">发布</button>
                </div>
              </div>
            </div>

            {/* Posts */}
            {POSTS.map((post, i) => (
              <motion.div 
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: i * 0.1 }}
                key={post.id} 
                className="bg-white rounded-2xl p-5 shadow-sm border border-slate-100"
              >
                <div className="flex justify-between items-start mb-4">
                  <div className="flex items-center gap-3">
                    <img src={post.avatar} alt={post.user} className="w-10 h-10 rounded-full bg-slate-100" />
                    <div>
                      <div className="font-semibold text-slate-800 flex items-center gap-2">
                        {post.user}
                        <span className="text-[10px] px-1.5 py-0.5 bg-slate-100 text-slate-500 rounded font-bold">{post.lang}</span>
                      </div>
                      <div className="text-xs text-slate-400">{post.time}</div>
                    </div>
                  </div>
                </div>
                
                <p className="text-slate-700 mb-4 leading-relaxed">{post.content}</p>
                
                {post.image && (
                  <div className="mb-4 rounded-xl overflow-hidden border border-slate-100">
                    <img src={post.image} alt="Post content" className="w-full h-48 object-cover" />
                  </div>
                )}

                <div className="flex items-center gap-6 pt-4 border-t border-slate-50 text-slate-500">
                  <button className="flex items-center gap-1.5 hover:text-red-500 transition-colors">
                    <Heart className="w-5 h-5" /> <span className="text-sm font-medium">{post.likes}</span>
                  </button>
                  <button className="flex items-center gap-1.5 hover:text-blue-500 transition-colors">
                    <MessageCircle className="w-5 h-5" /> <span className="text-sm font-medium">{post.comments}</span>
                  </button>
                  <button className="flex items-center gap-1.5 hover:text-green-500 transition-colors ml-auto">
                    <Share2 className="w-5 h-5" />
                  </button>
                </div>
              </motion.div>
            ))}
          </div>
        ) : (
          <div className="grid grid-cols-2 md:grid-cols-3 gap-6">
            {[
              { icon: Trophy, name: '初学乍练', desc: '完成第一节课', color: 'text-yellow-600', bg: 'bg-yellow-100', active: true },
              { icon: Flame, name: '持之以恒', desc: '连续打卡7天', color: 'text-red-600', bg: 'bg-red-100', active: true },
              { icon: Award, name: '词汇达人', desc: '掌握1000词', color: 'text-purple-600', bg: 'bg-purple-100', active: true },
              { icon: Star, name: '语法大师', desc: '满分通过语法测验', color: 'text-blue-600', bg: 'bg-blue-100', active: false },
              { icon: TrendingUp, name: '口语王者', desc: '口语评分连续A+', color: 'text-green-600', bg: 'bg-green-100', active: false },
            ].map((badge, i) => (
              <motion.div 
                initial={{ opacity: 0, scale: 0.9 }}
                animate={{ opacity: 1, scale: 1 }}
                transition={{ delay: i * 0.1 }}
                key={badge.name}
                className={`bg-white rounded-2xl p-6 shadow-sm border border-slate-100 flex flex-col items-center text-center gap-3 ${badge.active ? '' : 'opacity-50 grayscale'}`}
              >
                <div className={`w-16 h-16 rounded-full flex items-center justify-center ${badge.bg}`}>
                  <badge.icon className={`w-8 h-8 ${badge.color}`} />
                </div>
                <div>
                  <h4 className="font-bold text-slate-800">{badge.name}</h4>
                  <p className="text-xs text-slate-500 mt-1">{badge.desc}</p>
                </div>
              </motion.div>
            ))}
          </div>
        )}
      </div>

      {/* Right Sidebar - Leaderboard */}
      <div className="w-full lg:w-80 space-y-6">
        <div className="bg-white rounded-2xl p-6 shadow-sm border border-slate-100">
          <h3 className="text-lg font-bold text-slate-800 mb-6 flex items-center gap-2">
            <Trophy className="w-5 h-5 text-amber-500" />
            本周学习榜
          </h3>
          
          <div className="space-y-4">
            {LEADERBOARD.map((user, idx) => (
              <div key={user.name} className="flex items-center gap-3">
                <div className={`w-6 text-center font-bold ${
                  idx === 0 ? 'text-yellow-500' : 
                  idx === 1 ? 'text-slate-400' : 
                  idx === 2 ? 'text-amber-600' : 'text-slate-300'
                }`}>
                  {user.rank}
                </div>
                <img src={user.avatar} alt={user.name} className="w-10 h-10 rounded-full bg-slate-100" />
                <div className="flex-1 min-w-0">
                  <div className="font-medium text-slate-800 text-sm truncate">{user.name}</div>
                  <div className="text-xs text-slate-500">{user.exp} EXP</div>
                </div>
              </div>
            ))}
          </div>
          
          <div className="mt-6 pt-4 border-t border-slate-100 flex items-center justify-between text-sm">
            <span className="text-slate-500">我的排名: <strong className="text-slate-800">128</strong></span>
            <span className="text-blue-600 font-medium">{user?.expPoints} EXP</span>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Community;
