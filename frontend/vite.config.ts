import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'

// https://vitejs.dev/config/
export default defineConfig({
  envDir: '../',
  plugins: [react()],

  preview: {
    port: 2000
  },
  server: {
    port: 2000
  }
})
