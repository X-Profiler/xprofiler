import React, { useState } from 'react';
import { useStore } from '../store';
import { Plus, MessageSquare, Mail, MessageCircle, Edit, Trash, Calendar, Clock } from 'lucide-react';

export const Touchpoints = () => {
  const { touchpoints, addTouchpoint } = useStore();
  const [showModal, setShowModal] = useState(false);
  const [newTouchpoint, setNewTouchpoint] = useState({
    name: '',
    channel: 'sms',
    content: '',
    schedule: ''
  });

  const handleCreate = (e: React.FormEvent) => {
    e.preventDefault();
    addTouchpoint({
      id: Date.now().toString(),
      name: newTouchpoint.name,
      channel: newTouchpoint.channel as 'sms' | 'im' | 'inapp',
      content: newTouchpoint.content,
      schedule: newTouchpoint.schedule,
      status: 'active'
    });
    setShowModal(false);
    setNewTouchpoint({ name: '', channel: 'sms', content: '', schedule: '' });
  };

  const getIcon = (channel: string) => {
    switch (channel) {
      case 'sms': return <MessageSquare className="text-blue-500" />;
      case 'im': return <MessageCircle className="text-green-500" />;
      case 'inapp': return <Mail className="text-orange-500" />;
      default: return <MessageSquare />;
    }
  };

  const getLabel = (channel: string) => {
    switch (channel) {
      case 'sms': return '短信';
      case 'im': return '即时消息';
      case 'inapp': return '站内信';
      default: return '未知';
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h2 className="text-2xl font-bold text-gray-800">触达编排</h2>
        <button 
          onClick={() => setShowModal(true)}
          className="bg-blue-600 text-white px-4 py-2 rounded-lg flex items-center gap-2 hover:bg-blue-700 transition-colors shadow-sm"
        >
          <Plus size={18} />
          新建触达
        </button>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {touchpoints.map((tp) => (
          <div key={tp.id} className="bg-white p-6 rounded-xl shadow-sm border border-gray-100 hover:shadow-md transition-shadow group relative overflow-hidden">
            <div className={`absolute top-0 left-0 w-1 h-full ${tp.status === 'active' ? 'bg-green-500' : 'bg-gray-300'}`}></div>
            
            <div className="pl-4 flex justify-between items-start">
              <div className="flex items-start gap-4">
                <div className="w-12 h-12 bg-gray-50 rounded-lg flex items-center justify-center shrink-0">
                  {getIcon(tp.channel)}
                </div>
                <div>
                  <h3 className="text-lg font-semibold text-gray-800 group-hover:text-blue-600 transition-colors">{tp.name}</h3>
                  <span className="text-xs text-gray-500 bg-gray-100 px-2 py-0.5 rounded-full mt-1 inline-block">
                    {getLabel(tp.channel)}
                  </span>
                </div>
              </div>
              <div className="flex gap-2">
                <button className="text-gray-400 hover:text-blue-600 p-1"><Edit size={16} /></button>
                <button className="text-gray-400 hover:text-red-600 p-1"><Trash size={16} /></button>
              </div>
            </div>

            <div className="mt-4 pl-4 bg-gray-50 p-3 rounded-lg text-sm text-gray-700 border-l-4 border-blue-200 italic">
              "{tp.content}"
            </div>

            <div className="mt-4 pl-4 flex items-center gap-4 text-sm text-gray-500">
              <div className="flex items-center gap-1">
                <Calendar size={14} />
                <span>{tp.schedule.split(' ')[0]}</span>
              </div>
              <div className="flex items-center gap-1">
                <Clock size={14} />
                <span>{tp.schedule.split(' ')[1] || 'Trigger'}</span>
              </div>
              <span className={`px-2 py-0.5 rounded-full text-xs font-medium ${tp.status === 'active' ? 'bg-green-100 text-green-700' : 'bg-gray-100 text-gray-600'}`}>
                {tp.status === 'active' ? '运行中' : '已停止'}
              </span>
            </div>
          </div>
        ))}
      </div>

      {showModal && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
          <div className="bg-white rounded-2xl w-full max-w-lg shadow-2xl p-8 animate-in fade-in zoom-in duration-200">
            <h3 className="text-xl font-bold mb-6 text-gray-800">新建触达任务</h3>
            <form onSubmit={handleCreate} className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">任务名称</label>
                <input 
                  required
                  value={newTouchpoint.name}
                  onChange={e => setNewTouchpoint({...newTouchpoint, name: e.target.value})}
                  className="w-full px-4 py-2 border border-gray-200 rounded-lg focus:ring-2 focus:ring-blue-100 focus:border-blue-400 outline-none"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">触达渠道</label>
                <select 
                  value={newTouchpoint.channel}
                  onChange={e => setNewTouchpoint({...newTouchpoint, channel: e.target.value})}
                  className="w-full px-4 py-2 border border-gray-200 rounded-lg focus:ring-2 focus:ring-blue-100 focus:border-blue-400 outline-none"
                >
                  <option value="sms">短信</option>
                  <option value="im">即时消息 (IM)</option>
                  <option value="inapp">站内信</option>
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">发送内容</label>
                <textarea 
                  required
                  value={newTouchpoint.content}
                  onChange={e => setNewTouchpoint({...newTouchpoint, content: e.target.value})}
                  className="w-full px-4 py-2 border border-gray-200 rounded-lg focus:ring-2 focus:ring-blue-100 focus:border-blue-400 outline-none"
                  rows={4}
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">发送时间/触发条件</label>
                <input 
                  placeholder="如: 2023-11-11 10:00 或 注册触发"
                  required
                  value={newTouchpoint.schedule}
                  onChange={e => setNewTouchpoint({...newTouchpoint, schedule: e.target.value})}
                  className="w-full px-4 py-2 border border-gray-200 rounded-lg focus:ring-2 focus:ring-blue-100 focus:border-blue-400 outline-none"
                />
              </div>

              <div className="flex gap-3 pt-6">
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
                  保存
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
};
