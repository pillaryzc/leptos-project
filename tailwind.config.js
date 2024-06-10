/** @type {import('tailwindcss').Config} */
module.exports = {
  content: { 
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      keyframes: {
        'color-change': {
          '0%, 100%': { color: '#00C6FF' },
          '50%': { color: '#0072FF' },
        },
      },
      animation: {
        'color-change': 'color-change 3s infinite',
      },
    },
  },
  plugins: [],
}