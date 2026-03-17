import { create } from 'zustand';
import { Campaign, Audience, Right, Rule, Touchpoint } from '../types';

interface AppState {
  campaigns: Campaign[];
  audiences: Audience[];
  rights: Right[];
  rules: Rule[];
  touchpoints: Touchpoint[];
  
  addCampaign: (campaign: Campaign) => void;
  updateCampaign: (id: string, campaign: Partial<Campaign>) => void;
  deleteCampaign: (id: string) => void;
  
  addAudience: (audience: Audience) => void;
  addRight: (right: Right) => void;
  addRule: (rule: Rule) => void;
  addTouchpoint: (touchpoint: Touchpoint) => void;
}

export const useStore = create<AppState>((set) => ({
  campaigns: [
    {
      id: '1',
      name: '双十一预热活动',
      status: 'online',
      audienceId: '1',
      ruleId: '1',
      touchpointId: '1',
      rightsId: '1',
      startTime: '2023-11-01',
      endTime: '2023-11-11',
      metrics: {
        participants: 12500,
        conversion: 3.5,
        roi: 120
      },
      logs: []
    },
    {
      id: '2',
      name: '新用户注册礼包',
      status: 'testing',
      audienceId: '2',
      ruleId: '2',
      touchpointId: '2',
      rightsId: '2',
      startTime: '2023-12-01',
      endTime: '2023-12-31',
      metrics: {
        participants: 50,
        conversion: 0,
        roi: 0
      },
      logs: []
    }
  ],
  audiences: [
    {
      id: '1',
      name: '高价值活跃用户',
      description: '过去30天购买超过3次且客单价>500元',
      filters: {
        tags: ['High Value', 'Active'],
        behaviors: ['Purchase > 3'],
      },
      count: 5000,
      createdAt: '2023-10-25'
    },
    {
      id: '2',
      name: '潜在流失用户',
      description: '过去60天未登录',
      filters: {
        tags: ['Inactive'],
        behaviors: ['Login = 0'],
      },
      count: 12000,
      createdAt: '2023-10-26'
    }
  ],
  rights: [
    {
      id: '1',
      name: '满100减20优惠券',
      type: 'coupon',
      config: {
        amount: 20,
        validity: 7
      },
      stock: 10000,
      createdAt: '2023-10-20'
    },
    {
      id: '2',
      name: '新人积分礼包',
      type: 'points',
      config: {
        amount: 500
      },
      stock: 99999,
      createdAt: '2023-10-21'
    }
  ],
  rules: [
    {
      id: '1',
      name: '双十一满减规则',
      nodes: [
        { id: '1', type: 'input', data: { label: '开始' }, position: { x: 250, y: 5 } },
        { id: '2', data: { label: '消费金额 > 100' }, position: { x: 100, y: 100 }, type: 'default' },
        { id: '3', data: { label: '发放优惠券' }, position: { x: 400, y: 100 }, type: 'output' },
      ],
      edges: [
        { id: 'e1-2', source: '1', target: '2' },
        { id: 'e2-3', source: '2', target: '3' }
      ],
      createdAt: '2023-10-28'
    }
  ],
  touchpoints: [
    {
      id: '1',
      name: '双十一短信通知',
      channel: 'sms',
      content: '双十一预热开启！全场满100减20，速来抢购！',
      schedule: '2023-11-01 10:00',
      status: 'active'
    },
    {
      id: '2',
      name: '新人欢迎站内信',
      channel: 'inapp',
      content: '欢迎加入我们！送您500积分，点击领取。',
      schedule: 'Trigger: Registration',
      status: 'active'
    }
  ],

  addCampaign: (campaign) => set((state) => ({ campaigns: [...state.campaigns, campaign] })),
  updateCampaign: (id, campaign) => set((state) => ({
    campaigns: state.campaigns.map((c) => (c.id === id ? { ...c, ...campaign } : c))
  })),
  deleteCampaign: (id) => set((state) => ({ campaigns: state.campaigns.filter((c) => c.id !== id) })),
  
  addAudience: (audience) => set((state) => ({ audiences: [...state.audiences, audience] })),
  addRight: (right) => set((state) => ({ rights: [...state.rights, right] })),
  addRule: (rule) => set((state) => ({ rules: [...state.rules, rule] })),
  addTouchpoint: (touchpoint) => set((state) => ({ touchpoints: [...state.touchpoints, touchpoint] })),
}));
