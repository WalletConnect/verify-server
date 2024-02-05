local grafana     = import 'grafonnet-lib/grafana.libsonnet';
local panels      = import 'panels/panels.libsonnet';

local dashboard   = grafana.dashboard;
local row         = grafana.row;
local annotation  = grafana.annotation;
local layout      = grafana.layout;

local ds    = {
  prometheus: {
    type: 'prometheus',
    uid:  std.extVar('prometheus_uid'),
  },
  cloudwatch: {
    type: 'cloudwatch',
    uid:  std.extVar('cloudwatch_uid'),
  },
};
local vars  = {
  namespace:        'Verify',
  environment:      std.extVar('environment'),
  notifications:    std.parseJson(std.extVar('notifications')),

  ecs_service_name: std.extVar('ecs_service_name'),
  redis_cluster_id: std.extVar('redis_cluster_id'),
  load_balancer:    std.extVar('load_balancer'),
  target_group:     std.extVar('target_group'),
};

////////////////////////////////////////////////////////////////////////////////

local height    = 8;
local pos       = grafana.layout.pos(height);

////////////////////////////////////////////////////////////////////////////////

dashboard.new(
  title         = std.extVar('dashboard_title'),
  uid           = std.extVar('dashboard_uid'),
  editable      = true,
  graphTooltip  = dashboard.graphTooltips.sharedCrosshair,
  timezone      = dashboard.timezones.utc,
)
.addAnnotation(
  annotation.new(
    target = {
      limit:    100,
      matchAny: false,
      tags:     [],
      type:     'dashboard',
    },
  )
)

.addPanels(layout.generate_grid([
  //////////////////////////////////////////////////////////////////////////////
  row.new('HTTP Server'),
    panels.http.response_status(ds, vars)   { gridPos: pos.one_third },
    panels.http.request_response(ds, vars)  { gridPos: pos.two_thirds },
    panels.http.latency_quantiles(ds, vars) { gridPos: pos.one_third },
    panels.http.average_latency(ds, vars)   { gridPos: pos.two_thirds },

  //////////////////////////////////////////////////////////////////////////////
  row.new('ECS'),
    panels.ecs.cpu(ds, vars)                { gridPos: pos._2 },
    panels.ecs.memory(ds, vars)             { gridPos: pos._2 },

  //////////////////////////////////////////////////////////////////////////////
  row.new('Project Registry'),
    panels.registry.requests(ds, vars)      { gridPos: pos._3 },
    panels.registry.cache_read(ds, vars)    { gridPos: pos._3 },
    panels.registry.cache_write(ds, vars)   { gridPos: pos._3 },

  //////////////////////////////////////////////////////////////////////////////
  row.new('Redis'),
    panels.redis.cpu(ds, vars)              { gridPos: pos._2 },
    panels.redis.reads(ds, vars)            { gridPos: pos._2 },
    panels.redis.memory(ds, vars)           { gridPos: pos._2 },
    panels.redis.writes(ds, vars)           { gridPos: pos._2 },
]))
