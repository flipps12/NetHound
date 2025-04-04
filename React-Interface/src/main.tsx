import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { BrowserRouter, Route, Routes } from 'react-router-dom';
import './index.css'
import App from './pages/Login.tsx'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <BrowserRouter>
      <Routes>
        <Route path="/login" element={<App />} />
      </Routes>
    </BrowserRouter>
  </StrictMode>,
)
