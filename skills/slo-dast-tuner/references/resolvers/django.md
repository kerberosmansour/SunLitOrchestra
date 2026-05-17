---
name: resolver-django
status: stable-reference
validation: spec-only:from-framework-docs
---

# Adapter: Django / Django REST Framework (Python)

**Detection.** `Django`/`djangorestframework` in `requirements.txt`/`pyproject`;
`urls.py`; `settings.py`; `ROOT_URLCONF`.

**Entry-point declaration model.** Central URLconf. `urlpatterns = [path('users/<int:id>/',
views.detail), re_path(r'^x/(?P<pk>\d+)/$', V.as_view())]`. `include('app.urls')` composes
prefixes (root urls.py → app urls.py). Views: function `def view(request)`, class-based
(`View`/`APIView`/`ViewSet`). DRF routers: `router.register('users', UserViewSet)` ⇒
auto CRUD routes (`/users/`, `/users/{pk}/`). Inputs: `request.GET/POST`, `request.data`,
URL kwargs, `serializer.validated_data`.

**Path/selector template syntax.** `<int:id>`, `<slug:s>`, `<str:x>`,
`(?P<name>regex)`. Address = include-prefix chain + path; DRF viewset action → method
(list=GET `/users/`, retrieve=GET `/users/{pk}/`, create=POST, update=PUT/PATCH, destroy=DELETE).

**Auth-marker vocabulary.** `@login_required`/`@permission_required` decorators;
`LoginRequiredMixin`/`PermissionRequiredMixin` on CBV; DRF
`permission_classes=[IsAuthenticated|IsAdminUser]`, `authentication_classes`; global
`REST_FRAMEWORK['DEFAULT_PERMISSION_CLASSES']`. `AdminUser`/`DjangoModelPermissions` ⇒ role.

**Sink→entry-point resolution.**
1. Sink in a view/serializer/model method → identify the view class/func.
2. Grep `urls.py`/router registrations referencing it; compose `include()` prefixes; for a
   ViewSet map the sink's action method (`list/create/retrieve/update`) to verb+path.
3. Auth = view decorator/mixin/`permission_classes`, else global DRF default permission.
4. `settings.py`/middleware/management-command sinks ⇒ `unresolved`.

**Worked example.**
`api/views.py:54` `UserViewSet.create` (`User.objects.raw(f"… {request.data['email']}")`,
`sqli`); `router.register('users', UserViewSet)` under root `path('api/', include('api.urls'))`;
`permission_classes=[IsAuthenticated]` ⇒
`{http, POST, /api/users/, requires_auth:true, auth_kind:session|bearer, vuln_class:sqli, param:email}`.

**Validation status.** `spec-only:from-framework-docs`.
