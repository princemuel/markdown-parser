import "./styles/main.css";
// A dependency graph that contains any wasm must all be imported
// asynchronously. This `main.js` file does the single async import, so
// that no one else needs to worry about it again.
import("./scripts/index.ts").catch((e) =>
  console.error("Error importing `index.js`:", e)
);
