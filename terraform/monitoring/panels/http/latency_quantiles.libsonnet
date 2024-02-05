local grafana   = import '../../grafonnet-lib/grafana.libsonnet';
local defaults  = import '../../grafonnet-lib/defaults.libsonnet';

local panels = grafana.panels;
local targets = grafana.targets;

local thresholds = {
  warning:  80,
};

local _configuration = defaults.configuration.timeseries
  .withUnit(grafana.fieldConfig.units.Milliseconds)
  .withThresholdStyle(grafana.fieldConfig.thresholdStyle.Area)
  .withThresholds(
    baseColor = defaults.values.colors.ok,
    steps = [
      { value: thresholds.warning, color: defaults.values.colors.critical },
    ]
  );

{
  new(ds, vars)::
    panels.barGauge(
      title       = 'Latency quantiles',
      datasource  = ds.prometheus,
    )

    .addTarget(targets.prometheus(
      datasource    = ds.prometheus,
      legendFormat  = '0.5',
      refId         = 'A',
      expr          = 'histogram_quantile(0.5, sum by (le) (rate(axum_http_requests_duration_seconds_bucket[$__range]))) * 1000',
      exemplar      = true,
    ))
    .addTarget(targets.prometheus(
      datasource    = ds.prometheus,
      legendFormat  = '0.9',
      refId         = 'B',
      expr          = 'histogram_quantile(0.9, sum by (le) (rate(axum_http_requests_duration_seconds_bucket[$__range]))) * 1000',
      exemplar      = true,
    ))
    .addTarget(targets.prometheus(
      datasource    = ds.prometheus,
      legendFormat  = '0.99',
      refId         = 'C',
      expr          = 'histogram_quantile(0.99, sum by (le) (rate(axum_http_requests_duration_seconds_bucket[$__range]))) * 1000',
      exemplar      = true,
    ))
    .addTarget(targets.prometheus(
      datasource    = ds.prometheus,
      legendFormat  = '0.999',
      refId         = 'D',
      expr          = 'histogram_quantile(0.999, sum by (le) (rate(axum_http_requests_duration_seconds_bucket[$__range]))) * 1000',
      exemplar      = true,
    )) + {
      fieldConfig+: _configuration.fieldConfig,
      options+:     _configuration.options,
    }
}
