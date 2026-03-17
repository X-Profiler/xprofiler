export interface User {
  id: string;
  name: string;
  avatar: string;
  role: 'admin' | 'operator';
}

export interface Audience {
  id: string;
  name: string;
  description: string;
  filters: {
    tags: string[];
    behaviors: string[];
    whitelist?: string[];
  };
  count: number;
  createdAt: string;
}

export interface Right {
  id: string;
  name: string;
  type: 'coupon' | 'points' | 'goods';
  config: {
    amount?: number;
    validity?: number; // days
    skuId?: string;
  };
  stock: number;
  createdAt: string;
}

export interface RuleNode {
  id: string;
  type: string;
  position: { x: number; y: number };
  data: { label: string; [key: string]: any };
}

export interface RuleEdge {
  id: string;
  source: string;
  target: string;
  label?: string;
}

export interface Rule {
  id: string;
  name: string;
  nodes: RuleNode[];
  edges: RuleEdge[];
  createdAt: string;
}

export interface Touchpoint {
  id: string;
  name: string;
  channel: 'sms' | 'im' | 'inapp';
  content: string;
  schedule: string;
  status: 'active' | 'inactive';
}

export interface Campaign {
  id: string;
  name: string;
  status: 'draft' | 'testing' | 'online' | 'offline';
  audienceId: string;
  ruleId: string;
  touchpointId: string;
  rightsId: string;
  startTime: string;
  endTime: string;
  metrics?: {
    participants: number;
    conversion: number;
    roi: number;
  };
  logs: {
    user: string;
    action: string;
    time: string;
  }[];
}
