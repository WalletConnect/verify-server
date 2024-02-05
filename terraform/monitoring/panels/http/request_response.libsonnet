local grafana   = import '../../grafonnet-lib/grafana.libsonnet';
local defaults  = import '../../grafonnet-lib/defaults.libsonnet';

local panels = grafana.panels;
local targets = grafana.targets;

local _configuration = defaults.configuration.timeseries
  .withThresholdStyle(grafana.fieldConfig.thresholdStyle.Area)
  .withThresholds(
    baseColor = 'transparent',
    steps = []
  )
  .addOverrides([
    grafana.override.newColorOverride(
      name  = 'status_2xx',
      color = 'green'
    ),
    grafana.override.newColorOverride(
      name  = 'status_4xx',
      color = 'yellow'
    ),
    grafana.override.newColorOverride(
      name  = 'status_5xx',
      color = 'red'
    ),
  ]);

{
  new(ds, vars)::
    panels.timeseries(
      title       = 'Request-Response / s',
      datasource  = ds.prometheus,
    )
    .configure(_configuration)

    .addTarget(targets.prometheus(
      datasource    = ds.prometheus,
      legendFormat  = '{{status}} {{method}} {{endpoint}}',
      refId         = 'status_2xx',
      expr          = 'sum(rate(axum_http_requests_total{status=~"^2.*$"}[1m])) by (status, method, endpoint)',
      exemplar      = true,
    ))
    .addTarget(targets.prometheus(
      datasource    = ds.prometheus,
      legendFormat  = '{{status}} {{method}} {{endpoint}}',
      refId         = 'status_4xx',
      expr          = 'sum(rate(axum_http_requests_total{status=~"^4.*"}[1m])) by (status, method, endpoint)',
      exemplar      = true,
    ))
    .addTarget(targets.prometheus(
      datasource    = ds.prometheus,
      legendFormat  = '{{status}} {{method}} {{endpoint}}',
      refId         = 'status_5xx',
      expr          = 'sum(rate(axum_http_requests_total{status=~"^5.*"}[1m])) by (status, method, endpoint)',
      exemplar      = true,
    ))
    .addTarget(targets.prometheus(
      datasource    = ds.prometheus,
      legendFormat  = '{{status}} {{method}} {{endpoint}}',
      refId         = 'status_all',
      expr          = 'sum(rate(axum_http_requests_total{status!~"^(2|4|5).*"}[1m])) by (status, method, endpoint)',
      exemplar      = true,
    ))
}
