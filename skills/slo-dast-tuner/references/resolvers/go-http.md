---
name: resolver-go-http
status: stable-reference
validation: spec-only:from-framework-docs
---

# Adapter: Go (net/http, chi, gin, echo, gorilla/mux)

**Detection.** `go.mod`; imports `net/http`, `github.com/go-chi/chi`,
`github.com/gin-gonic/gin`, `github.com/labstack/echo`, `github.com/gorilla/mux`.

**Entry-point declaration model.**
- net/http: `mux.HandleFunc("/users/", handler)`, `http.Handle("/x", h)`.
- chi/mux: `r.Get("/users/{id}", handler)`, `r.Route("/api", func(r){ r.Post("/x", h) })`,
  `r.Mount("/v1", sub)` ⇒ nested prefix.
- gin: `r.GET("/users/:id", h)`, `g := r.Group("/api"); g.POST("/x", h)`.
- echo: `e.GET("/users/:id", h)`, `g := e.Group("/api"); g.POST("/x", h)`.
Inputs: `chi.URLParam(r,"id")`, `r.URL.Query().Get("q")`, `c.Param("id")`/`c.Query`/`c.Bind`.

**Path/selector template syntax.** chi/echo/gin `:id` or `{id}`; gorilla `{id:[0-9]+}`;
net/http prefix match (trailing `/`). Address = Route/Group/Mount prefix chain + path.

**Auth-marker vocabulary.** Middleware in the chain: `r.Use(AuthMiddleware)`,
`r.With(jwtauth.Verifier(...))`, gin `g.Use(AuthRequired())`, echo
`e.Use(middleware.JWT(...))` / group `g.Use(authMW)`; a wrapping `Auth(handler)`. Role via a
middleware checking claims/context. A router-level `Use` before route registration ⇒ global.

**Sink→entry-point resolution.**
1. Sink in a handler `func(w http.ResponseWriter, r *http.Request)` / `gin.HandlerFunc` /
   `echo.HandlerFunc` (or a service it calls — trace to the handler).
2. Find the registration binding that handler symbol; collect verb + path; compose
   `Route`/`Group`/`Mount` prefixes.
3. Auth = middleware on the route/group/router chain (`Use`/`With`/wrapper).
4. `main()` wiring/config, non-HTTP goroutines ⇒ `unresolved` (unless they're a server).

**Worked example.**
`handlers/file.go:40` (`os.Open("data/"+chi.URLParam(r,"name"))`, `path-traversal`);
`r.Route("/api", func(r){ r.With(jwtauth.Authenticator).Get("/files/{name}", DownloadFile) })` ⇒
`{http, GET, /api/files/{name}, requires_auth:true, auth_kind:bearer-jwt, vuln_class:path-traversal, param:name}`.

**Validation status.** `spec-only:from-framework-docs`.
