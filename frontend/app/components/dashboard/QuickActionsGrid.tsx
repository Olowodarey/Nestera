"use client";

import React from "react";
import "./QuickActionsGrid.css";
import { ArrowDownCircle, ArrowUpCircle, Repeat, Link } from "lucide-react";

const actions = [
  { label: "Deposit", icon: ArrowDownCircle },
  { label: "Withdraw", icon: ArrowUpCircle },
  { label: "Swap", icon: Repeat },
  { label: "Bridge", icon: Link },
];

const QuickActionsGrid: React.FC = () => {
  return (
    <div className="quick-actions">
      {actions.map((a) => {
        const Icon = a.icon as any;
        return (
          <button key={a.label} className="qa-btn">
            <div className="qa-icon"><Icon size={20} /></div>
            <div className="qa-label">{a.label}</div>
          </button>
        );
      })}
    </div>
  );
};

export default QuickActionsGrid;
