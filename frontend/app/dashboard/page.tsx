import React from "react";
import NetWorthCard from "../components/dashboard/NetWorthCard";
import QuickActionsGrid from "../components/dashboard/QuickActionsGrid";
import ActivePoolList from "../components/dashboard/ActivePoolList";
import "./dashboard.css";

export default function DashboardPage() {
  return (
    <div className="dashboard-page">
      <div className="top-row">
        <div className="top-left">
          <NetWorthCard />
        </div>

        <div className="top-right">
          <QuickActionsGrid />
        </div>
      </div>

      <div className="second-row" style={{marginTop:20, display:'flex', gap:20}}>
        <div style={{flex:1, maxWidth:760}}>
          <ActivePoolList />
        </div>

        <div style={{width:320}}>
          {/* right-side placeholder for other dashboard widgets */}
          <div style={{height:120, borderRadius:12, background:'rgba(6,18,20,0.4)'}} />
        </div>
      </div>
    </div>
  );
}
