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
      name  = 'CPU_Avg',
      color = defaults.values.colors.cpu
    ),
    grafana.override.newColorOverride(
      name  = 'CPU_Max',
      color = defaults.values.colors.cpu_alt
    )
  ]);

{
  new(ds, vars)::
    panels.timeseries(
      title       = 'Redis CPU',
      datasource  = ds.cloudwatch,
    )
    .configure(_configuration)
    .setAlert(vars.environment, defaults.alerts.cpu(
      namespace     = vars.namespace,
      env           = vars.environment,
      title         = 'Redis',
      notifications = vars.notifications,
    ))

    .addTarget(targets.cloudwatch(
      alias       = 'CPU (Max)',
      datasource  = ds.cloudwatch,
      dimensions  = {
        CacheClusterId: vars.redis_cluster_id,
      },
      matchExact  = true,
      metricName  = 'CPUUtilization',
      namespace   = 'AWS/ElastiCache',
      statistic   = 'Maximum',
      refId       = 'CPU_Max',
    ))
    .addTarget(targets.cloudwatch(
      alias       = 'CPU (Avg)',
      datasource  = ds.cloudwatch,
      dimensions  = {
        CacheClusterId: vars.redis_cluster_id,
      },
      matchExact  = true,
      metricName  = 'CPUUtilization',
      namespace   = 'AWS/ElastiCache',
      statistic   = 'Average',
      refId       = 'CPU_Avg',
    ))
}
