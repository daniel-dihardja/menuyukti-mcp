import { enableSelfContainedProductionViews } from "./self-contained-views.js";

await enableSelfContainedProductionViews();

const { app } = await import("./server.js");

export default await app.run();
