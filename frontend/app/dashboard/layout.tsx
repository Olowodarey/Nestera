import React from "react";
import Sidebar from "../components/dashboard/Sidebar";
import TopNav from "../components/dashboard/TopNav";

export const metadata = {
  title: "Dashboard - Nestera",
};

export default function DashboardLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div className="block bg-[#061218] min-h-screen">
      <Sidebar />

      {/* 260px left margin to clear the fixed sidebar on all screens */}
      <div style={{ marginLeft: 180 }} className="min-h-screen px-6 py-5">
        <TopNav />
        <div className="mt-2">{children}</div>
      </div>
    </div>
  );
}
