local grafana   = import '../../grafonnet-lib/grafana.libsonnet';
local defaults  = import '../../grafonnet-lib/defaults.libsonnet';

local panels = grafana.panels;
local targets = grafana.targets;

local _configuration = defaults.configuration.timeseries
  .addOverrides([
    grafana.override.newColorOverride(
      name  = 'status_2XX',
      color = 'green'
    ),
    grafana.override.newColorOverride(
      name  = 'status_4XX',
      color = 'orange'
    ),
    grafana.override.newColorOverride(
      name  = 'status_5XX',
      color = 'red'
    ),
    grafana.override.newColorOverride(
      name  = 'status_other',
      color = 'blue'
    ),
  ]);

{
  new(ds, vars)::
    panels.pieChart(
      title       = 'Response Status Codes',
      datasource  = ds.prometheus,
    )

    .addTarget(targets.prometheus(
      datasource    = ds.prometheus,
      legendFormat  ='2XX',
      refId         = 'status_2XX',
      expr          = 'sum(increase(axum_http_requests_total { status =~ "^2.*"}[$__range]))',
      exemplar      = true,
    ))
    .addTarget(targets.prometheus(
      datasource    = ds.prometheus,
      legendFormat  ='4XX',
      refId         = 'status_4XX',
      expr          = 'sum(increase(axum_http_requests_total { status =~ "^4.*"}[$__range]))',
      exemplar      = true,
    ))
    .addTarget(targets.prometheus(
      datasource    = ds.prometheus,
      legendFormat  ='5XX',
      refId         = 'status_5XX',
      expr          = 'sum(increase(axum_http_requests_total { status =~ "^5.*"}[$__range]))',
      exemplar      = true,
    ))
    .addTarget(targets.prometheus(
      datasource    = ds.prometheus,
      legendFormat  ='Other',
      refId         = 'status_other',
      expr          = 'sum(increase(axum_http_requests_total { status !~ "^(2|4|5).*"}[$__range]))',
      exemplar      = true,
    )) + {
      fieldConfig+: _configuration.fieldConfig,
      options+:     _configuration.options,
    }
}
