import * as React from 'react'
import { cn } from '@/lib/utils'

type Props = React.ButtonHTMLAttributes<HTMLButtonElement> & {
  variant?: 'primary' | 'secondary' | 'ghost' | 'danger'
  size?: 'sm' | 'md'
}

export default function Button({
  className,
  variant = 'secondary',
  size = 'md',
  ...props
}: Props) {
  return (
    <button
      className={cn(
        'inline-flex items-center justify-center gap-2 rounded-xl border px-3 py-2 text-sm font-medium transition',
        'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-cyan-400/50 focus-visible:ring-offset-2 focus-visible:ring-offset-zinc-950',
        'disabled:pointer-events-none disabled:opacity-50',
        size === 'sm' ? 'h-9 px-3' : 'h-10 px-4',
        variant === 'primary' &&
          'border-cyan-400/40 bg-cyan-400/10 text-cyan-50 hover:bg-cyan-400/15 hover:border-cyan-300/60',
        variant === 'secondary' &&
          'border-zinc-800 bg-zinc-900/60 text-zinc-100 hover:bg-zinc-900/90 hover:border-zinc-700',
        variant === 'ghost' &&
          'border-transparent bg-transparent text-zinc-200 hover:bg-zinc-900/60 hover:border-zinc-800',
        variant === 'danger' &&
          'border-rose-500/40 bg-rose-500/10 text-rose-50 hover:bg-rose-500/15 hover:border-rose-400/60',
        className,
      )}
      {...props}
    />
  )
}

