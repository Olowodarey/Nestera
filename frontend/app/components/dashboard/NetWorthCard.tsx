"use client";

import React from "react";
import "./NetWorthCard.css";

const NetWorthCard: React.FC = () => {
  return (
    <div className="networth-card">
      <div className="networth-body">
        <div className="networth-top">
          <div className="networth-label">TOTAL NET WORTH</div>
          <div className="networth-change-pill">+ $1,240.50 <span className="muted">(+5.4%)</span></div>
        </div>

        <div className="networth-amount">$24,593.82</div>

        <div className="networth-footer muted">vs last month</div>
      </div>

      <svg className="networth-wave" viewBox="0 0 600 200" preserveAspectRatio="none" aria-hidden>
        <defs>
          <linearGradient id="g1" x1="0" x2="1">
            <stop offset="0%" stopColor="#063f3f" stopOpacity="0.25" />
            <stop offset="100%" stopColor="#083b3b" stopOpacity="0.08" />
          </linearGradient>
        </defs>
        <path d="M0,80 C150,200 350,0 600,80 L600,200 L0,200 Z" fill="url(#g1)" />
      </svg>
    </div>
  );
};

export default NetWorthCard;
