/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        'primary': '#3873F2',
        'action-b': '#1e88e5',
        'action-g': '#43a047',
        'action-r': '#e53935',
        'action-gr': '#757575',
      },

      backgroundImage: {
        'gradient-radial': 'radial-gradient(var(--tw-gradient-stops))',
        'gradient-conic':
          'conic-gradient(from 180deg at 50% 50%, var(--tw-gradient-stops))',
      },
      borderWidth: {
        1: '1px',
      },
      fontFamily: {
        inter: ['Inter', 'sans-serif'],
      },
      height: {
        'input-px': '42px',
      }
    },
  },
  plugins: [],
  safelist: [
    'grid-cols-1',
    'grid-cols-2',
    'grid-cols-3',
    'grid-cols-4',
    'grid-cols-5',
    'grid-cols-6',
    'grid-cols-7',
    'grid-cols-8',
    'h-1/2',
    'h-1/3',
    'h-2/3',
    'h-1/4',
    'h-3/4',
    'h-1/5',
    'h-2/5',
    'h-1/6',
    'h'
  ]
}
