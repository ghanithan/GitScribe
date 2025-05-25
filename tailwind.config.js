/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{rs,html,css}",
    "./index.html",
    "./dist/**/*.html"
  ],
  theme: {
    extend: {
      colors: {
        primary: {
          DEFAULT: '#3b82f6',
          dark: '#2563eb',
          light: '#60a5fa',
        },
        secondary: {
          DEFAULT: '#10b981',
          dark: '#059669',
          light: '#34d399',
        },
        background: {
          light: '#ffffff',
          dark: '#1f2937',
        },
        text: {
          light: '#1f2937',
          dark: '#f9fafb',
        },
      },
    },
  },
  plugins: [],
  darkMode: 'class',
}