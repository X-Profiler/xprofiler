import React, { useState } from 'react';
import { useStore } from '../store';
import { Plus, Edit, Trash, GitBranch } from 'lucide-react';
import { useNavigate } from 'react-router-dom';

export const Rules = () => {
  const { rules } = useStore();
  const navigate = useNavigate();

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h2 className="text-2xl font-bold text-gray-800">规则引擎配置</h2>
        <button 
          onClick={() => navigate('/rules/new')}
          className="bg-blue-600 text-white px-4 py-2 rounded-lg flex items-center gap-2 hover:bg-blue-700 transition-colors shadow-sm"
        >
          <Plus size={18} />
          新建规则
        </button>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {rules.map((rule) => (
          <div key={rule.id} className="bg-white p-6 rounded-xl shadow-sm border border-gray-100 hover:shadow-md transition-shadow group">
            <div className="flex justify-between items-start mb-4">
              <div className="flex items-center gap-3">
                <div className="w-10 h-10 bg-blue-50 text-blue-600 rounded-lg flex items-center justify-center">
                  <GitBranch size={20} />
                </div>
                <div>
                  <h3 className="text-lg font-semibold text-gray-800 group-hover:text-blue-600 transition-colors">{rule.name}</h3>
                  <span className="text-xs text-gray-500">{rule.nodes.length} 个节点, {rule.edges.length} 条连线</span>
                </div>
              </div>
              <div className="flex gap-2">
                <button 
                  onClick={() => navigate(`/rules/edit/${rule.id}`)}
                  className="text-gray-400 hover:text-blue-600 p-2 hover:bg-blue-50 rounded-lg transition-colors"
                >
                  <Edit size={18} />
                </button>
                <button className="text-gray-400 hover:text-red-600 p-2 hover:bg-red-50 rounded-lg transition-colors">
                  <Trash size={18} />
                </button>
              </div>
            </div>
            
            <div className="bg-gray-50 rounded-lg p-3 text-sm text-gray-600 font-mono">
              <div className="flex items-center gap-2 mb-1">
                <span className="w-2 h-2 rounded-full bg-green-500"></span>
                Input: 开始
              </div>
              {rule.nodes.slice(1, 3).map((node, i) => (
                <div key={i} className="flex items-center gap-2 ml-4 border-l-2 border-gray-200 pl-2">
                  <span className="w-1.5 h-1.5 rounded-full bg-gray-400"></span>
                  {node.data.label}
                </div>
              ))}
              {rule.nodes.length > 3 && (
                <div className="ml-6 text-xs text-gray-400">...</div>
              )}
            </div>

            <div className="mt-4 pt-4 border-t border-gray-50 flex justify-between text-sm text-gray-400">
              <span>ID: {rule.id}</span>
              <span>{rule.createdAt}</span>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
