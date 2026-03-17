import React, { useCallback } from 'react';
import ReactFlow, {
  MiniMap,
  Controls,
  Background,
  useNodesState,
  useEdgesState,
  addEdge,
  Connection,
  Edge,
  Node,
} from 'reactflow';
import 'reactflow/dist/style.css';
import { useStore } from '../store';
import { useParams, useNavigate } from 'react-router-dom';
import { Save, ArrowLeft } from 'lucide-react';

const initialNodes: Node[] = [
  { id: '1', type: 'input', data: { label: '开始' }, position: { x: 250, y: 5 } },
];

export const RuleEditor = () => {
  const { id } = useParams();
  const navigate = useNavigate();
  const { rules, addRule } = useStore();
  const existingRule = rules.find(r => r.id === id);

  const [nodes, setNodes, onNodesChange] = useNodesState(existingRule?.nodes || initialNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState(existingRule?.edges || []);

  const onConnect = useCallback(
    (params: Connection) => setEdges((eds) => addEdge(params, eds)),
    [setEdges],
  );

  const handleSave = () => {
    // In a real app, we would update the store
    alert('规则保存成功！');
    navigate('/rules');
  };

  return (
    <div className="h-[calc(100vh-100px)] flex flex-col">
      <div className="flex items-center justify-between mb-4 px-2">
        <div className="flex items-center gap-4">
          <button onClick={() => navigate('/rules')} className="p-2 hover:bg-gray-100 rounded-full">
            <ArrowLeft size={20} />
          </button>
          <h2 className="text-xl font-bold text-gray-800">
            {existingRule ? `编辑规则: ${existingRule.name}` : '新建规则'}
          </h2>
        </div>
        <button 
          onClick={handleSave}
          className="bg-blue-600 text-white px-4 py-2 rounded-lg flex items-center gap-2 hover:bg-blue-700 shadow-sm"
        >
          <Save size={18} />
          保存规则
        </button>
      </div>
      
      <div className="flex-1 border border-gray-200 rounded-xl overflow-hidden bg-white shadow-sm">
        <ReactFlow
          nodes={nodes}
          edges={edges}
          onNodesChange={onNodesChange}
          onEdgesChange={onEdgesChange}
          onConnect={onConnect}
          fitView
        >
          <Controls />
          <MiniMap />
          <Background gap={12} size={1} />
        </ReactFlow>
      </div>
    </div>
  );
};
