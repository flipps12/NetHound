import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'
import tailwindcss from '@tailwindcss/vite'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    tailwindcss(),
    react()
  ],
  server: {
    port: 8080,
    host: true, // esto permite que sea accesible desde otras IPs
    allowedHosts: ["captive.portal.local"]
  },
})
