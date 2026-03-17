import { NavLink } from 'react-router-dom'
import { cn } from '@/lib/utils'
import {
  Activity,
  BarChart3,
  Layers3,
  Send,
  ShieldCheck,
  Users,
} from 'lucide-react'

const items = [
  { to: '/activities', label: '活动', icon: Activity },
  { to: '/analytics', label: '看板', icon: BarChart3 },
]

export default function Sidebar() {
  return (
    <aside className="flex h-full w-[260px] flex-col border-r border-zinc-900 bg-zinc-950/40">
      <div className="px-5 py-5">
        <div className="text-xs font-semibold tracking-[0.2em] text-zinc-400">
          OPS CONFIG
        </div>
        <div className="mt-2 text-lg font-semibold text-zinc-100">
          运营工具配置台
        </div>
      </div>

      <nav className="px-3 pb-4">
        <div className="mb-2 px-2 text-[11px] font-medium uppercase tracking-[0.18em] text-zinc-500">
          工作区
        </div>
        <div className="space-y-1">
          {items.map((it) => (
            <NavLink
              key={it.to}
              to={it.to}
              className={({ isActive }) =>
                cn(
                  'flex items-center gap-2 rounded-xl border px-3 py-2 text-sm transition',
                  isActive
                    ? 'border-cyan-400/30 bg-cyan-400/10 text-cyan-50'
                    : 'border-transparent text-zinc-200 hover:border-zinc-800 hover:bg-zinc-950/60',
                )
              }
            >
              <it.icon className="h-4 w-4 opacity-90" />
              {it.label}
            </NavLink>
          ))}
        </div>

        <div className="mt-6 mb-2 px-2 text-[11px] font-medium uppercase tracking-[0.18em] text-zinc-500">
          说明
        </div>
        <div className="space-y-2 px-2 text-xs leading-5 text-zinc-400">
          <div className="flex items-center gap-2">
            <Users className="h-4 w-4" />
            人群/规则/权益/触达配置均在活动内完成
          </div>
          <div className="flex items-center gap-2">
            <Layers3 className="h-4 w-4" />
            所有写操作自动生成快照，支持回滚
          </div>
          <div className="flex items-center gap-2">
            <Send className="h-4 w-4" />
            触达先做模拟发送，后续可接真实渠道
          </div>
          <div className="flex items-center gap-2">
            <ShieldCheck className="h-4 w-4" />
            上线态默认只读，避免“边跑边改”
          </div>
        </div>
      </nav>
    </aside>
  )
}

