local grafana   = import '../../grafonnet-lib/grafana.libsonnet';
local defaults  = import '../../grafonnet-lib/defaults.libsonnet';

local panels = grafana.panels;
local targets = grafana.targets;

local thresholds = {
  warning:  defaults.values.resource.thresholds.warning,
  critical: defaults.values.resource.thresholds.critical,
};

local _configuration = defaults.configuration.timeseries
  .withUnit('percent')
  .withThresholdStyle(grafana.fieldConfig.thresholdStyle.Area)
  .withThresholds(
    baseColor = defaults.values.colors.ok,
    steps = [
      { value: thresholds.warning,  color: defaults.values.colors.warn      },
      { value: thresholds.critical, color: defaults.values.colors.critical  },
    ]
  )
  .withSoftLimit(
    axisSoftMin = 0,
    axisSoftMax = thresholds.warning,
  )
  .addOverrides([
    grafana.override.newColorOverride(
      name = 'Mem_Avg',
      color = defaults.values.colors.memory
    ),
    grafana.override.newColorOverride(
      name = 'Mem_Max',
      color = defaults.values.colors.memory_alt
    )
  ]);

{
  new(ds, vars)::
    panels.timeseries(
      title       = 'Redis Memory',
      datasource  = ds.cloudwatch,
    )
    .configure(_configuration)
    .setAlert(vars.environment, defaults.alerts.memory(
      namespace     = vars.namespace,
      env           = vars.environment,
      title         = 'Redis',
      notifications = vars.notifications,
    ))

    .addTarget(targets.cloudwatch(
      alias       = 'Memory (Max)',
      datasource  = ds.cloudwatch,
      dimensions  = {
        CacheClusterId: vars.redis_cluster_id,
      },
      matchExact  = true,
      metricName  = 'DatabaseMemoryUsagePercentage',
      namespace   = 'AWS/ElastiCache',
      statistic   = 'Maximum',
      refId       = 'Mem_Max',
    ))
    .addTarget(targets.cloudwatch(
      alias       = 'Memory (Avg)',
      datasource  = ds.cloudwatch,
      dimensions  = {
        CacheClusterId: vars.redis_cluster_id,
      },
      matchExact  = true,
      metricName  = 'DatabaseMemoryUsagePercentage',
      namespace   = 'AWS/ElastiCache',
      statistic   = 'Average',
      refId       = 'Mem_Avg',
    ))
}
