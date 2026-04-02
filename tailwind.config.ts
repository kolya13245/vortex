import type { Config } from "tailwindcss";
import plugin from "tailwindcss/plugin";

export default {
  content: ["./index.html", "./src/**/*.{ts,tsx}"],
  darkMode: ["selector", '[data-theme="dark"]'],
  theme: {
    extend: {
      fontFamily: {
        sans: ["Inter", "system-ui", "-apple-system", "sans-serif"],
        mono: ["JetBrains Mono", "ui-monospace", "monospace"],
      },
      colors: {
        "bg-base": "rgb(var(--color-bg-base) / <alpha-value>)",
        accent: {
          DEFAULT: "rgb(var(--color-accent) / <alpha-value>)",
          hover: "#00e8bc",
          muted: "rgba(0, 212, 170, 0.10)",
          glow: "rgba(0, 212, 170, 0.25)",
        },
        danger: {
          DEFAULT: "#ef4444",
          hover: "#f87171",
        },
        status: {
          connected: "rgb(var(--color-status-connected) / <alpha-value>)",
          connecting: "rgb(var(--color-status-connecting) / <alpha-value>)",
          error: "rgb(var(--color-status-error) / <alpha-value>)",
        },
      },
      animation: {
        "orb-glow": "orb-glow 3s ease-in-out infinite",
        "orb-pulse": "orb-pulse 1.5s ease-in-out infinite",
        "orb-pulse-fast": "orb-pulse 1s ease-in-out infinite",
        "modal-in": "modal-in 200ms ease-out",
        "fade-in": "fade-in 150ms ease-out",
        "pulse-slow": "pulse 3s ease-in-out infinite",
      },
      keyframes: {
        "orb-glow": {
          "0%, 100%": { opacity: "0.6", transform: "scale(1)" },
          "50%": { opacity: "1", transform: "scale(1.08)" },
        },
        "orb-pulse": {
          "0%, 100%": {
            boxShadow: "0 0 0 0 var(--pulse-color, rgba(0,212,170,0.3))",
          },
          "50%": {
            boxShadow:
              "0 0 0 14px var(--pulse-color-end, rgba(0,212,170,0))",
          },
        },
        "modal-in": {
          from: { opacity: "0", transform: "scale(0.96) translateY(8px)" },
          to: { opacity: "1", transform: "scale(1) translateY(0)" },
        },
        "fade-in": {
          from: { opacity: "0", transform: "translateY(4px)" },
          to: { opacity: "1", transform: "translateY(0)" },
        },
      },
      boxShadow: {
        glass: "0 8px 32px rgba(0, 0, 0, 0.20)",
        "glass-lg": "0 16px 64px rgba(0, 0, 0, 0.35)",
        "accent-glow": "0 0 20px rgba(0, 212, 170, 0.12)",
        "accent-glow-lg": "0 0 40px rgba(0, 212, 170, 0.20)",
      },
    },
  },
  plugins: [
    plugin(function ({ addUtilities }) {
      addUtilities({
        ".glass": {
          background: "var(--glass-bg)",
          "backdrop-filter": "blur(16px) saturate(180%)",
          "-webkit-backdrop-filter": "blur(16px) saturate(180%)",
          border: "1px solid var(--glass-border)",
        },
        ".glass-elevated": {
          background: "var(--glass-bg-elevated)",
          "backdrop-filter": "blur(16px) saturate(180%)",
          "-webkit-backdrop-filter": "blur(16px) saturate(180%)",
          border: "1px solid var(--glass-border)",
          "box-shadow": "0 8px 32px rgba(0, 0, 0, 0.30)",
        },
        ".glass-subtle": {
          background: "var(--glass-bg)",
          "backdrop-filter": "blur(8px) saturate(150%)",
          "-webkit-backdrop-filter": "blur(8px) saturate(150%)",
          border: "1px solid var(--glass-border-subtle)",
        },
        ".glass-none": {
          background: "transparent",
          "backdrop-filter": "none",
          "-webkit-backdrop-filter": "none",
          border: "none",
        },
      });
    }),
  ],
} satisfies Config;
