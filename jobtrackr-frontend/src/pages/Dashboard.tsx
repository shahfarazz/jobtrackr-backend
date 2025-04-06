import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import {jwtDecode} from "jwt-decode";
import Navbar from "../components/NavBar";
import { useAuth } from "../context/AuthContext"; // Importing AuthContext

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
  const { token } = useAuth(); // Accessing token from AuthContext
  

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

  

  return (
    <>
      <Navbar />
      <div style={{ padding: "2rem" }}>
        <h2>📄 Your Job Applications</h2>
        <ul>
          {apps.map((app: JobApplication) => (
            <li key={app.id}>
              {app.position} @ {app.company}
            </li>
          ))}
        </ul>
      </div>
    </>
  );
}