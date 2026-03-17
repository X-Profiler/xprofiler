import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom'
import Protected from '@/components/Protected'
import AppShell from '@/layout/AppShell'
import Login from '@/pages/Login'
import Activities from '@/pages/Activities'
import ActivityOverview from '@/pages/ActivityOverview'
import Audience from '@/pages/Audience'
import Benefits from '@/pages/Benefits'
import Rules from '@/pages/Rules'
import Orchestration from '@/pages/Orchestration'
import Logs from '@/pages/Logs'
import Analytics from '@/pages/Analytics'

export default function App() {
  return (
    <Router>
      <Routes>
        <Route path="/login" element={<Login />} />
        <Route element={<Protected />}>
          <Route element={<AppShell />}>
            <Route path="/" element={<Navigate to="/activities" replace />} />
            <Route path="/activities" element={<Activities />} />
            <Route path="/activities/:id" element={<ActivityOverview />} />
            <Route path="/activities/:id/audience" element={<Audience />} />
            <Route path="/activities/:id/benefits" element={<Benefits />} />
            <Route path="/activities/:id/rules" element={<Rules />} />
            <Route path="/activities/:id/orchestration" element={<Orchestration />} />
            <Route path="/activities/:id/logs" element={<Logs />} />
            <Route path="/analytics" element={<Analytics />} />
          </Route>
        </Route>
        <Route path="*" element={<Navigate to="/" replace />} />
      </Routes>
    </Router>
  )
}
