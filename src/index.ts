/** Groundset's globally deployed HTTP entrypoint. */
export default {
  fetch(): Response {
    return Response.json({ name: "Groundset", status: "ok" });
  },
} satisfies ExportedHandler<Env>;
