import { BrowserRouter, Routes, Route } from "react-router-dom";
import Login from "./pages/Login";
import Dashboard from "./pages/Dashboard";
import NewApplication from "./pages/NewApplication";
import ProtectedRoute from "./components/ProtectedRoute";
export default function App() {
  return (
    <div className="min-h-screen flex flex-col">
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<ProtectedRoute><Dashboard /></ProtectedRoute>} />
          <Route path="/login" element={<Login />} />
          <Route path="/new" element={<NewApplication />} />
        </Routes>
      </BrowserRouter>
    </div>
  );
}
