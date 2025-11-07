import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./components/App.tsx";
import "./index.css";

const queryClient = new QueryClient();

if (import.meta.env.DEV) {
  const devFont = "https://rsms.me/inter/font-files/InterVariable.ttf";
  const font = new FontFace("Foundry", `url(${devFont})`);
  await font.load();
  document.fonts.add(font);
}

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <QueryClientProvider client={queryClient}>
      <App />
    </QueryClientProvider>
  </StrictMode>
);
