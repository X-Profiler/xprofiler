import * as React from 'react'
import { cn } from '@/lib/utils'

type Props = React.InputHTMLAttributes<HTMLInputElement>

export default function Input({ className, ...props }: Props) {
  return (
    <input
      className={cn(
        'h-10 w-full rounded-xl border border-zinc-800 bg-zinc-950/40 px-3 text-sm text-zinc-100 placeholder:text-zinc-500',
        'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-cyan-400/40',
        className,
      )}
      {...props}
    />
  )
}

