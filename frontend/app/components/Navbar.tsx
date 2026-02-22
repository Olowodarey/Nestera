"use client";

import React, { useState } from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { Wallet } from "lucide-react";
import "./Navbar.css";

interface NavLink {
  label: string;
  href: string;
}

const Navbar: React.FC = () => {
  const pathname = usePathname();
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);

  const navLinks: NavLink[] = [
    { label: "Features", href: "/features" },
    { label: "Savings", href: "/savings" },
    { label: "Community", href: "/community" },
    { label: "Docs", href: "/docs" },
  ];

  const isActiveLink = (href: string): boolean => {
    return pathname === href || pathname?.startsWith(href + "/");
  };

  return (
    <nav className="navbar">
      <div className="navbar__container">
        <div className="navbar__content">
          <div className="navbar__logo">
            <Link href="/" className="navbar__logo-link">
              <div className="navbar__logo-icon">
                <Wallet size={18} color="#061a1a" strokeWidth={2} />
              </div>
              <span className="navbar__logo-text">Nestera</span>
            </Link>
          </div>

          <div className="navbar__nav">
            {navLinks.map((link) => (
              <Link
                key={link.href}
                href={link.href}
                className={`navbar__nav-link ${
                  isActiveLink(link.href) ? "active" : ""
                }`}
              >
                {link.label}
              </Link>
            ))}
          </div>

          <div className="navbar__actions">
            <button className="navbar__wallet-btn">Connect Wallet</button>

            <button
              onClick={() => setIsMobileMenuOpen(!isMobileMenuOpen)}
              className="navbar__mobile-toggle"
              aria-expanded={isMobileMenuOpen}
            >
              <span className="sr-only">Open main menu</span>
              {isMobileMenuOpen ? (
                <svg
                  className="navbar__mobile-toggle-icon"
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    d="M6 18L18 6M6 6l12 12"
                  />
                </svg>
              ) : (
                <svg
                  className="navbar__mobile-toggle-icon"
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    d="M4 6h16M4 12h16M4 18h16"
                  />
                </svg>
              )}
            </button>
          </div>
        </div>
      </div>

      <div className={`navbar__mobile-menu ${isMobileMenuOpen ? "open" : ""}`}>
        <div className="navbar__mobile-menu-content">
          {navLinks.map((link) => (
            <Link
              key={link.href}
              href={link.href}
              className={`navbar__mobile-menu-link ${
                isActiveLink(link.href) ? "active" : ""
              }`}
              onClick={() => setIsMobileMenuOpen(false)}
            >
              {link.label}
            </Link>
          ))}
          <button className="navbar__mobile-wallet-btn">Connect Wallet</button>
        </div>
      </div>
    </nav>
  );
};

export default Navbar;
