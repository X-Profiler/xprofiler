import React, { useState } from 'react';
import { useStore } from '../store';
import { Plus, Gift, CreditCard, ShoppingBag, Edit, Trash } from 'lucide-react';

export const Rights = () => {
  const { rights, addRight } = useStore();
  const [showModal, setShowModal] = useState(false);
  const [newRight, setNewRight] = useState({
    name: '',
    type: 'coupon',
    amount: '',
    validity: '',
    stock: ''
  });

  const handleCreate = (e: React.FormEvent) => {
    e.preventDefault();
    addRight({
      id: Date.now().toString(),
      name: newRight.name,
      type: newRight.type as 'coupon' | 'points' | 'goods',
      config: {
        amount: Number(newRight.amount),
        validity: Number(newRight.validity)
      },
      stock: Number(newRight.stock),
      createdAt: new Date().toISOString().split('T')[0]
    });
    setShowModal(false);
    setNewRight({ name: '', type: 'coupon', amount: '', validity: '', stock: '' });
  };

  const getIcon = (type: string) => {
    switch (type) {
      case 'coupon': return <CreditCard className="text-blue-500" />;
      case 'points': return <Gift className="text-yellow-500" />;
      case 'goods': return <ShoppingBag className="text-purple-500" />;
      default: return <Gift />;
    }
  };

  const getLabel = (type: string) => {
    switch (type) {
      case 'coupon': return '优惠券';
      case 'points': return '积分';
      case 'goods': return '实物商品';
      default: return '未知';
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h2 className="text-2xl font-bold text-gray-800">权益配置中心</h2>
        <button 
          onClick={() => setShowModal(true)}
          className="bg-blue-600 text-white px-4 py-2 rounded-lg flex items-center gap-2 hover:bg-blue-700 transition-colors shadow-sm"
        >
          <Plus size={18} />
          新增权益
        </button>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {rights.map((right) => (
          <div key={right.id} className="bg-white p-6 rounded-xl shadow-sm border border-gray-100 flex items-start gap-4 hover:shadow-md transition-shadow">
            <div className="w-12 h-12 bg-gray-50 rounded-lg flex items-center justify-center shrink-0">
              {getIcon(right.type)}
            </div>
            <div className="flex-1">
              <div className="flex justify-between items-start">
                <div>
                  <h3 className="text-lg font-semibold text-gray-800">{right.name}</h3>
                  <span className="text-xs text-gray-500 bg-gray-100 px-2 py-0.5 rounded-full mt-1 inline-block">
                    {getLabel(right.type)}
                  </span>
                </div>
                <div className="flex gap-2">
                  <button className="text-gray-400 hover:text-blue-600 p-1"><Edit size={16} /></button>
                  <button className="text-gray-400 hover:text-red-600 p-1"><Trash size={16} /></button>
                </div>
              </div>
              
              <div className="mt-4 grid grid-cols-2 gap-4 text-sm text-gray-600">
                {right.type === 'coupon' && (
                  <>
                    <div className="flex flex-col">
                      <span className="text-gray-400 text-xs">面额</span>
                      <span className="font-medium">¥ {right.config.amount}</span>
                    </div>
                    <div className="flex flex-col">
                      <span className="text-gray-400 text-xs">有效期</span>
                      <span className="font-medium">{right.config.validity} 天</span>
                    </div>
                  </>
                )}
                {right.type === 'points' && (
                   <div className="flex flex-col">
                      <span className="text-gray-400 text-xs">积分数值</span>
                      <span className="font-medium">{right.config.amount} 积分</span>
                   </div>
                )}
                 <div className="flex flex-col">
                    <span className="text-gray-400 text-xs">库存</span>
                    <span className="font-medium">{right.stock.toLocaleString()}</span>
                 </div>
              </div>
            </div>
          </div>
        ))}
      </div>

      {showModal && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
          <div className="bg-white rounded-2xl w-full max-w-lg shadow-2xl p-8 animate-in fade-in zoom-in duration-200">
            <h3 className="text-xl font-bold mb-6 text-gray-800">新增权益</h3>
            <form onSubmit={handleCreate} className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">权益名称</label>
                <input 
                  required
                  value={newRight.name}
                  onChange={e => setNewRight({...newRight, name: e.target.value})}
                  className="w-full px-4 py-2 border border-gray-200 rounded-lg focus:ring-2 focus:ring-blue-100 focus:border-blue-400 outline-none"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">类型</label>
                <select 
                  value={newRight.type}
                  onChange={e => setNewRight({...newRight, type: e.target.value})}
                  className="w-full px-4 py-2 border border-gray-200 rounded-lg focus:ring-2 focus:ring-blue-100 focus:border-blue-400 outline-none"
                >
                  <option value="coupon">优惠券</option>
                  <option value="points">积分</option>
                  <option value="goods">实物商品</option>
                </select>
              </div>
              
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    {newRight.type === 'coupon' ? '面额 (元)' : '数值'}
                  </label>
                  <input 
                    type="number"
                    required
                    value={newRight.amount}
                    onChange={e => setNewRight({...newRight, amount: e.target.value})}
                    className="w-full px-4 py-2 border border-gray-200 rounded-lg focus:ring-2 focus:ring-blue-100 focus:border-blue-400 outline-none"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">库存</label>
                  <input 
                    type="number"
                    required
                    value={newRight.stock}
                    onChange={e => setNewRight({...newRight, stock: e.target.value})}
                    className="w-full px-4 py-2 border border-gray-200 rounded-lg focus:ring-2 focus:ring-blue-100 focus:border-blue-400 outline-none"
                  />
                </div>
              </div>

              {newRight.type === 'coupon' && (
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">有效期 (天)</label>
                  <input 
                    type="number"
                    required
                    value={newRight.validity}
                    onChange={e => setNewRight({...newRight, validity: e.target.value})}
                    className="w-full px-4 py-2 border border-gray-200 rounded-lg focus:ring-2 focus:ring-blue-100 focus:border-blue-400 outline-none"
                  />
                </div>
              )}

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
