---
name: resolver-rails
status: stable-reference
validation: spec-only:from-framework-docs
---

# Adapter: Ruby on Rails

**Detection.** `rails` in `Gemfile`; `config/routes.rb`; `app/controllers/*_controller.rb`.

**Entry-point declaration model.** Central DSL in `config/routes.rb`:
`resources :users` (⇒ index/show/new/create/edit/update/destroy REST set),
`get '/x', to: 'controller#action'`, `post 'y' => 'c#a'`, `namespace :api { … }`,
`scope path: '/v1'`, `constraints`. Controller actions are methods in
`UsersController < ApplicationController`. Inputs: `params[:id]`, `params[:user][:email]`
(strong params `params.require(:user).permit(...)`).

**Path/selector template syntax.** `:param` (`/users/:id`), `*glob`. `resources` mapping:
index→GET `/users`, show→GET `/users/:id`, create→POST `/users`, update→PATCH/PUT
`/users/:id`, destroy→DELETE `/users/:id`. Address = namespace/scope chain + path.

**Auth-marker vocabulary.** `before_action :authenticate_user!` (Devise) /
`:require_login` in the controller or `ApplicationController` (global);
`before_action :require_admin` ⇒ role; `skip_before_action :authenticate_user!` marks
public actions. Pundit/CanCanCan `authorize`/`load_and_authorize_resource` ⇒ object authz.

**Sink→entry-point resolution.**
1. Sink in a model/service/controller → find the controller action method (the public
   method whose name matches a route action).
2. Resolve the route: scan `routes.rb` for `resources`/explicit mapping to
   `controller#action`; apply `namespace`/`scope` prefixes; expand `resources` to verb+path.
3. Auth = `before_action` auth filters on the controller/ApplicationController minus
   `skip_before_action`; role from admin filter / Pundit policy.
4. Rake tasks, initializers, jobs ⇒ `unresolved`.

**Worked example.**
`app/controllers/reports_controller.rb:19` `#export` (`ActiveRecord::Base.connection.execute("… #{params[:q]}")`,
`sqli`); routes `namespace :api { resources :reports do collection { get :export } } }`;
`before_action :authenticate_user!` ⇒
`{http, GET, /api/reports/export, requires_auth:true, auth_kind:session, vuln_class:sqli, param:q}`.

**Validation status.** `spec-only:from-framework-docs`.
