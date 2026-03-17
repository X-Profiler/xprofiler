import React, { useState } from 'react';
import { useStore } from '../store';
import { Plus, Search, Filter, Upload, FileText } from 'lucide-react';

export const Audience = () => {
  const { audiences, addAudience } = useStore();
  const [showModal, setShowModal] = useState(false);
  const [newAudience, setNewAudience] = useState({
    name: '',
    description: '',
    tags: '',
    behaviors: ''
  });

  const handleCreate = (e: React.FormEvent) => {
    e.preventDefault();
    addAudience({
      id: Date.now().toString(),
      name: newAudience.name,
      description: newAudience.description,
      filters: {
        tags: newAudience.tags.split(',').map(t => t.trim()),
        behaviors: newAudience.behaviors.split(',').map(b => b.trim()),
      },
      count: Math.floor(Math.random() * 10000),
      createdAt: new Date().toISOString().split('T')[0]
    });
    setShowModal(false);
    setNewAudience({ name: '', description: '', tags: '', behaviors: '' });
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h2 className="text-2xl font-bold text-gray-800">人群圈选</h2>
        <button 
          onClick={() => setShowModal(true)}
          className="bg-blue-600 text-white px-4 py-2 rounded-lg flex items-center gap-2 hover:bg-blue-700 transition-colors shadow-sm"
        >
          <Plus size={18} />
          新建人群
        </button>
      </div>

      <div className="bg-white p-4 rounded-xl shadow-sm border border-gray-100 flex gap-4">
        <div className="relative flex-1">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400" size={18} />
          <input 
            type="text" 
            placeholder="搜索人群名称或ID..." 
            className="w-full pl-10 pr-4 py-2 border border-gray-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-100 focus:border-blue-400 transition-all"
          />
        </div>
        <button className="px-4 py-2 border border-gray-200 rounded-lg text-gray-600 hover:bg-gray-50 flex items-center gap-2">
          <Filter size={18} />
          筛选
        </button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {audiences.map((audience) => (
          <div key={audience.id} className="bg-white p-6 rounded-xl shadow-sm border border-gray-100 hover:shadow-md transition-shadow group">
            <div className="flex justify-between items-start mb-4">
              <div>
                <h3 className="text-lg font-semibold text-gray-800 group-hover:text-blue-600 transition-colors">{audience.name}</h3>
                <p className="text-sm text-gray-500 mt-1">{audience.description}</p>
              </div>
              <span className="bg-blue-50 text-blue-700 text-xs px-2 py-1 rounded-full font-medium">
                {audience.count.toLocaleString()} 人
              </span>
            </div>
            
            <div className="space-y-3">
              <div className="flex flex-wrap gap-2">
                {audience.filters.tags.map((tag, i) => (
                  <span key={i} className="text-xs bg-gray-100 text-gray-600 px-2 py-1 rounded border border-gray-200">
                    #{tag}
                  </span>
                ))}
              </div>
              <div className="flex flex-wrap gap-2">
                {audience.filters.behaviors.map((behavior, i) => (
                  <span key={i} className="text-xs bg-green-50 text-green-700 px-2 py-1 rounded border border-green-100">
                    {behavior}
                  </span>
                ))}
              </div>
            </div>

            <div className="mt-6 pt-4 border-t border-gray-50 flex justify-between text-sm text-gray-400">
              <span>ID: {audience.id}</span>
              <span>{audience.createdAt}</span>
            </div>
          </div>
        ))}
      </div>

      {showModal && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
          <div className="bg-white rounded-2xl w-full max-w-lg shadow-2xl p-8 animate-in fade-in zoom-in duration-200">
            <h3 className="text-xl font-bold mb-6 text-gray-800">新建人群圈选</h3>
            <form onSubmit={handleCreate} className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">人群名称</label>
                <input 
                  required
                  value={newAudience.name}
                  onChange={e => setNewAudience({...newAudience, name: e.target.value})}
                  className="w-full px-4 py-2 border border-gray-200 rounded-lg focus:ring-2 focus:ring-blue-100 focus:border-blue-400 outline-none"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">描述</label>
                <textarea 
                  value={newAudience.description}
                  onChange={e => setNewAudience({...newAudience, description: e.target.value})}
                  className="w-full px-4 py-2 border border-gray-200 rounded-lg focus:ring-2 focus:ring-blue-100 focus:border-blue-400 outline-none"
                  rows={2}
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">标签筛选 (逗号分隔)</label>
                <input 
                  placeholder="如: 男性, 25-30岁, 北京"
                  value={newAudience.tags}
                  onChange={e => setNewAudience({...newAudience, tags: e.target.value})}
                  className="w-full px-4 py-2 border border-gray-200 rounded-lg focus:ring-2 focus:ring-blue-100 focus:border-blue-400 outline-none"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">行为筛选 (逗号分隔)</label>
                <input 
                  placeholder="如: 浏览>5次, 收藏商品"
                  value={newAudience.behaviors}
                  onChange={e => setNewAudience({...newAudience, behaviors: e.target.value})}
                  className="w-full px-4 py-2 border border-gray-200 rounded-lg focus:ring-2 focus:ring-blue-100 focus:border-blue-400 outline-none"
                />
              </div>
              
              <div className="border-t border-gray-100 pt-4 mt-2">
                 <label className="block text-sm font-medium text-gray-700 mb-2">白名单导入</label>
                 <div className="border-2 border-dashed border-gray-200 rounded-lg p-6 flex flex-col items-center justify-center text-gray-400 hover:border-blue-300 hover:bg-blue-50 transition-colors cursor-pointer">
                    <Upload size={24} className="mb-2" />
                    <span className="text-xs">点击上传 Excel / CSV 文件</span>
                 </div>
              </div>

              <div className="flex gap-3 pt-4">
                <button 
                  type="button" 
                  onClick={() => setShowModal(false)}
                  className="flex-1 px-4 py-2 border border-gray-200 rounded-lg text-gray-600 hover:bg-gray-50"
                >
                  取消
                </button>
                <button 
                  type="submit" 
                  className="flex-1 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 shadow-sm"
                >
                  创建
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
};
