import { NavLink } from 'react-router-dom'
import { cn } from '@/lib/utils'

const items = [
  { suffix: '', label: '总览' },
  { suffix: '/audience', label: '人群' },
  { suffix: '/benefits', label: '权益' },
  { suffix: '/rules', label: '规则' },
  { suffix: '/orchestration', label: '触达' },
  { suffix: '/logs', label: '历史' },
]

export default function ActivityNav({ id }: { id: string }) {
  return (
    <div className="flex flex-wrap items-center gap-2">
      {items.map((it) => (
        <NavLink
          key={it.suffix || 'root'}
          to={`/activities/${id}${it.suffix}`}
          end={it.suffix === ''}
          className={({ isActive }) =>
            cn(
              'rounded-xl border px-3 py-2 text-sm transition',
              isActive
                ? 'border-cyan-400/30 bg-cyan-400/10 text-cyan-50'
                : 'border-zinc-900 bg-zinc-950/10 text-zinc-300 hover:border-zinc-800 hover:bg-zinc-950/25',
            )
          }
        >
          {it.label}
        </NavLink>
      ))}
    </div>
  )
}

