import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";

export default function Dashboard() {
  const navigate = useNavigate();
  const [checkingAuth, setCheckingAuth] = useState(true);

  useEffect(() => {
    const token = localStorage.getItem("token");
    if (!token) {
      navigate("/login");
    } else {
      setCheckingAuth(false); // ✅ only render if token exists
    }
  }, [navigate]);

  if (checkingAuth) return <p>🔐 Checking auth...</p>;

  return (
    <div style={{ padding: "2rem" }}>
      <h2>Welcome to your Dashboard 🎯</h2>
      <p>You are now authenticated and ready to crush job apps.</p>
      <button onClick={() => {
        localStorage.removeItem("token");
        navigate("/login");
      }}>Logout</button>
    </div>
  );
}
