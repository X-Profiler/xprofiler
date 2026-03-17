export type ApiResult<T> =
  | { success: true; data: T }
  | { success: false; error: string }

export async function apiFetch<T>(
  input: RequestInfo | URL,
  init?: RequestInit & { actor?: string; token?: string },
): Promise<T> {
  const headers = new Headers(init?.headers)
  headers.set('content-type', 'application/json')
  if (init?.actor) headers.set('x-actor', init.actor)
  if (init?.token) headers.set('authorization', `Bearer ${init.token}`)

  const res = await fetch(input, {
    ...init,
    headers,
  })

  const json = (await res.json().catch(() => null)) as ApiResult<T> | null
  if (!json) throw new Error('响应解析失败')
  if (json.success === false) throw new Error(json.error || '请求失败')
  return json.data
}
