import { defineConfig } from "vite";
import { glob } from "glob";

export default defineConfig({
  envPrefix: ["VITE_"],
  build: {
    emptyOutDir: true,
    target: "esnext",
    rollupOptions: {
      input: glob.sync("./frontend/templates/**/*.html")
    },
    outDir: "../src/dist/",
  },
  root: "./frontend",
});
