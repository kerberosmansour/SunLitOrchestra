---
name: resolver-graphql
status: spec-only:from-framework-docs
---

# Adapter: GraphQL (non-HTTP-route surface)

GraphQL has **one HTTP endpoint** (`/graphql`); the real entry points are
schema fields, not URL paths. `protocol: graphql`, `selector.path = "Type.field"`.

**Detection.** `graphql`, `apollo-server*`, `@nestjs/graphql`, `graphene`/`strawberry`
(Python), `graphql-ruby`, `graphql-java`; `*.graphql`/`*.gql` SDL; a `/graphql` route;
`typeDefs`/`buildSchema`/`makeExecutableSchema`.

**Entry-point declaration model.** SDL types + resolver map: `Query`/`Mutation`/
`Subscription` fields bound to resolver functions (`resolvers = { Query: { user: (_,args)=>… } }`,
or code-first decorators `@Resolver()/@Query()/@Mutation()`). The injectable input is the
field's `args`/`input` object, not a query string.

**Path/selector template syntax.** `selector = { method: "query|mutation|subscription",
path: "Query.user" | "Mutation.login", param: "<arg name>" }`. `address` = base + the
GraphQL HTTP endpoint (commonly `/graphql`); the operation is in the request body.

**Auth-marker vocabulary.** Field/resolver guards: `@nestjs` `@UseGuards(GqlAuthGuard)`,
`graphql-shield` permission rules, an auth check in `context`/at resolver top
(`if(!ctx.user) throw`), Apollo `context` building a user from a token, schema directives
`@auth`/`@hasRole`. A global context auth ⇒ all resolvers `requires_auth` unless explicitly
public.

**Sink→entry-point resolution.**
1. Sink in a resolver/service → find the resolver field (map key or `@Query/@Mutation`).
2. Read the SDL/type for that field name + parent type ⇒ `Type.field`, op kind, arg name.
3. Auth = resolver guard/shield rule/context check, or global context auth.
4. Schema build/config sinks ⇒ `unresolved`.
5. **Probe transfer:** use the GraphQL probe variant (injection/idor in a `query`/`mutation`
   document to `/graphql`, introspection for schema, aliases for enumeration) — **not** the
   REST payloads. Reflected-HTML XSS class generally N/A.

**Worked example.**
`resolvers/user.js:30` `Query.users` (`db.find({$where:'this.name=="'+args.q+'"'})`,
`nosqli`); `context` throws when no `ctx.user` ⇒
`{graphql, query, Query.users, address: base+/graphql, requires_auth:true, auth_kind:bearer-jwt, vuln_class:nosqli, param:q}`.

**Validation status.** `spec-only:from-framework-docs`.
