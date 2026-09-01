import solid from "vite-plugin-solid";
import { defineConfig } from "vitest/config";

export default defineConfig({
  plugins: [solid()],
  server: {
    port: 4173,
  },
  build: {
    target: "esnext",
  },
  test: {
    environment: "jsdom",
  },
});
