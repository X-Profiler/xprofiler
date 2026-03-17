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
  value?: unknown
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

export function isConditionGroup(v: unknown): v is ConditionGroup {
  return (
    typeof v === 'object' &&
    v !== null &&
    (v as any).logic &&
    Array.isArray((v as any).children)
  )
}

export function validateConditionGroup(
  group: ConditionGroup,
  maxDepth: number,
): { ok: true } | { ok: false; path: string; error: string } {
  const visit = (
    node: ConditionNode,
    depth: number,
    path: string,
  ): { ok: true } | { ok: false; path: string; error: string } => {
    if (depth > maxDepth) {
      return { ok: false, path, error: '条件嵌套层级过深' }
    }

    if (isConditionGroup(node)) {
      if (node.logic !== 'AND' && node.logic !== 'OR') {
        return { ok: false, path, error: '条件组逻辑非法' }
      }
      if (!Array.isArray(node.children) || node.children.length === 0) {
        return { ok: false, path, error: '条件组不能为空' }
      }
      for (let i = 0; i < node.children.length; i++) {
        const r = visit(node.children[i] as any, depth + 1, `${path}.children[${i}]`)
        if (!r.ok) return r
      }
      return { ok: true }
    }

    const a = node as AtomicCondition
    if (!a.field || typeof a.field !== 'string') {
      return { ok: false, path, error: '字段不能为空' }
    }
    if (!a.op || typeof a.op !== 'string') {
      return { ok: false, path, error: '运算符不能为空' }
    }
    if (!a.valueType || typeof a.valueType !== 'string') {
      return { ok: false, path, error: '值类型不能为空' }
    }
    if (a.op !== 'exists' && typeof a.value === 'undefined') {
      return { ok: false, path, error: '值不能为空' }
    }
    return { ok: true }
  }

  return visit(group, 1, '$')
}

function compare(op: ConditionOp, left: unknown, right: unknown): boolean {
  if (op === 'exists') return typeof left !== 'undefined' && left !== null

  if (op === 'contains') {
    if (typeof left === 'string' && typeof right === 'string') {
      return left.includes(right)
    }
    if (Array.isArray(left)) {
      return left.includes(right as any)
    }
    return false
  }

  if (op === 'in' || op === 'not_in') {
    if (!Array.isArray(right)) return false
    const hit = right.includes(left as any)
    return op === 'in' ? hit : !hit
  }

  if (op === 'between') {
    if (!Array.isArray(right) || right.length !== 2) return false
    const [min, max] = right
    if (typeof left !== 'number' || typeof min !== 'number' || typeof max !== 'number')
      return false
    return left >= min && left <= max
  }

  if (typeof left === 'number' && typeof right === 'number') {
    if (op === 'gt') return left > right
    if (op === 'gte') return left >= right
    if (op === 'lt') return left < right
    if (op === 'lte') return left <= right
  }

  if (op === 'eq') return left === right
  if (op === 'neq') return left !== right
  return false
}

export function evaluateConditionGroup(
  group: ConditionGroup,
  context: Record<string, unknown>,
): boolean {
  const evalNode = (node: ConditionNode): boolean => {
    if (isConditionGroup(node)) {
      const results = node.children.map(evalNode)
      const v = node.logic === 'AND' ? results.every(Boolean) : results.some(Boolean)
      return node.not ? !v : v
    }
    const a = node as AtomicCondition
    const left = context[a.field]
    const v = compare(a.op, left, a.value)
    return v
  }
  return evalNode(group)
}

