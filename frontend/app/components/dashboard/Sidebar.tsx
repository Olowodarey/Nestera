"use client";

import React, { useState } from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { Menu, X, Home, PieChart, ShieldCheck, Settings, Landmark, TrendingUp } from "lucide-react";
import "./Sidebar.css";

const navLinks = [
  { label: "Dashboard", href: "/dashboard" , icon: Home},
  { label: "Savings Pools", href: "/dashboard/savings-pools", icon: Landmark},
  { label: "Staking", href: "/dashboard/staking", icon: TrendingUp},
  { label: "Analytics", href: "/dashboard/analytics", icon: PieChart},
  { label: "Governance", href: "/dashboard/governance", icon: ShieldCheck},
  { label: "Settings", href: "/dashboard/settings", icon: Settings},
];

const Sidebar: React.FC = () => {
  const pathname = usePathname();
  const [open, setOpen] = useState(false);

  const isActive = (href: string) => {
    if (!pathname) return false;
    return pathname === href || pathname.startsWith(href + "/");
  };

  return (
    <aside className={`dashboard-sidebar ${open ? "open" : ""}`} aria-label="Sidebar">
      <div className="sidebar-top">
        <Link href="/dashboard" className="sidebar-logo">
          <div className="logo-icon">N</div>
          <span className="logo-text">Nestera</span>
        </Link>

        <button className="sidebar-hamburger" onClick={() => setOpen(!open)} aria-label="Toggle menu">
          {open ? <X size={18} /> : <Menu size={18} />}
        </button>
      </div>

      <nav className="sidebar-nav">
        {navLinks.map((l) => {
          const Icon = l.icon as any;
          return (
            <Link
              key={l.href}
              href={l.href}
              className={`sidebar-link ${isActive(l.href) ? "active" : ""}`}
              onClick={() => setOpen(false)}
            >
              <span className="sidebar-link-icon">
                <Icon size={16} />
              </span>
              <span className="sidebar-link-text">{l.label}</span>
            </Link>
          );
        })}
      </nav>

      <div className="sidebar-footer">
        <div className="connected">
          <div className="dot" />
          <div className="connected-info">
            <div className="connected-label">CONNECTED</div>
            <div className="connected-wallet">0x4a...8f</div>
          </div>
        </div>
      </div>
    </aside>
  );
};

export default Sidebar;
