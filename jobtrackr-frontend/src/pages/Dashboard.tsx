import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import {jwtDecode} from "jwt-decode";
import { useAuth } from "../context/AuthContext";

interface JobApplication {
  id: string;
  user_id: string;
  company: string;
  position: string;
  status: string;
  applied_at: string;
  notes?: string;
}


export default function Dashboard() {
  const [apps, setApps] = useState([]);
  const navigate = useNavigate();
  const {token, setToken} = useAuth();

  useEffect(() => {
    const token = localStorage.getItem("token");
    if (!token) {
      navigate("/login");
      return;
    }

    const decoded = jwtDecode<{ sub: string }>(token);
    const userId = decoded.sub;

    fetch(`http://127.0.0.1:8000/applications?user_id=${userId}`, {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    })
      .then((res) => res.json())
      .then((data) => setApps(data))
      .catch((err) => console.error("Error fetching apps:", err));
  }, [navigate, token]);

  const handleLogout = () => {
    localStorage.removeItem("token");
    setToken(null); // Clear the context state
    navigate("/login");
  };

  return (
    <div style={{ padding: "2rem" }}>
      <h2>📄 Your Job Applications</h2>
      <button onClick={handleLogout} style={{ marginBottom: "1rem" }}>Logout</button>
      <ul>
        {apps.map((app: JobApplication) => (
          <li key={app.id}>
            {app.position} @ {app.company}
          </li>
        ))}
      </ul>
    </div>
  );
}