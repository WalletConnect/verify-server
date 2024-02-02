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
      { value: thresholds.warning,  color: defaults.values.colors.critical  },
    ]
  )
  .withSoftLimit(
    axisSoftMin = 0,
    axisSoftMax = thresholds.warning,
  )
  .addOverrides([
    grafana.override.newColorOverride(
      name = 'total_errors',
      color = 'dark-red'
    ),
    grafana.override.newColorOverride(
      name = 'total',
      color = 'dark-green'
    ),
    grafana.override.newColorOverride(
      name = 'errors_per_database',
      color = 'red'
    ),
    grafana.override.newColorOverride(
      name = 'per_database',
      color = 'green'
    ),
  ]);

{
  new(ds, vars)::
    panels.timeseries(
      title       = 'Redis Writes / sec',
      datasource  = ds.prometheus,
    )
    .configure(_configuration)

    .addTarget(targets.prometheus(
      datasource    = ds.prometheus,
      legendFormat  ='total errors',
      refId         = 'total_errors',
      expr          = 'sum(rate(redis_write_errors{}[1m])) or vector(0)',
      exemplar      = true,
    ))
    .addTarget(targets.prometheus(
      datasource    = ds.prometheus,
      legendFormat  ='total',
      refId         = 'total',
      expr          = 'sum(rate(redis_writes{}[1m]))',
      exemplar      = true,
    ))
    .addTarget(targets.prometheus(
      datasource    = ds.prometheus,
      legendFormat  ='{{db}} errors',
      refId         = 'errors_per_database',
      expr          = 'sum(rate(redis_write_errors{}[1m]) or vector(0)) by (db)',
      exemplar      = true,
    ))
    .addTarget(targets.prometheus(
      datasource    = ds.prometheus,
      legendFormat  ='{{db}}',
      refId         = 'per_database',
      expr          = 'sum(rate(redis_writes{}[1m])) by (db)',
      exemplar      = true,
    ))
}
