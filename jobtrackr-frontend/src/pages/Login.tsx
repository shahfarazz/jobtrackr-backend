import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { useEffect } from "react";

console.log("🧭 Inside Login page");
export default function Login() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [message, setMessage] = useState("");
  const navigate = useNavigate(); // 🚀 use React Router for redirection
  
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
      setMessage("✅ Login successful!");
      console.log("👉 About to navigate to /");
      navigate("/");

    } catch {
      setMessage("⚠️ Network error. Please try again.");
    }
  };

  return (
    <div style={{ maxWidth: 400, margin: "auto", padding: 20 }}>
      <h2>Login to JobTrackr</h2>
      <form onSubmit={handleLogin}>
        <input
          type="email"
          placeholder="Email"
          value={email}
          required
          onChange={(e) => setEmail(e.target.value)}
        /><br /><br />
        <input
          type="password"
          placeholder="Password"
          value={password}
          required
          onChange={(e) => setPassword(e.target.value)}
        /><br /><br />
        <button type="submit">Login</button>
      </form>
      {message && <p style={{ marginTop: "1rem", color: "gray" }}>{message}</p>}
    </div>
  );
}
