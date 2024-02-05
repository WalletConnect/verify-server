local grafana   = import '../../grafonnet-lib/grafana.libsonnet';
local defaults  = import '../../grafonnet-lib/defaults.libsonnet';

local panels = grafana.panels;
local targets = grafana.targets;

local thresholds = {
  warning:  100,
};

local _configuration = defaults.configuration.timeseries
  .withUnit(grafana.fieldConfig.units.Milliseconds)
  .withThresholdStyle(grafana.fieldConfig.thresholdStyle.Area)
  .withThresholds(
    baseColor = defaults.values.colors.ok,
    steps = [
      { value: thresholds.warning, color: defaults.values.colors.critical },
    ]
  )
  .withSoftLimit(
    axisSoftMin = 0,
    axisSoftMax = thresholds.warning,
  )
  .addOverrides([
    grafana.override.newColorOverride(
      name  = 'read',
      color = 'green'
    ),
    grafana.override.newColorOverride(
      name  = 'write',
      color = 'blue'
    )
  ]);

{
  new(ds, vars)::
    panels.timeseries(
      title       = 'Max avg latency [1m]',
      datasource  = ds.prometheus,
    )
    .configure(_configuration)

    .addTarget(targets.prometheus(
      datasource    = ds.prometheus,
      legendFormat  = '{{method}}  {{endpoint}}',
      refId         = 'write',
      expr          = 'max(increase(axum_http_requests_duration_seconds_sum{method !~ "GET|HEAD"}[1m]) * 1000 / increase(axum_http_requests_duration_seconds_count{method !~ "GET|HEAD"}[1m])) by (method, endpoint)',
      exemplar      = true,
    ))
    .addTarget(targets.prometheus(
      datasource    = ds.prometheus,
      legendFormat  = '{{method}}  {{endpoint}}',
      refId         = 'read',
      expr          = 'max(increase(axum_http_requests_duration_seconds_sum{method =~ "GET|HEAD"}[1m]) * 1000 / increase(axum_http_requests_duration_seconds_count{method =~ "GET|HEAD"}[1m])) by (status, method, endpoint)',
      exemplar      = true,
    ))
}
