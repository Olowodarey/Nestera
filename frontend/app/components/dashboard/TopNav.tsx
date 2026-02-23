"use client";

import React from "react";
import "./TopNav.css";
import { Search, Bell, HelpCircle } from "lucide-react";

const TopNav: React.FC = () => {
  return (
    <header className="dashboard-topnav">
      <div className="topnav-left">
        <h2 className="greeting">Welcome back, Alex</h2>
      </div>

      <div className="topnav-actions">
        <button className="icon-btn" aria-label="Search">
          <Search size={16} />
        </button>
        <button className="icon-btn" aria-label="Notifications">
          <Bell size={16} />
        </button>
        <button className="icon-btn" aria-label="Help">
          <HelpCircle size={16} />
        </button>

        <div className="avatar">A</div>
      </div>
    </header>
  );
};

export default TopNav;
