"use client";

import React from "react";
import "./ActivePoolList.css";

export interface PoolItem {
  id: number;
  title: string;
  subtitle: string;
  apy: number;
  staked: string;
  earnings: string;
}

const ActivePoolCard: React.FC<{ pool: PoolItem }> = ({ pool }) => {
  const initials = pool.title.split(" ")[0].slice(0, 2).toUpperCase();

  return (
    <div className="ap-card">
      <div className="ap-left">
        <div className="ap-token">
          <div className="ap-token-icon">{initials}</div>
        </div>

        <div className="ap-text">
          <div className="ap-title">{pool.title}</div>
          <div className="ap-subtitle">{pool.subtitle}</div>
        </div>
      </div>

      <div className="ap-columns">
        <div className="ap-column ap-apy">
          <div className="col-label">APY</div>
          <div className="col-value">{pool.apy}%</div>
        </div>

        <div className="ap-column ap-staked">
          <div className="col-label">STAKED</div>
          <div className="col-value">{pool.staked}</div>
        </div>

        <div className="ap-column ap-earnings">
          <div className="col-label">EARNINGS</div>
          <div className="col-value">{pool.earnings}</div>
        </div>
      </div>

      <div className="ap-more">⋯</div>
    </div>
  );
};

export default ActivePoolCard;
