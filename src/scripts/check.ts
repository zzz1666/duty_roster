import { z } from "zod";
const schema = z.object({
  role_types: z.array(z.string()),
  groups: z.array(
    z.object({
      id: z.number(),
      assignments: z.array(z.array(z.string())),
    }),
  ),
});
export default (json: string|object) => {
  let parsed: unknown;
  if (typeof json == "string") {
    parsed = JSON.parse(json);
  } else if (typeof json == "object") {
    parsed = json
  } else throw new Error("error")
  const result = schema.parse(parsed);
  return result;
};
