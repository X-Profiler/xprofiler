import { cn } from '@/lib/utils'

export default function Badge({
  children,
  tone = 'neutral',
  className,
}: {
  children: React.ReactNode
  tone?: 'neutral' | 'cyan' | 'amber' | 'rose' | 'green'
  className?: string
}) {
  return (
    <span
      className={cn(
        'inline-flex items-center rounded-full border px-2.5 py-1 text-xs font-medium',
        tone === 'neutral' && 'border-zinc-800 bg-zinc-950/40 text-zinc-200',
        tone === 'cyan' && 'border-cyan-400/30 bg-cyan-400/10 text-cyan-100',
        tone === 'amber' && 'border-amber-400/30 bg-amber-400/10 text-amber-100',
        tone === 'rose' && 'border-rose-400/30 bg-rose-400/10 text-rose-100',
        tone === 'green' && 'border-emerald-400/30 bg-emerald-400/10 text-emerald-100',
        className,
      )}
    >
      {children}
    </span>
  )
}

