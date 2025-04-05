import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App'; // 👈 Make sure this exists

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
