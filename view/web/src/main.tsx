/** Frontend entry point. Mounts the React application into #root. */
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App";
import "./styles.css";

// HINT: React 19 uses createRoot() instead of the legacy ReactDOM.render().
// StrictMode double-invokes effects in development to surface side-effect
// mistakes early — it does nothing in production builds.
const container = document.getElementById("root");
if (!container) {
  throw new Error("Missing #root element in index.html");
}

createRoot(container).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
