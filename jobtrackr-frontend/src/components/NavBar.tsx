import { Link, useNavigate } from "react-router-dom";
import { useAuth } from "../context/AuthContext"; // Importing AuthContext

export default function Navbar() {
  const navigate = useNavigate();
  const { setToken } = useAuth(); // Accessing setToken from AuthContext

  const handleLogout = () => {
    localStorage.removeItem("token");
    setToken(null); // Clear the context state
    navigate("/login");
  };

  return (
    <nav style={{ display: "flex", alignItems: "center", gap: "1rem", padding: "1rem", borderBottom: "1px solid #ccc" }}>
      <Link to="/">Dashboard</Link>
      <button onClick={handleLogout}>Logout</button>
    </nav>
  );
  
}
