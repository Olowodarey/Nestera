"use client";

import React from "react";
import ActivePoolCard, { PoolItem } from "./ActivePoolCard";
import "./ActivePoolList.css";

const MOCK_POOLS: PoolItem[] = [
  {
    id: 1,
    title: "USDC Flexible Savings",
    subtitle: "Stellar Network",
    apy: 12.0,
    staked: "$10,000.00",
    earnings: "$450.21",
  },
  {
    id: 2,
    title: "ETH Locked Staking",
    subtitle: "Ethereum Network",
    apy: 5.2,
    staked: "4.50 ETH",
    earnings: "0.12 ETH",
  },
];

const ActivePoolList: React.FC = () => {
  return (
    <section className="active-pools-panel">
      <div className="panel-header">
        <h4>Active Savings & Staking</h4>
        <a className="view-all" href="#">View all</a>
      </div>

      <div className="panel-body">
        {MOCK_POOLS.map((p) => (
          <ActivePoolCard key={p.id} pool={p} />
        ))}
      </div>
    </section>
  );
};

export default ActivePoolList;
