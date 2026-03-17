import React from 'react';
import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import { Layout } from './layout/Layout';
import { Dashboard } from './pages/Dashboard';
import { Audience } from './pages/Audience';
import { Rights } from './pages/Rights';
import { Rules } from './pages/Rules';
import { RuleEditor } from './pages/RuleEditor';
import { Touchpoints } from './pages/Touchpoints';
import { Campaigns } from './pages/Campaigns';
import { Settings } from './pages/Settings';

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route index element={<Dashboard />} />
          <Route path="audience" element={<Audience />} />
          <Route path="rights" element={<Rights />} />
          <Route path="rules" element={<Rules />} />
          <Route path="rules/new" element={<RuleEditor />} />
          <Route path="rules/edit/:id" element={<RuleEditor />} />
          <Route path="touchpoints" element={<Touchpoints />} />
          <Route path="campaigns" element={<Campaigns />} />
          <Route path="settings" element={<Settings />} />
          <Route path="*" element={<Navigate to="/" replace />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
