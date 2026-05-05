// WARNING: This tool can execute arbitrary SQL including INSERT/UPDATE/DELETE.
// Use a READONLY connection for safety.
// Depends on DATABASE_URL env var (supports: PostgreSQL, MySQL, SQLite, Turso, D1)
import { tool } from "@opencode-ai/plugin";
import { sql } from "bun";

export default tool({
  description: "Run arbitrary SQL against a database and return results",
  args: {
    sql: tool.schema
      .string()
      .describe("SQL query to execute (use $1, $2, etc. for bind variables)"),
    params: tool.schema
      .array(tool.schema.any())
      .optional()
      .describe("Bind variables to pass to the query"),
  },
  async execute(args: { sql: string; params?: any[] }) {
    console.log("ARGS", JSON.stringify(args, null, 2));
    const result = await sql.unsafe(args.sql, args.params);
    console.log("RESULT", JSON.stringify(result, null, 2));
    return JSON.stringify(result);
  },
});
