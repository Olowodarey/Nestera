import React from "react";
import Sidebar from "../components/dashboard/Sidebar";
import TopNav from "../components/dashboard/TopNav";
import "./dashboard.css";

export const metadata = {
  title: "Dashboard - Nestera",
};

export default function DashboardLayout({ children }: { children: React.ReactNode }) {
  return (
    <div className="dashboard-root">
      <Sidebar />

      <div className="dashboard-content">
        <TopNav />

        <div className="dashboard-children">{children}</div>
      </div>
    </div>
  );
}
