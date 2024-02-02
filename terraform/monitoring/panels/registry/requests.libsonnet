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
  .addOverrides([
    grafana.override.newColorOverride(
      name  = 'errors',
      color = 'red'
    ),
    grafana.override.newColorOverride(
      name  = 'fixed',
      color = 'green'
    ),
  ]);

{
  new(ds, vars)::
    panels.timeseries(
      title       = 'Requests / s',
      datasource  = ds.prometheus,
    )
    .configure(_configuration)

    .addTarget(targets.prometheus(
      datasource    = ds.prometheus,
      legendFormat  = 'errors',
      refId         = 'errors',
      expr          = 'sum(rate(project_registry_errors[1m]))',
      exemplar      = true,
    ))
    .addTarget(targets.prometheus(
      datasource    = ds.prometheus,
      legendFormat  = 'total',
      refId         = 'total',
      expr          = 'sum(rate(project_registry_requests[1m]))',
      exemplar      = true,
    ))
}
