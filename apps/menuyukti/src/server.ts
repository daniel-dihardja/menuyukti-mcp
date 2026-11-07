import { Skybridge } from "skybridge/server";
import { z } from "zod";
import {
  computeMenuEngineeringFromOrders,
} from "./lib/analytics/index.js";
import {
  buildMenuHeatmapsPayload,
  buildPairLiftPayload,
  buildVenueSlotPayload,
  buildWeakItemsPayload,
  DEFAULT_MENU_HEATMAP_TOP_N,
  WeakItemsPayloadError,
} from "./lib/analytics-payloads.js";
import {
  menuHeatmapsPayloadSchema,
  pairLiftPayloadSchema,
  venueSlotPayloadSchema,
  weakItemsPayloadSchema,
} from "./lib/analytics-schemas.js";
import { CogsError, loadWorkspaceCogs } from "./lib/cogs.js";
import { LoadError, loadNormalizedSales } from "./lib/load.js";
import { errJson, okJson, widgetJson } from "./lib/mcp-result.js";
import { listWorkspaceFiles, listWorkspaces } from "./lib/paths.js";
import { getSetupStatus } from "./lib/setup.js";
import { listNormalizedRows, summarizeSales } from "./lib/summary.js";
import { VERSION } from "./lib/version.js";
import {
  readWorkspaceKnowledge,
  writeWorkspaceConfig,
  writeWorkspaceKnowledge,
  type WorkspaceConfig,
} from "./lib/workspace.js";

const workspacePathSchema = {
  path: z
    .string()
    .describe(
      "Relative or absolute path to a workspace directory under MENUYUKTI_DATA_DIR.",
    ),
};

const fileRoleSchema = z.object({
  path: z.string(),
  description: z.string().optional(),
});

const workspaceConfigSchema = z.object({
  version: z.number().int(),
  name: z.string(),
  venue: z.string(),
  period: z
    .object({
      label: z.string().optional(),
      start: z.string().optional(),
      end: z.string().optional(),
    })
    .optional(),
  currency: z.string().optional(),
  files: z.object({
    sales: fileRoleSchema,
    cogs: fileRoleSchema.optional(),
    knowledge: fileRoleSchema.optional(),
  }),
});

const salesPathSchema = {
  path: z
    .string()
    .describe(
      "Workspace folder (reads files.sales from workspace.yaml) or an Excel file under MENUYUKTI_DATA_DIR.",
    ),
};

const menuHeatmapsPathSchema = {
  ...salesPathSchema,
  limit: z
    .number()
    .int()
    .positive()
    .max(500)
    .optional()
    .describe(
      `Max menus to return, ranked by quantity. Omit for all menus on get_menu_heatmaps; render_menu_heatmaps defaults to ${DEFAULT_MENU_HEATMAP_TOP_N}.`,
    ),
  active_hours_only: z
    .boolean()
    .optional()
    .describe(
      "When true, daily_heatmap rows are limited to hours with demand across the returned menus. Default false for get_menu_heatmaps; true for render_menu_heatmaps.",
    ),
};

/**
 * Menuyukti Skybridge MCP (migration).
 * Workspace, sales load/summary, analytics get_/analyze_, and MCP Apps render_ views are ported.
 */
export const app = new Skybridge({
  name: "menuyukti",
  version: VERSION,
  instructions: [
    "Local Menuyukti MCP (Skybridge): restaurant POS Excel analytics on the owner's machine.",
    "Files stay under MENUYUKTI_DATA_DIR; never leave the local sandbox.",
    "Owner ritual: (1) keep this MCP server running and connected,",
    "(2) drop sales Excel (+ optional COGS) into one folder per venue/month under the data dir,",
    "(3) ask you to set up the workspace, then ask outcome questions.",
    "First contact: call get_setup_status (and list_workspaces if useful).",
    "Onboard a new folder: list_workspace_files → infer sales/COGS roles → write_workspace_config → summarize_local_sales.",
    "Goal→tool: how did we do → summarize_local_sales; cleaned line items → list_normalized_sales;",
    "stars/plow horses/puzzles/weak matrix → analyze_menu_engineering (needs sales+COGS).",
    "For show/chart/heatmap/table: prefer render_venue_slot_strength, render_pair_lift_matrix,",
    "render_menu_heatmaps, or render_weak_items_review when the host mounts MCP Apps widgets;",
    "otherwise use get_venue_slot_strength / get_pair_lift_matrix / get_menu_heatmaps / get_weak_items_review",
    "and let a presentation skill draw once. Never both render_ and get_ for the same chart in one turn.",
    "render_hello_world_widget is UI smoke only (mock data) — never use it for analytics.",
  ].join(" "),
  handler: (server) =>
    server
      .registerTool(
        {
          name: "get_setup_status",
          description:
            "Diagnose first-run setup under MENUYUKTI_DATA_DIR: resolved data dir, server version, workspace folders, which folders lack workspace.yaml, and whether demo-cafe-july-2026 is present. No path argument. Call on first contact or when path/config tools fail.",
          annotations: {
            title: "Setup status",
            readOnlyHint: true,
            destructiveHint: false,
            openWorldHint: false,
          },
        },
        async () => {
          try {
            const status = getSetupStatus();
            return okJson({
              server: "menuyukti",
              version: status.version,
              data_dir: status.data_dir,
              workspace_count: status.workspace_count,
              workspaces: status.workspaces,
              folders_missing_workspace_yaml:
                status.folders_missing_workspace_yaml,
              demo_workspace_present: status.demo_workspace_present,
            });
          } catch (err) {
            return errJson(err instanceof Error ? err.message : String(err));
          }
        },
      )
      .registerTool(
        {
          name: "list_workspaces",
          description:
            "List analysis workspaces under MENUYUKTI_DATA_DIR (typically workspaces/). Groups Excel files by workspace folder and returns relative file paths you can pass to the other sales tools. Ignores non-Excel files.",
          annotations: {
            title: "List workspaces",
            readOnlyHint: true,
            destructiveHint: false,
            openWorldHint: false,
          },
        },
        async () => {
          try {
            return okJson(listWorkspaces());
          } catch (err) {
            return errJson(err instanceof Error ? err.message : String(err));
          }
        },
      )
      .registerTool(
        {
          name: "list_workspace_files",
          description:
            "List all files in one workspace directory under MENUYUKTI_DATA_DIR (Excel, workspace.yaml, notes, and other docs). Path must be a workspace folder (for example demo-cafe-july-2026). Skips Excel lock files (~$*). Use this first when onboarding a new folder.",
          inputSchema: workspacePathSchema,
          annotations: {
            title: "List workspace files",
            readOnlyHint: true,
            destructiveHint: false,
            openWorldHint: false,
          },
        },
        async ({ path }) => {
          try {
            return okJson(listWorkspaceFiles(path));
          } catch (err) {
            return errJson(err instanceof Error ? err.message : String(err));
          }
        },
      )
      .registerTool(
        {
          name: "write_workspace_config",
          description:
            "Write workspace.yaml for a workspace folder under MENUYUKTI_DATA_DIR. Use after list_workspace_files when onboarding: set sales/cogs roles, venue, period, and currency. Overwrites an existing config. Role paths must be relative to the workspace folder.",
          inputSchema: {
            path: workspacePathSchema.path,
            config: workspaceConfigSchema,
          },
          annotations: {
            title: "Write workspace config",
            readOnlyHint: false,
            destructiveHint: true,
            openWorldHint: false,
          },
        },
        async ({ path, config }) => {
          try {
            return okJson(
              writeWorkspaceConfig(path, config as WorkspaceConfig),
            );
          } catch (err) {
            return errJson(err instanceof Error ? err.message : String(err));
          }
        },
      )
      .registerTool(
        {
          name: "read_workspace_knowledge",
          description:
            "Read knowledge.md (location/venue notes) for a workspace folder under MENUYUKTI_DATA_DIR. Uses files.knowledge when set, otherwise knowledge.md. Missing file returns exists=false with empty content. Call this before write_workspace_knowledge so rewrites keep prior notes.",
          inputSchema: workspacePathSchema,
          annotations: {
            title: "Read workspace knowledge",
            readOnlyHint: true,
            destructiveHint: false,
            openWorldHint: false,
          },
        },
        async ({ path }) => {
          try {
            return okJson(readWorkspaceKnowledge(path));
          } catch (err) {
            return errJson(err instanceof Error ? err.message : String(err));
          }
        },
      )
      .registerTool(
        {
          name: "write_workspace_knowledge",
          description:
            "Create or fully replace knowledge.md for a workspace folder under MENUYUKTI_DATA_DIR. Store location/venue facts (hours, seating, cuisine, constraints, past decisions). Full replace — call read_workspace_knowledge first. When workspace.yaml exists without files.knowledge, sets that role to knowledge.md.",
          inputSchema: {
            path: workspacePathSchema.path,
            content: z.string().describe("Full markdown content to write."),
          },
          annotations: {
            title: "Write workspace knowledge",
            readOnlyHint: false,
            destructiveHint: true,
            openWorldHint: false,
          },
        },
        async ({ path, content }) => {
          try {
            return okJson(writeWorkspaceKnowledge(path, content));
          } catch (err) {
            return errJson(err instanceof Error ? err.message : String(err));
          }
        },
      )
      .registerTool(
        {
          name: "summarize_local_sales",
          description:
            "Summarize local POS sales. Path may be a workspace folder (reads files.sales from workspace.yaml) or an Excel file under MENUYUKTI_DATA_DIR. Returns detected POS, period, revenue, quantity, and a capped top-item list.",
          inputSchema: salesPathSchema,
          annotations: {
            title: "Summarize local sales",
            readOnlyHint: true,
            destructiveHint: false,
            openWorldHint: false,
          },
        },
        async ({ path }) => {
          try {
            const loaded = loadNormalizedSales(path);
            return okJson(summarizeSales(loaded.pos, loaded.rows));
          } catch (err) {
            const message =
              err instanceof LoadError || err instanceof Error
                ? err.message
                : String(err);
            return errJson(message);
          }
        },
      )
      .registerTool(
        {
          name: "list_normalized_sales",
          description:
            "List plain normalized line items from local POS sales. Path may be a workspace folder (workspace.yaml sales role) or an Excel file under MENUYUKTI_DATA_DIR. Returns detected POS and the full cleaned row list (no aggregates).",
          inputSchema: salesPathSchema,
          annotations: {
            title: "List normalized sales",
            readOnlyHint: true,
            destructiveHint: false,
            openWorldHint: false,
          },
        },
        async ({ path }) => {
          try {
            const loaded = loadNormalizedSales(path);
            return okJson(listNormalizedRows(loaded.pos, loaded.rows));
          } catch (err) {
            const message =
              err instanceof LoadError || err instanceof Error
                ? err.message
                : String(err);
            return errJson(message);
          }
        },
      )
      .registerTool(
        {
          name: "get_menu_heatmaps",
          description:
            `Return per-menu daily (hourly) and weekly demand heatmaps from local POS sales. Path may be a workspace folder (workspace.yaml sales role) or an Excel file under MENUYUKTI_DATA_DIR. Sales alone is enough. When the path is a workspace with usable COGS, each item also includes matrix_category (star / plow_horse / puzzle / weak). Optional limit truncates to top menus by quantity; optional active_hours_only prunes empty daily hours. Response includes menu_total and limit_applied so agents know what was truncated. Prefer render_menu_heatmaps (default top ${DEFAULT_MENU_HEATMAP_TOP_N} + active hours) when the host mounts MCP Apps views.`,
          inputSchema: menuHeatmapsPathSchema,
          outputSchema: menuHeatmapsPayloadSchema,
          annotations: {
            title: "Menu heatmaps",
            readOnlyHint: true,
            destructiveHint: false,
            openWorldHint: false,
          },
        },
        async ({ path, limit, active_hours_only }) => {
          try {
            return okJson(
              buildMenuHeatmapsPayload(path, {
                topN: limit ?? null,
                activeHoursOnly: active_hours_only === true,
              }),
            );
          } catch (err) {
            return errJson(loadErrMessage(err));
          }
        },
      )
      .registerTool(
        {
          name: "get_venue_slot_strength",
          description:
            "Return venue slot strength: weekday × meal-period demand (order counts, traffic share, demand index, relative demand). Path may be a workspace folder (workspace.yaml sales role) or an Excel file under MENUYUKTI_DATA_DIR. Sales alone is enough — use for timing, staffing, and when to promote. Returns a fixed 35-cell slot_demand_profile.",
          inputSchema: salesPathSchema,
          outputSchema: venueSlotPayloadSchema,
          annotations: {
            title: "Venue slot strength",
            readOnlyHint: true,
            destructiveHint: false,
            openWorldHint: false,
          },
        },
        async ({ path }) => {
          try {
            return okJson(buildVenueSlotPayload(path));
          } catch (err) {
            return errJson(loadErrMessage(err));
          }
        },
      )
      .registerTool(
        {
          name: "analyze_menu_engineering",
          description:
            "Compute the menu engineering matrix (star / plow_horse / puzzle / weak) for a workspace folder under MENUYUKTI_DATA_DIR. Requires workspace.yaml with both files.sales and files.cogs. Returns thresholds, quadrant distribution, per-item classifications, and COGS coverage. If COGS is missing, list_workspace_files / write_workspace_config or ask the owner for unit costs first.",
          inputSchema: workspacePathSchema,
          annotations: {
            title: "Analyze menu engineering",
            readOnlyHint: true,
            destructiveHint: false,
            openWorldHint: false,
          },
        },
        async ({ path }) => {
          try {
            let cogs;
            try {
              cogs = loadWorkspaceCogs(path);
            } catch (err) {
              const message =
                err instanceof CogsError || err instanceof Error
                  ? err.message
                  : String(err);
              return errJson(
                `${message} Menu engineering needs sales + a unit-cost (COGS) Excel: ask the owner to drop COGS into the folder, then list_workspace_files and write_workspace_config (or call get_setup_status).`,
              );
            }
            const loaded = loadNormalizedSales(path);
            try {
              const matrix = computeMenuEngineeringFromOrders(
                loaded.rows,
                cogs.unit_costs,
              );
              return okJson({
                pos: loaded.pos,
                workspace: path,
                cogs_path: cogs.path,
                thresholds: matrix.thresholds,
                distribution: matrix.distribution,
                items: matrix.items,
                coverage: matrix.coverage,
              });
            } catch (err) {
              const message =
                err instanceof Error ? err.message : String(err);
              return errJson(
                `${message}. Fix COGS coverage (Menu + COGS columns matching sold menu names) or ask the owner to update unit costs.`,
              );
            }
          } catch (err) {
            return errJson(loadErrMessage(err));
          }
        },
      )
      .registerTool(
        {
          name: "get_weak_items_review",
          description:
            "Return Weak items review analytics for reprice / improve / remove decisions. Workspace folder under MENUYUKTI_DATA_DIR with files.sales and files.cogs required. Filters the menu-engineering matrix to the weak quadrant and adds period depth (avg qty/day, portfolio item/margin share). If COGS is missing, list_workspace_files / write_workspace_config or ask the owner for unit costs first. Prefer render_weak_items_review when the host mounts MCP Apps views and the owner should see the table; use this tool for JSON-only answers.",
          inputSchema: workspacePathSchema,
          outputSchema: weakItemsPayloadSchema,
          annotations: {
            title: "Weak items review",
            readOnlyHint: true,
            destructiveHint: false,
            openWorldHint: false,
          },
        },
        async ({ path }) => {
          try {
            return okJson(buildWeakItemsPayload(path));
          } catch (err) {
            if (err instanceof WeakItemsPayloadError) {
              return errJson(err.message);
            }
            return errJson(loadErrMessage(err));
          }
        },
      )
      .registerTool(
        {
          name: "get_pair_lift_matrix",
          description:
            "Return the pair lift matrix (co-purchase affinity) for combo and upsell ideas. Path may be a workspace folder (workspace.yaml sales role) or an Excel file under MENUYUKTI_DATA_DIR. Sales alone is enough (top menus by order presence). When the path is a workspace with files.cogs and menu engineering yields at least two stars, those stars become the focus. Returns focus_menus, matrix_lift, totals, scope, and ranked pairs.",
          inputSchema: salesPathSchema,
          outputSchema: pairLiftPayloadSchema,
          annotations: {
            title: "Pair lift matrix",
            readOnlyHint: true,
            destructiveHint: false,
            openWorldHint: false,
          },
        },
        async ({ path }) => {
          try {
            return okJson(buildPairLiftPayload(path));
          } catch (err) {
            return errJson(loadErrMessage(err));
          }
        },
      )
      .registerTool(
        {
          name: "render_venue_slot_strength",
          description:
            "Render the venue slot strength heatmap widget (weekday × meal-period demand). Path may be a workspace folder (workspace.yaml sales role) or an Excel file under MENUYUKTI_DATA_DIR. Sales alone is enough. Prefer this when the owner should see the timing chart; use get_venue_slot_strength for JSON-only answers.",
          inputSchema: salesPathSchema,
          outputSchema: venueSlotPayloadSchema,
          annotations: {
            title: "Render venue slot strength",
            readOnlyHint: true,
            destructiveHint: false,
            openWorldHint: false,
          },
          _meta: {
            "openai/toolInvocation/invoking":
              "Rendering venue slot strength…",
            "openai/toolInvocation/invoked": "Venue slot strength ready.",
          },
          view: {
            component: "venue-slot-strength",
            description:
              "Weekday × meal-period venue demand heatmap with demand/orders/share toggle",
          },
        },
        async ({ path }) => {
          try {
            const payload = buildVenueSlotPayload(path);
            return widgetJson(
              payload,
              `Showing venue slot strength heatmap (${payload.pos}, ${payload.slot_count} slots).`,
            );
          } catch (err) {
            return errJson(loadErrMessage(err));
          }
        },
      )
      .registerTool(
        {
          name: "render_pair_lift_matrix",
          description:
            "Render the pair lift index heatmap widget (focus menu × focus menu co-purchase lift). Path may be a workspace folder (workspace.yaml sales role) or an Excel file under MENUYUKTI_DATA_DIR. Sales alone is enough; COGS upgrades focus to stars when ≥2. Prefer this when the owner should see the combo chart; use get_pair_lift_matrix for JSON-only answers.",
          inputSchema: salesPathSchema,
          outputSchema: pairLiftPayloadSchema,
          annotations: {
            title: "Render pair lift matrix",
            readOnlyHint: true,
            destructiveHint: false,
            openWorldHint: false,
          },
          _meta: {
            "openai/toolInvocation/invoking": "Rendering pair lift matrix…",
            "openai/toolInvocation/invoked": "Pair lift matrix ready.",
          },
          view: {
            component: "pair-lift-matrix",
            description:
              "Focus menu × focus menu co-purchase lift heatmap with column sort",
          },
        },
        async ({ path }) => {
          try {
            const payload = buildPairLiftPayload(path);
            return widgetJson(
              payload,
              `Showing pair lift index heatmap (${payload.pos}, ${payload.focus_menus.length} focus menus).`,
            );
          } catch (err) {
            return errJson(loadErrMessage(err));
          }
        },
      )
      .registerTool(
        {
          name: "render_menu_heatmaps",
          description:
            `Render the menu heatmaps widget (per-menu daily hour and weekly weekday unit quantities). Path may be a workspace folder (workspace.yaml sales role) or an Excel file under MENUYUKTI_DATA_DIR. Sales alone is enough; workspace COGS adds matrix_category filters (weak off by default). Defaults to top ${DEFAULT_MENU_HEATMAP_TOP_N} menus by quantity and active daily hours only (override with limit / active_hours_only). Response fields menu_total and limit_applied describe truncation. Prefer this when the owner should see the dish timing chart; use get_menu_heatmaps for full JSON.`,
          inputSchema: menuHeatmapsPathSchema,
          outputSchema: menuHeatmapsPayloadSchema,
          annotations: {
            title: "Render menu heatmaps",
            readOnlyHint: true,
            destructiveHint: false,
            openWorldHint: false,
          },
          _meta: {
            "openai/toolInvocation/invoking": "Rendering menu heatmaps…",
            "openai/toolInvocation/invoked": "Menu heatmaps ready.",
          },
          view: {
            component: "menu-heatmaps",
            description:
              "Per-menu daily/weekly quantity heatmaps with category filters",
          },
        },
        async ({ path, limit, active_hours_only }) => {
          try {
            const payload = buildMenuHeatmapsPayload(path, {
              topN: limit ?? DEFAULT_MENU_HEATMAP_TOP_N,
              activeHoursOnly: active_hours_only !== false,
            });
            const truncated =
              payload.limit_applied != null &&
              payload.menu_total > payload.menu_count
                ? `, top ${payload.menu_count} of ${payload.menu_total}`
                : `, ${payload.menu_count} menus`;
            return widgetJson(
              payload,
              `Showing menu heatmaps (${payload.pos}${truncated}${payload.active_hours_only ? ", active hours" : ""}).`,
            );
          } catch (err) {
            return errJson(loadErrMessage(err));
          }
        },
      )
      .registerTool(
        {
          name: "render_weak_items_review",
          description:
            "Render the weak items review widget (sortable weak-quadrant table with portfolio KPIs). Workspace folder under MENUYUKTI_DATA_DIR with files.sales and files.cogs required. Prefer this when the owner should see the weak items table; use get_weak_items_review for JSON-only answers. Never also redraw HTML via a presentation skill in the same turn.",
          inputSchema: workspacePathSchema,
          outputSchema: weakItemsPayloadSchema,
          annotations: {
            title: "Render weak items review",
            readOnlyHint: true,
            destructiveHint: false,
            openWorldHint: false,
          },
          _meta: {
            "openai/toolInvocation/invoking":
              "Rendering weak items review…",
            "openai/toolInvocation/invoked": "Weak items review ready.",
          },
          view: {
            component: "weak-items-review",
            description:
              "Sortable weak-quadrant table with portfolio KPI strip",
          },
        },
        async ({ path }) => {
          try {
            const payload = buildWeakItemsPayload(path);
            return widgetJson(
              payload,
              `Showing weak items review (${payload.pos}, ${payload.items.length} weak items).`,
            );
          } catch (err) {
            if (err instanceof WeakItemsPayloadError) {
              return errJson(err.message);
            }
            return errJson(loadErrMessage(err));
          }
        },
      )
      .registerTool(
        {
          name: "render_hello_world_widget",
          description:
            "MCP Apps smoke widget that renders mock Hello World structuredContent. Not for analytics.",
          annotations: {
            title: "Hello World widget",
            readOnlyHint: true,
            destructiveHint: false,
            openWorldHint: false,
          },
          _meta: {
            "openai/toolInvocation/invoking": "Rendering Hello World widget…",
            "openai/toolInvocation/invoked": "Hello World widget ready.",
          },
          view: {
            component: "hello-world",
            description: "Hello World smoke widget",
          },
        },
        async () => {
          const structuredContent = {
            title: "Hello World",
            message: "Menuyukti Skybridge MCP Apps smoke widget.",
            source: "apps/menuyukti",
          };
          return {
            structuredContent,
            content: [
              {
                type: "text" as const,
                text: "Showing the Hello World MCP Apps widget (mock data).",
              },
            ],
            isError: false,
          };
        },
      ),
});

function loadErrMessage(err: unknown): string {
  if (err instanceof LoadError || err instanceof Error) return err.message;
  return String(err);
}

export type AppType = typeof app;
