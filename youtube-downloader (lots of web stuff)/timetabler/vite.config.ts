import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  base: './',
  server: {
    proxy: {
      '/api': 'http://127.0.0.1:8000',
    }  
  } ,
  build: {
    outDir: 'API/dist',
    emptyOutDir: true
  },
  plugins: [react()],
})
