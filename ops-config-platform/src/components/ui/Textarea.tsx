import * as React from 'react'
import { cn } from '@/lib/utils'

type Props = React.TextareaHTMLAttributes<HTMLTextAreaElement>

export default function Textarea({ className, ...props }: Props) {
  return (
    <textarea
      className={cn(
        'min-h-[120px] w-full resize-y rounded-xl border border-zinc-800 bg-zinc-950/40 px-3 py-2 text-sm text-zinc-100 placeholder:text-zinc-500',
        'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-cyan-400/40',
        className,
      )}
      {...props}
    />
  )
}

