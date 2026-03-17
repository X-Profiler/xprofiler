import { Outlet, useLocation, useNavigate } from 'react-router-dom'
import Sidebar from '@/layout/Sidebar'
import Button from '@/components/ui/Button'
import { useTheme } from '@/hooks/useTheme'
import { useAuthStore } from '@/stores/auth'
import { LogOut, Moon, Sun } from 'lucide-react'

export default function AppShell() {
  const { pathname } = useLocation()
  const navigate = useNavigate()
  const { theme, toggleTheme, isDark } = useTheme()
  const { username, role, logout } = useAuthStore()

  return (
    <div className="h-screen w-screen overflow-hidden bg-[radial-gradient(1200px_800px_at_20%_-10%,rgba(34,211,238,0.12),transparent_50%),radial-gradient(900px_600px_at_80%_0%,rgba(251,191,36,0.08),transparent_50%),linear-gradient(to_bottom,rgba(9,9,11,1),rgba(3,7,18,1))] text-zinc-100">
      <div className="flex h-full">
        <Sidebar />
        <div className="flex min-w-0 flex-1 flex-col">
          <header className="flex items-center justify-between border-b border-zinc-900 bg-zinc-950/30 px-6 py-4 backdrop-blur">
            <div className="min-w-0">
              <div className="truncate text-xs font-medium tracking-[0.18em] text-zinc-400">
                {pathname}
              </div>
              <div className="mt-1 truncate text-sm font-semibold text-zinc-100">
                {role.toUpperCase()}
                {username ? ` · ${username}` : ''}
              </div>
            </div>
            <div className="flex items-center gap-2">
              <Button variant="ghost" onClick={toggleTheme}>
                {isDark ? <Sun className="h-4 w-4" /> : <Moon className="h-4 w-4" />}
                {theme === 'dark' ? '浅色' : '深色'}
              </Button>
              <Button
                variant="ghost"
                onClick={() => {
                  logout()
                  navigate('/login')
                }}
              >
                <LogOut className="h-4 w-4" />
                退出
              </Button>
            </div>
          </header>
          <main className="min-h-0 flex-1 overflow-auto px-6 py-6">
            <Outlet />
          </main>
        </div>
      </div>
    </div>
  )
}

