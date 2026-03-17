import React from 'react';
import { Outlet, NavLink } from 'react-router-dom';
import { 
  LayoutDashboard, 
  Users, 
  Gift, 
  Workflow, 
  Send, 
  Activity, 
  Settings,
  Bell,
  User
} from 'lucide-react';
import clsx from 'clsx';

const SidebarItem = ({ to, icon: Icon, label }: { to: string; icon: any; label: string }) => (
  <NavLink
    to={to}
    className={({ isActive }) =>
      clsx(
        'flex items-center gap-3 px-4 py-3 rounded-lg transition-colors',
        isActive
          ? 'bg-blue-50 text-blue-600 font-medium'
          : 'text-gray-600 hover:bg-gray-50 hover:text-gray-900'
      )
    }
  >
    <Icon size={20} />
    <span>{label}</span>
  </NavLink>
);

export const Layout = () => {
  return (
    <div className="flex h-screen bg-gray-50 font-sans">
      {/* Sidebar */}
      <aside className="w-64 bg-white border-r border-gray-200 flex flex-col">
        <div className="p-6 flex items-center gap-2 border-b border-gray-100">
          <div className="w-8 h-8 bg-blue-600 rounded-lg flex items-center justify-center text-white font-bold text-xl">
            O
          </div>
          <span className="text-xl font-bold text-gray-800">OpsPlatform</span>
        </div>

        <nav className="flex-1 p-4 space-y-1 overflow-y-auto">
          <SidebarItem to="/" icon={LayoutDashboard} label="工作台" />
          <SidebarItem to="/audience" icon={Users} label="人群圈选" />
          <SidebarItem to="/rights" icon={Gift} label="权益中心" />
          <SidebarItem to="/rules" icon={Workflow} label="规则引擎" />
          <SidebarItem to="/touchpoints" icon={Send} label="触达编排" />
          <SidebarItem to="/campaigns" icon={Activity} label="活动管理" />
        </nav>

        <div className="p-4 border-t border-gray-100">
          <SidebarItem to="/settings" icon={Settings} label="系统设置" />
        </div>
      </aside>

      {/* Main Content */}
      <div className="flex-1 flex flex-col overflow-hidden">
        {/* Header */}
        <header className="h-16 bg-white border-b border-gray-200 flex items-center justify-between px-6 shadow-sm z-10">
          <h2 className="text-lg font-semibold text-gray-800">运营配置中心</h2>
          
          <div className="flex items-center gap-4">
            <button className="p-2 text-gray-500 hover:bg-gray-100 rounded-full relative">
              <Bell size={20} />
              <span className="absolute top-1.5 right-1.5 w-2 h-2 bg-red-500 rounded-full"></span>
            </button>
            <div className="flex items-center gap-3 pl-4 border-l border-gray-200">
              <div className="text-right hidden sm:block">
                <p className="text-sm font-medium text-gray-900">Admin User</p>
                <p className="text-xs text-gray-500">超级管理员</p>
              </div>
              <div className="w-9 h-9 bg-gray-200 rounded-full flex items-center justify-center text-gray-600">
                <User size={20} />
              </div>
            </div>
          </div>
        </header>

        {/* Page Content */}
        <main className="flex-1 overflow-auto p-6">
          <Outlet />
        </main>
      </div>
    </div>
  );
};
