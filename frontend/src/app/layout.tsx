import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import "./globals.css";

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "Intelligence Rail — Cortex Protocol",
  description:
    "Open marketplace for autonomous AI agent intelligence assets. Discover, exchange, and monetize prompts, workflows, and reasoning chains via Stellar micropayments.",
  keywords: [
    "AI agents",
    "intelligence marketplace",
    "Stellar",
    "Soroban",
    "micropayments",
    "prompt marketplace",
  ],
  openGraph: {
    title: "Intelligence Rail — Cortex Protocol",
    description:
      "Open infrastructure for autonomous agents to discover, exchange, and evolve intelligence assets through programmable micropayments.",
    type: "website",
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className={`${geistSans.variable} ${geistMono.variable}`}>
      <body>{children}</body>
    </html>
  );
}
