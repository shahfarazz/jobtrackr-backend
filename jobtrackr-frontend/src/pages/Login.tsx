import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { useEffect } from "react";
import { useAuth } from "../context/AuthContext"; // Importing AuthContext

console.log("🧭 Inside Login page");
export default function Login() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [message, setMessage] = useState("");
  const navigate = useNavigate(); // 🚀 use React Router for redirection
  const {setToken } = useAuth();

  
  useEffect(() => {
    const token = localStorage.getItem("token");
    if (token) {
      navigate("/"); // Already logged in? Redirect to dashboard
    }
  }, []);

  const handleLogin = async (e: React.FormEvent) => {
    console.log("🔁 handleLogin triggered");
    e.preventDefault();
    try {
      const res = await fetch("http://127.0.0.1:8000/auth/login", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ email, password }),
      });

      if (!res.ok) {
        setMessage("❌ Login failed. Check your credentials.");
        return;
      }

      const data = await res.json();
      localStorage.setItem("token", data.token);
      setToken(data.token); // Update the context state
      setMessage("✅ Login successful!");
      console.log("👉 About to navigate to /");
      navigate("/");

    } catch {
      setMessage("⚠️ Network error. Please try again.");
    }
  };

  return (
    <div className="login-container">
      <div className="login-card">
        <h2 className="text-2xl font-bold text-center mb-6">Login to JobTrackr</h2>
        <form onSubmit={handleLogin} className="flex flex-col items-center space-y-4">
          <input
            type="email"
            placeholder="Email"
            value={email}
            required
            onChange={(e) => setEmail(e.target.value)}
            className="login-input"
          />
          <input
            type="password"
            placeholder="Password"
            value={password}
            required
            onChange={(e) => setPassword(e.target.value)}
            className="login-input"
          />
          <button type="submit" className="login-btn">
            Login
          </button>
        </form>
        {message && <p className="login-message">{message}</p>}
      </div>
    </div>
  );
  
  
  
}
