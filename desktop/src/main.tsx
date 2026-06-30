import "./app/styles/index.scss";
import { createRoot } from "react-dom/client";
import { QueryProvider } from "./app/providers";
import App from "./app/App.tsx";

createRoot(document.getElementById("root")!).render(
    <QueryProvider>
        <App />
    </QueryProvider>
);
