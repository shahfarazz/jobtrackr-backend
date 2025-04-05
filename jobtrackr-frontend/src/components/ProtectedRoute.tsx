import { Navigate } from "react-router-dom";
import {jwtDecode} from "jwt-decode";
import { JSX } from "react";

interface Claims {
  sub: string;
  exp: number;
}

export default function ProtectedRoute({ children }: { children: JSX.Element }) {
  const token = localStorage.getItem("token");

  if (!token) {
    return <Navigate to="/login" replace />;
  }

  try {
    const decoded = jwtDecode<Claims>(token);
    const isExpired = decoded.exp * 1000 < Date.now(); // Convert to ms

    if (isExpired) {
      localStorage.removeItem("token"); // Clear stale token
      return <Navigate to="/login" replace />;
    }
  } catch (err) {
    console.error("Invalid token:", err);
    localStorage.removeItem("token");
    return <Navigate to="/login" replace />;
  }

  return children;
}
