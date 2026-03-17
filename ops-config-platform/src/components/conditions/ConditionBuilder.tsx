import * as React from 'react'
import { cn } from '@/lib/utils'
import type { AtomicCondition, ConditionGroup, ConditionNode, ConditionOp } from '@/types'
import Button from '@/components/ui/Button'
import Input from '@/components/ui/Input'
import Select from '@/components/ui/Select'
import Badge from '@/components/ui/Badge'
import { Plus, Trash2, CornerDownRight } from 'lucide-react'

const OPS: Array<{ value: ConditionOp; label: string }> = [
  { value: 'eq', label: '=' },
  { value: 'neq', label: '≠' },
  { value: 'in', label: 'IN' },
  { value: 'not_in', label: 'NOT IN' },
  { value: 'gt', label: '>' },
  { value: 'gte', label: '≥' },
  { value: 'lt', label: '<' },
  { value: 'lte', label: '≤' },
  { value: 'between', label: 'BETWEEN' },
  { value: 'contains', label: 'CONTAINS' },
  { value: 'exists', label: 'EXISTS' },
]

type FieldSpec = {
  key: string
  label: string
  valueType: AtomicCondition['valueType']
  placeholder?: string
}

function isGroup(n: ConditionNode): n is ConditionGroup {
  return typeof n === 'object' && !!n && Array.isArray((n as any).children)
}

function newAtomic(field: FieldSpec): AtomicCondition {
  return {
    field: field.key,
    op: 'eq',
    valueType: field.valueType,
    value: field.valueType.endsWith('[]') ? [] : '',
  }
}

function safeStringify(v: any): string {
  if (typeof v === 'string') return v
  try {
    return JSON.stringify(v)
  } catch {
    return String(v)
  }
}

export default function ConditionBuilder({
  value,
  onChange,
  fields,
  maxDepth = 2,
  className,
}: {
  value: ConditionGroup
  onChange: (next: ConditionGroup) => void
  fields: FieldSpec[]
  maxDepth?: number
  className?: string
}) {
  const setAt = React.useCallback(
    (path: number[], next: ConditionNode) => {
      const clone = structuredClone(value) as ConditionGroup
      let cur: any = clone
      for (let i = 0; i < path.length - 1; i++) {
        cur = cur.children[path[i]]
      }
      cur.children[path[path.length - 1]] = next
      onChange(clone)
    },
    [onChange, value],
  )

  const removeAt = React.useCallback(
    (path: number[]) => {
      const clone = structuredClone(value) as ConditionGroup
      let cur: any = clone
      for (let i = 0; i < path.length - 1; i++) {
        cur = cur.children[path[i]]
      }
      cur.children.splice(path[path.length - 1], 1)
      onChange(clone)
    },
    [onChange, value],
  )

  const addAtomicTo = React.useCallback(
    (path: number[]) => {
      const clone = structuredClone(value) as ConditionGroup
      let cur: any = clone
      for (let i = 0; i < path.length; i++) cur = cur.children[path[i]]
      const first = fields[0]
      cur.children.push(newAtomic(first))
      onChange(clone)
    },
    [fields, onChange, value],
  )

  const addGroupTo = React.useCallback(
    (path: number[]) => {
      const clone = structuredClone(value) as ConditionGroup
      let cur: any = clone
      for (let i = 0; i < path.length; i++) cur = cur.children[path[i]]
      cur.children.push({
        logic: 'AND',
        children: [newAtomic(fields[0])],
      })
      onChange(clone)
    },
    [fields, onChange, value],
  )

  const renderNode = (node: ConditionNode, path: number[], depth: number) => {
    if (isGroup(node)) {
      return (
        <div
          key={path.join('.')}
          className={cn(
            'rounded-2xl border border-zinc-800/80 bg-zinc-950/30 p-3',
            depth > 1 && 'ml-3',
          )}
        >
          <div className="flex flex-wrap items-center justify-between gap-2">
            <div className="flex items-center gap-2">
              {depth > 1 ? (
                <CornerDownRight className="h-4 w-4 text-zinc-500" />
              ) : null}
              <Badge tone="neutral">条件组</Badge>
              <Select
                className="h-9 w-[140px]"
                value={node.logic}
                onChange={(e) =>
                  setAt(path, { ...node, logic: e.target.value as any })
                }
              >
                <option value="AND">AND（全部满足）</option>
                <option value="OR">OR（任意满足）</option>
              </Select>
              <label className="flex items-center gap-2 text-xs text-zinc-300">
                <input
                  type="checkbox"
                  checked={!!node.not}
                  onChange={(e) => setAt(path, { ...node, not: e.target.checked })}
                />
                NOT
              </label>
            </div>
            <div className="flex items-center gap-2">
              <Button
                size="sm"
                variant="ghost"
                onClick={() => addAtomicTo(path)}
              >
                <Plus className="h-4 w-4" />
                条件
              </Button>
              <Button
                size="sm"
                variant="ghost"
                disabled={depth >= maxDepth}
                onClick={() => addGroupTo(path)}
              >
                <Plus className="h-4 w-4" />
                子组
              </Button>
              {depth > 1 ? (
                <Button
                  size="sm"
                  variant="ghost"
                  onClick={() => removeAt(path)}
                >
                  <Trash2 className="h-4 w-4" />
                </Button>
              ) : null}
            </div>
          </div>
          <div className="mt-3 space-y-2">
            {node.children.map((c, i) => renderNode(c, [...path, i], depth + 1))}
          </div>
        </div>
      )
    }

    const a = node as AtomicCondition
    const fieldSpec = fields.find((f) => f.key === a.field) ?? fields[0]

    return (
      <div
        key={path.join('.')}
        className={cn(
          'grid grid-cols-12 items-center gap-2 rounded-2xl border border-zinc-800/60 bg-zinc-950/20 p-3',
          depth > 1 && 'ml-3',
        )}
      >
        <div className="col-span-4">
          <Select
            className="h-9"
            value={a.field}
            onChange={(e) => {
              const f = fields.find((x) => x.key === e.target.value) ?? fields[0]
              setAt(path, { ...a, field: f.key, valueType: f.valueType, value: '' })
            }}
          >
            {fields.map((f) => (
              <option key={f.key} value={f.key}>
                {f.label}
              </option>
            ))}
          </Select>
        </div>
        <div className="col-span-3">
          <Select
            className="h-9"
            value={a.op}
            onChange={(e) => setAt(path, { ...a, op: e.target.value as any })}
          >
            {OPS.map((o) => (
              <option key={o.value} value={o.value}>
                {o.label}
              </option>
            ))}
          </Select>
        </div>
        <div className="col-span-4">
          {a.op === 'exists' ? (
            <Input className="h-9" value="存在即可" disabled />
          ) : (
            <Input
              className="h-9"
              placeholder={fieldSpec.placeholder ?? '值'}
              value={typeof a.value === 'string' ? a.value : safeStringify(a.value)}
              onChange={(e) => setAt(path, { ...a, value: e.target.value })}
            />
          )}
        </div>
        <div className="col-span-1 flex justify-end">
          <Button size="sm" variant="ghost" onClick={() => removeAt(path)}>
            <Trash2 className="h-4 w-4" />
          </Button>
        </div>
      </div>
    )
  }

  return (
    <div className={cn('space-y-3', className)}>{renderNode(value, [], 0)}</div>
  )
}

