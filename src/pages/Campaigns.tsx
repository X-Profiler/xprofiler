import React, { useState } from 'react';
import { useStore } from '../store';
import { Plus, Play, Pause, BarChart2, Edit, Trash, Activity } from 'lucide-react';
import clsx from 'clsx';

export const Campaigns = () => {
  const { campaigns, updateCampaign, deleteCampaign, addCampaign } = useStore();
  const [showModal, setShowModal] = useState(false);
  const [newCampaign, setNewCampaign] = useState({
    name: '',
    startTime: '',
    endTime: ''
  });

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'online': return 'bg-green-100 text-green-700 border-green-200';
      case 'testing': return 'bg-yellow-100 text-yellow-700 border-yellow-200';
      case 'draft': return 'bg-gray-100 text-gray-700 border-gray-200';
      case 'offline': return 'bg-red-100 text-red-700 border-red-200';
      default: return 'bg-gray-100 text-gray-700';
    }
  };

  const getStatusLabel = (status: string) => {
    switch (status) {
      case 'online': return '运行中';
      case 'testing': return '测试中';
      case 'draft': return '草稿';
      case 'offline': return '已下线';
      default: return '未知';
    }
  };

  const handleCreate = (e: React.FormEvent) => {
    e.preventDefault();
    addCampaign({
      id: Date.now().toString(),
      name: newCampaign.name,
      status: 'draft',
      audienceId: '',
      ruleId: '',
      touchpointId: '',
      rightsId: '',
      startTime: newCampaign.startTime,
      endTime: newCampaign.endTime,
      metrics: { participants: 0, conversion: 0, roi: 0 },
      logs: []
    });
    setShowModal(false);
    setNewCampaign({ name: '', startTime: '', endTime: '' });
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h2 className="text-2xl font-bold text-gray-800">活动管理</h2>
        <button 
          onClick={() => setShowModal(true)}
          className="bg-blue-600 text-white px-4 py-2 rounded-lg flex items-center gap-2 hover:bg-blue-700 transition-colors shadow-sm"
        >
          <Plus size={18} />
          新建活动
        </button>
      </div>

      <div className="bg-white rounded-xl shadow-sm border border-gray-200 overflow-hidden">
        <table className="w-full text-left border-collapse">
          <thead className="bg-gray-50 border-b border-gray-200 text-gray-500 text-xs uppercase font-medium">
            <tr>
              <th className="px-6 py-4">活动名称</th>
              <th className="px-6 py-4">状态</th>
              <th className="px-6 py-4">时间范围</th>
              <th className="px-6 py-4 text-center">参与人数</th>
              <th className="px-6 py-4 text-center">转化率</th>
              <th className="px-6 py-4 text-center">ROI</th>
              <th className="px-6 py-4 text-right">操作</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-100">
            {campaigns.map((campaign) => (
              <tr key={campaign.id} className="hover:bg-gray-50 transition-colors group">
                <td className="px-6 py-4">
                  <div className="font-medium text-gray-900">{campaign.name}</div>
                  <div className="text-xs text-gray-400 mt-0.5">ID: {campaign.id}</div>
                </td>
                <td className="px-6 py-4">
                  <span className={clsx("px-2.5 py-1 rounded-full text-xs font-medium border", getStatusColor(campaign.status))}>
                    {getStatusLabel(campaign.status)}
                  </span>
                </td>
                <td className="px-6 py-4 text-sm text-gray-500">
                  <div>{campaign.startTime}</div>
                  <div className="text-gray-400 text-xs">至 {campaign.endTime}</div>
                </td>
                <td className="px-6 py-4 text-center font-medium text-gray-700">
                  {campaign.metrics?.participants.toLocaleString()}
                </td>
                <td className="px-6 py-4 text-center font-medium text-gray-700">
                  {campaign.metrics?.conversion}%
                </td>
                <td className="px-6 py-4 text-center font-medium text-gray-700">
                  {campaign.metrics?.roi}%
                </td>
                <td className="px-6 py-4 text-right">
                  <div className="flex items-center justify-end gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
                    {campaign.status !== 'online' && (
                       <button 
                         onClick={() => updateCampaign(campaign.id, { status: 'online' })}
                         className="p-1.5 text-green-600 hover:bg-green-50 rounded-lg tooltip"
                         title="上线"
                       >
                         <Play size={16} />
                       </button>
                    )}
                    {campaign.status === 'online' && (
                       <button 
                         onClick={() => updateCampaign(campaign.id, { status: 'offline' })}
                         className="p-1.5 text-yellow-600 hover:bg-yellow-50 rounded-lg tooltip"
                         title="下线"
                       >
                         <Pause size={16} />
                       </button>
                    )}
                    <button className="p-1.5 text-blue-600 hover:bg-blue-50 rounded-lg tooltip" title="编辑">
                      <Edit size={16} />
                    </button>
                    <button className="p-1.5 text-purple-600 hover:bg-purple-50 rounded-lg tooltip" title="报表">
                      <BarChart2 size={16} />
                    </button>
                    <button 
                      onClick={() => deleteCampaign(campaign.id)}
                      className="p-1.5 text-red-600 hover:bg-red-50 rounded-lg tooltip" 
                      title="删除"
                    >
                      <Trash size={16} />
                    </button>
                  </div>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {showModal && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
          <div className="bg-white rounded-2xl w-full max-w-lg shadow-2xl p-8 animate-in fade-in zoom-in duration-200">
            <h3 className="text-xl font-bold mb-6 text-gray-800">新建活动</h3>
            <form onSubmit={handleCreate} className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">活动名称</label>
                <input 
                  required
                  value={newCampaign.name}
                  onChange={e => setNewCampaign({...newCampaign, name: e.target.value})}
                  className="w-full px-4 py-2 border border-gray-200 rounded-lg focus:ring-2 focus:ring-blue-100 focus:border-blue-400 outline-none"
                />
              </div>
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">开始时间</label>
                  <input 
                    type="date"
                    required
                    value={newCampaign.startTime}
                    onChange={e => setNewCampaign({...newCampaign, startTime: e.target.value})}
                    className="w-full px-4 py-2 border border-gray-200 rounded-lg focus:ring-2 focus:ring-blue-100 focus:border-blue-400 outline-none"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">结束时间</label>
                  <input 
                    type="date"
                    required
                    value={newCampaign.endTime}
                    onChange={e => setNewCampaign({...newCampaign, endTime: e.target.value})}
                    className="w-full px-4 py-2 border border-gray-200 rounded-lg focus:ring-2 focus:ring-blue-100 focus:border-blue-400 outline-none"
                  />
                </div>
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
