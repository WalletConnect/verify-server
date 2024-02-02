local grafana   = import '../../grafonnet-lib/grafana.libsonnet';
local defaults  = import '../../grafonnet-lib/defaults.libsonnet';

local panels = grafana.panels;
local targets = grafana.targets;

local thresholds = {
  warning:  80,
};

local _configuration = defaults.configuration.timeseries
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
      name  = 'errors',
      color = 'red'
    ),
    grafana.override.newColorOverride(
      name  = 'writes',
      color = 'green'
    ),
  ]);

{
  new(ds, vars)::
    panels.timeseries(
      title       = 'Cache Writes / s',
      datasource  = ds.prometheus,
    )
    .configure(_configuration)

    .addTarget(targets.prometheus(
      datasource    = ds.prometheus,
      legendFormat  = 'errors',
      refId         = 'errors',
      expr          = 'rate(project_registry_cache_write_errors[1m]) or vector(0)',
      exemplar      = true,
    ))
    .addTarget(targets.prometheus(
      datasource    = ds.prometheus,
      legendFormat  = 'writes',
      refId         = 'writes',
      expr          = 'sum(rate(project_registry_cache_writes[1m]))',
      exemplar      = true,
    ))
}
