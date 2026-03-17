import { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { apiFetch } from '@/api/http'
import Button from '@/components/ui/Button'
import Input from '@/components/ui/Input'
import Select from '@/components/ui/Select'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card'
import { useAuthStore, type UserRole } from '@/stores/auth'
import { Shield, Sparkles } from 'lucide-react'

export default function Login() {
  const navigate = useNavigate()
  const setAuth = useAuthStore((s) => s.setAuth)
  const [username, setUsername] = useState('demo')
  const [role, setRole] = useState<UserRole>('operator')
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  return (
    <div className="flex min-h-screen items-center justify-center bg-[radial-gradient(1000px_700px_at_20%_-10%,rgba(34,211,238,0.18),transparent_55%),radial-gradient(900px_600px_at_80%_0%,rgba(251,191,36,0.12),transparent_55%),linear-gradient(to_bottom,rgba(9,9,11,1),rgba(3,7,18,1))] px-6 text-zinc-100">
      <div className="w-full max-w-[460px]">
        <div className="mb-6">
          <div className="flex items-center gap-3">
            <div className="flex h-11 w-11 items-center justify-center rounded-2xl border border-cyan-400/25 bg-cyan-400/10">
              <Shield className="h-5 w-5 text-cyan-200" />
            </div>
            <div>
              <div className="text-xs font-semibold tracking-[0.2em] text-zinc-400">
                OPS CONFIG PLATFORM
              </div>
              <div className="mt-1 text-lg font-semibold">运营工具配置台</div>
            </div>
          </div>
        </div>

        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Sparkles className="h-4 w-4 text-amber-200" />
              登录（演示模式）
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="space-y-2">
              <div className="text-xs font-medium text-zinc-300">用户名</div>
              <Input value={username} onChange={(e) => setUsername(e.target.value)} />
            </div>
            <div className="space-y-2">
              <div className="text-xs font-medium text-zinc-300">角色</div>
              <Select value={role} onChange={(e) => setRole(e.target.value as UserRole)}>
                <option value="operator">运营</option>
                <option value="approver">审核</option>
                <option value="admin">管理员</option>
                <option value="analyst">分析（只读）</option>
              </Select>
            </div>

            {error ? <div className="text-sm text-rose-200">{error}</div> : null}

            <Button
              variant="primary"
              className="w-full"
              disabled={loading}
              onClick={async () => {
                setError(null)
                setLoading(true)
                try {
                  const data = await apiFetch<{
                    token: string
                    user: { username: string; role: UserRole }
                  }>('/api/auth/login', {
                    method: 'POST',
                    body: JSON.stringify({ username, role }),
                  })
                  setAuth({
                    token: data.token,
                    username: data.user.username,
                    role: data.user.role,
                  })
                  navigate('/activities')
                } catch (e: any) {
                  setError(e?.message ?? '登录失败')
                } finally {
                  setLoading(false)
                }
              }}
            >
              进入工作台
            </Button>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}

