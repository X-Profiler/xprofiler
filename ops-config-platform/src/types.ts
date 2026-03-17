export type ActivityStatus = 'draft' | 'testing' | 'online' | 'offline' | 'archived'

export type ConditionOp =
  | 'eq'
  | 'neq'
  | 'in'
  | 'not_in'
  | 'gt'
  | 'gte'
  | 'lt'
  | 'lte'
  | 'between'
  | 'contains'
  | 'exists'

export type AtomicCondition = {
  field: string
  op: ConditionOp
  value?: any
  valueType:
    | 'string'
    | 'number'
    | 'boolean'
    | 'date'
    | 'string[]'
    | 'number[]'
}

export type ConditionNode = AtomicCondition | ConditionGroup

export type ConditionGroup = {
  logic: 'AND' | 'OR'
  not?: boolean
  children: ConditionNode[]
}

export type AudienceDefinition = {
  labels?: ConditionGroup
  behaviors?: ConditionGroup
  whitelist?: { enabled: boolean; source: 'manual' | 'excel'; userIds: string[] }
}

export type BenefitConfig = {
  couponPacks?: Array<{ id: string; name: string; coupons: Array<{ code: string; count: number }> }>
  pointsRules?: Array<{ id: string; name: string; earnPerOrder?: number; costPerPoint?: number; dailyCap?: number }>
  others?: Array<{ id: string; name: string; kind: 'physical' | 'virtual'; stock?: number }>
}

export type RuleSet = {
  threshold?: ConditionGroup
  stacking?: { mode: 'exclusive' | 'allow'; priority?: number }
  limit?: { perUser: number; period: 'day' | 'week' | 'month' | 'lifecycle' }
}

export type TouchChannel = 'inbox' | 'sms' | 'im'

export type TouchStep = {
  id: string
  channel: TouchChannel
  templateId: string
  schedule: { type: 'immediate' | 'time_window' | 'cron'; value?: string }
  frequency: { maxPerUser: number; period: 'day' | 'week' | 'month' }
  ab?: { enabled: boolean; buckets: Array<{ name: string; ratio: number; templateId: string }> }
}

export type OrchestrationPlan = {
  steps: TouchStep[]
}

export type Activity = {
  id: string
  name: string
  description: string
  status: ActivityStatus
  created_at: string
  updated_at: string
}

export type ActivityFull = Activity & {
  config: {
    audience: AudienceDefinition
    benefits: BenefitConfig
    rules: RuleSet
    orchestration: OrchestrationPlan
  }
}

