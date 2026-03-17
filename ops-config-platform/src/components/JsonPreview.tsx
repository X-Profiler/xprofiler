import { cn } from '@/lib/utils'

function safeJson(v: unknown): string {
  try {
    return JSON.stringify(v, null, 2)
  } catch {
    return String(v)
  }
}

export default function JsonPreview({
  value,
  className,
}: {
  value: unknown
  className?: string
}) {
  return (
    <pre
      className={cn(
        'max-h-[360px] overflow-auto rounded-2xl border border-zinc-800/70 bg-zinc-950/50 p-4 text-xs text-zinc-200',
        className,
      )}
    >
      {safeJson(value)}
    </pre>
  )
}

