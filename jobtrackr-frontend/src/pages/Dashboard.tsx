import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";

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

  useEffect(() => {
    const token = localStorage.getItem("token");
    if (!token) {
      navigate("/login");
      return;
    }

    fetch("http://127.0.0.1:8000/applications", {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    })
      .then((res) => res.json())
      .then((data) => setApps(data))
      .catch((err) => console.error("Error fetching apps:", err));
  }, []);

  const handleLogout = () => {
    localStorage.removeItem("token");
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