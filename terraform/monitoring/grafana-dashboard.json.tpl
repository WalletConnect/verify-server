{
  "annotations": {
    "list": [
      {
        "builtIn": 1,
        "datasource": "-- Grafana --",
        "enable": true,
        "hide": true,
        "iconColor": "rgba(0, 211, 255, 1)",
        "name": "Annotations & Alerts",
        "target": {
          "limit": 100,
          "matchAny": false,
          "tags": [],
          "type": "dashboard"
        },
        "type": "dashboard"
      }
    ]
  },
  "editable": true,
  "fiscalYearStartMonth": 0,
  "graphTooltip": 0,
  "id": 44,
  "links": [],
  "liveNow": false,
  "panels": [
    {
      "collapsed": false,
      "gridPos": {
        "h": 1,
        "w": 24,
        "x": 0,
        "y": 0
      },
      "id": 16,
      "panels": [],
      "title": "HTTP Server",
      "type": "row"
    },
    {
      "datasource": {
        "type": "prometheus",
        "uid": "dAm_6PT4z"
      },
      "fieldConfig": {
        "defaults": {
          "color": {
            "mode": "palette-classic"
          },
          "custom": {
            "hideFrom": {
              "legend": false,
              "tooltip": false,
              "viz": false
            }
          },
          "mappings": []
        },
        "overrides": [
          {
            "matcher": {
              "id": "byName",
              "options": "5XX"
            },
            "properties": [
              {
                "id": "color",
                "value": {
                  "fixedColor": "red",
                  "mode": "fixed"
                }
              }
            ]
          },
          {
            "matcher": {
              "id": "byName",
              "options": "Other"
            },
            "properties": [
              {
                "id": "color",
                "value": {
                  "fixedColor": "orange",
                  "mode": "fixed"
                }
              }
            ]
          }
        ]
      },
      "gridPos": {
        "h": 8,
        "w": 8,
        "x": 0,
        "y": 1
      },
      "id": 25,
      "options": {
        "legend": {
          "displayMode": "list",
          "placement": "bottom"
        },
        "pieType": "pie",
        "reduceOptions": {
          "calcs": [
            "lastNotNull"
          ],
          "fields": "",
          "values": false
        },
        "tooltip": {
          "mode": "single",
          "sort": "none"
        }
      },
      "targets": [
        {
          "datasource": {
            "type": "prometheus",
            "uid": "dAm_6PT4z"
          },
          "exemplar": true,
          "expr": "sum(increase(axum_http_requests_total { status =~ \"^2.*\"}[$__range]))",
          "format": "time_series",
          "hide": false,
          "interval": "",
          "legendFormat": "2XX",
          "refId": "A"
        },
        {
          "datasource": {
            "type": "prometheus",
            "uid": "dAm_6PT4z"
          },
          "exemplar": true,
          "expr": "sum(increase(axum_http_requests_total { status =~ \"^4.*\"}[$__range]))",
          "format": "time_series",
          "hide": false,
          "interval": "",
          "legendFormat": "4XX",
          "refId": "B"
        },
        {
          "datasource": {
            "type": "prometheus",
            "uid": "dAm_6PT4z"
          },
          "exemplar": true,
          "expr": "sum(increase(axum_http_requests_total { status =~ \"^5.*\"}[$__range]))",
          "format": "time_series",
          "hide": false,
          "interval": "",
          "legendFormat": "5XX",
          "refId": "C"
        },
        {
          "datasource": {
            "type": "prometheus",
            "uid": "dAm_6PT4z"
          },
          "exemplar": true,
          "expr": "sum(increase(axum_http_requests_total { status !~ \"^(2|4|5).*\"}[$__range]))",
          "format": "time_series",
          "hide": false,
          "interval": "",
          "legendFormat": "Other",
          "refId": "D"
        }
      ],
      "title": "Response Status Codes ",
      "type": "piechart"
    },
    {
      "datasource": {
        "type": "prometheus",
        "uid": "dAm_6PT4z"
      },
      "description": "",
      "fieldConfig": {
        "defaults": {
          "color": {
            "mode": "palette-classic"
          },
          "custom": {
            "axisLabel": "",
            "axisPlacement": "auto",
            "barAlignment": 0,
            "drawStyle": "line",
            "fillOpacity": 0,
            "gradientMode": "none",
            "hideFrom": {
              "legend": false,
              "tooltip": false,
              "viz": false
            },
            "lineInterpolation": "linear",
            "lineWidth": 1,
            "pointSize": 5,
            "scaleDistribution": {
              "type": "linear"
            },
            "showPoints": "auto",
            "spanNulls": false,
            "stacking": {
              "group": "A",
              "mode": "none"
            },
            "thresholdsStyle": {
              "mode": "off"
            }
          },
          "mappings": [],
          "thresholds": {
            "mode": "absolute",
            "steps": [
              {
                "color": "green",
                "value": null
              },
              {
                "color": "red",
                "value": 80
              }
            ]
          }
        },
        "overrides": [
          {
            "matcher": {
              "id": "byFrameRefID",
              "options": "A"
            },
            "properties": [
              {
                "id": "color",
                "value": {
                  "fixedColor": "green",
                  "mode": "fixed"
                }
              }
            ]
          },
          {
            "matcher": {
              "id": "byFrameRefID",
              "options": "B"
            },
            "properties": [
              {
                "id": "color",
                "value": {
                  "fixedColor": "yellow",
                  "mode": "fixed"
                }
              }
            ]
          },
          {
            "matcher": {
              "id": "byFrameRefID",
              "options": "C"
            },
            "properties": [
              {
                "id": "color",
                "value": {
                  "fixedColor": "red",
                  "mode": "fixed"
                }
              }
            ]
          }
        ]
      },
      "gridPos": {
        "h": 8,
        "w": 16,
        "x": 8,
        "y": 1
      },
      "id": 18,
      "options": {
        "legend": {
          "calcs": [],
          "displayMode": "list",
          "placement": "right"
        },
        "tooltip": {
          "mode": "single",
          "sort": "none"
        }
      },
      "targets": [
        {
          "datasource": {
            "type": "prometheus",
            "uid": "dAm_6PT4z"
          },
          "exemplar": true,
          "expr": "sum(rate(axum_http_requests_total{status=~\"^5.*\"}[1m])) by (status, method, endpoint)",
          "hide": false,
          "instant": false,
          "interval": "",
          "intervalFactor": 1,
          "legendFormat": "{{status}} {{method}} {{endpoint}}",
          "refId": "C"
        },
        {
          "datasource": {
            "type": "prometheus",
            "uid": "dAm_6PT4z"
          },
          "exemplar": true,
          "expr": "sum(rate(axum_http_requests_total{status=~\"^4.*\"}[1m])) by (status, method, endpoint)",
          "hide": false,
          "instant": false,
          "interval": "",
          "intervalFactor": 1,
          "legendFormat": "{{status}} {{method}} {{endpoint}}",
          "refId": "B"
        },
        {
          "datasource": {
            "type": "prometheus",
            "uid": "dAm_6PT4z"
          },
          "exemplar": true,
          "expr": "sum(rate(axum_http_requests_total{status!~\"^(2|4|5).*\"}[1m])) by (status, method, endpoint)",
          "hide": false,
          "instant": false,
          "interval": "",
          "intervalFactor": 1,
          "legendFormat": "{{status}} {{method}} {{endpoint}}",
          "refId": "D"
        },
        {
          "datasource": {
            "type": "prometheus",
            "uid": "dAm_6PT4z"
          },
          "exemplar": true,
          "expr": "sum(rate(axum_http_requests_total{status=~\"^2.*$\"}[1m])) by (status, method, endpoint)",
          "instant": false,
          "interval": "",
          "intervalFactor": 1,
          "legendFormat": "{{status}} {{method}} {{endpoint}}",
          "refId": "A"
        }
      ],
      "title": "Request-Response / s",
      "type": "timeseries"
    },
    {
      "datasource": {
        "type": "prometheus",
        "uid": "dAm_6PT4z"
      },
      "fieldConfig": {
        "defaults": {
          "color": {
            "mode": "thresholds"
          },
          "mappings": [],
          "thresholds": {
            "mode": "absolute",
            "steps": [
              {
                "color": "green",
                "value": null
              },
              {
                "color": "red",
                "value": 80
              }
            ]
          },
          "unit": "ms"
        },
        "overrides": []
      },
      "gridPos": {
        "h": 8,
        "w": 8,
        "x": 0,
        "y": 9
      },
      "id": 23,
      "options": {
        "displayMode": "lcd",
        "orientation": "auto",
        "reduceOptions": {
          "calcs": [
            "lastNotNull"
          ],
          "fields": "",
          "values": false
        },
        "showUnfilled": true
      },
      "pluginVersion": "8.4.7",
      "targets": [
        {
          "datasource": {
            "type": "prometheus",
            "uid": "dAm_6PT4z"
          },
          "exemplar": true,
          "expr": "histogram_quantile(0.5, sum by (le) (rate(axum_http_requests_duration_seconds_bucket[$__range]))) * 1000",
          "format": "time_series",
          "hide": false,
          "interval": "",
          "legendFormat": "0.5",
          "refId": "B"
        },
        {
          "datasource": {
            "type": "prometheus",
            "uid": "dAm_6PT4z"
          },
          "exemplar": true,
          "expr": "histogram_quantile(0.9, sum by (le) (rate(axum_http_requests_duration_seconds_bucket[$__range]))) * 1000",
          "format": "time_series",
          "hide": false,
          "interval": "",
          "legendFormat": "0.9",
          "refId": "C"
        },
        {
          "datasource": {
            "type": "prometheus",
            "uid": "dAm_6PT4z"
          },
          "exemplar": true,
          "expr": "histogram_quantile(0.99, sum by (le) (rate(axum_http_requests_duration_seconds_bucket[$__range]))) * 1000",
          "format": "time_series",
          "hide": false,
          "interval": "",
          "legendFormat": "0.99",
          "refId": "D"
        },
        {
          "datasource": {
            "type": "prometheus",
            "uid": "dAm_6PT4z"
          },
          "exemplar": true,
          "expr": "histogram_quantile(0.999, sum by (le) (rate(axum_http_requests_duration_seconds_bucket[$__range]))) * 1000",
          "format": "time_series",
          "hide": false,
          "interval": "",
          "legendFormat": "0.999",
          "refId": "E"
        }
      ],
      "title": "Latency quantiles",
      "type": "bargauge"
    },
    {
      "datasource": {
        "type": "prometheus",
        "uid": "dAm_6PT4z"
      },
      "description": "",
      "fieldConfig": {
        "defaults": {
          "color": {
            "mode": "palette-classic"
          },
          "custom": {
            "axisLabel": "",
            "axisPlacement": "auto",
            "barAlignment": 0,
            "drawStyle": "line",
            "fillOpacity": 0,
            "gradientMode": "none",
            "hideFrom": {
              "legend": false,
              "tooltip": false,
              "viz": false
            },
            "lineInterpolation": "linear",
            "lineStyle": {
              "fill": "solid"
            },
            "lineWidth": 1,
            "pointSize": 5,
            "scaleDistribution": {
              "type": "linear"
            },
            "showPoints": "auto",
            "spanNulls": false,
            "stacking": {
              "group": "A",
              "mode": "none"
            },
            "thresholdsStyle": {
              "mode": "line"
            }
          },
          "mappings": [],
          "thresholds": {
            "mode": "absolute",
            "steps": [
              {
                "color": "green",
                "value": null
              },
              {
                "color": "red",
                "value": 100
              }
            ]
          },
          "unit": "ms"
        },
        "overrides": [
          {
            "matcher": {
              "id": "byFrameRefID",
              "options": "read"
            },
            "properties": [
              {
                "id": "color",
                "value": {
                  "fixedColor": "green",
                  "mode": "fixed"
                }
              }
            ]
          },
          {
            "matcher": {
              "id": "byFrameRefID",
              "options": "write"
            },
            "properties": [
              {
                "id": "color",
                "value": {
                  "fixedColor": "blue",
                  "mode": "fixed"
                }
              }
            ]
          }
        ]
      },
      "gridPos": {
        "h": 8,
        "w": 16,
        "x": 8,
        "y": 9
      },
      "id": 21,
      "options": {
        "legend": {
          "calcs": [],
          "displayMode": "list",
          "placement": "right"
        },
        "tooltip": {
          "mode": "single",
          "sort": "none"
        }
      },
      "pluginVersion": "8.4.7",
      "targets": [
        {
          "datasource": {
            "type": "prometheus",
            "uid": "dAm_6PT4z"
          },
          "exemplar": true,
          "expr": "max(increase(axum_http_requests_duration_seconds_sum{method !~ \"GET|HEAD\"}[1m]) * 1000 / increase(axum_http_requests_duration_seconds_count{method !~ \"GET|HEAD\"}[1m])) by (method, endpoint)",
          "format": "time_series",
          "hide": false,
          "instant": false,
          "interval": "",
          "legendFormat": "{{method}}  {{endpoint}}",
          "refId": "write"
        },
        {
          "datasource": {
            "type": "prometheus",
            "uid": "dAm_6PT4z"
          },
          "exemplar": true,
          "expr": "max(increase(axum_http_requests_duration_seconds_sum{method =~ \"GET|HEAD\"}[1m]) * 1000 / increase(axum_http_requests_duration_seconds_count{method =~ \"GET|HEAD\"}[1m])) by (status, method, endpoint)",
          "format": "time_series",
          "hide": false,
          "interval": "",
          "legendFormat": "{{method}}  {{endpoint}}",
          "refId": "read"
        }
      ],
      "title": "Max avg latency [1m]",
      "type": "timeseries"
    },
    {
      "collapsed": false,
      "gridPos": {
        "h": 1,
        "w": 24,
        "x": 0,
        "y": 17
      },
      "id": 8,
      "panels": [],
      "title": "Project Registry",
      "type": "row"
    },
    {
      "datasource": {
        "type": "prometheus",
        "uid": "dAm_6PT4z"
      },
      "fieldConfig": {
        "defaults": {
          "color": {
            "mode": "palette-classic"
          },
          "custom": {
            "axisLabel": "",
            "axisPlacement": "auto",
            "barAlignment": 0,
            "drawStyle": "line",
            "fillOpacity": 0,
            "gradientMode": "none",
            "hideFrom": {
              "legend": false,
              "tooltip": false,
              "viz": false
            },
            "lineInterpolation": "linear",
            "lineWidth": 1,
            "pointSize": 5,
            "scaleDistribution": {
              "type": "linear"
            },
            "showPoints": "auto",
            "spanNulls": false,
            "stacking": {
              "group": "A",
              "mode": "none"
            },
            "thresholdsStyle": {
              "mode": "off"
            }
          },
          "mappings": [],
          "thresholds": {
            "mode": "absolute",
            "steps": [
              {
                "color": "green",
                "value": null
              },
              {
                "color": "red",
                "value": 80
              }
            ]
          }
        },
        "overrides": [
          {
            "matcher": {
              "id": "byName",
              "options": "errors"
            },
            "properties": [
              {
                "id": "color",
                "value": {
                  "fixedColor": "red",
                  "mode": "fixed"
                }
              }
            ]
          },
          {
            "matcher": {
              "id": "byName",
              "options": "total"
            },
            "properties": [
              {
                "id": "color",
                "value": {
                  "fixedColor": "green",
                  "mode": "fixed"
                }
              }
            ]
          }
        ]
      },
      "gridPos": {
        "h": 6,
        "w": 8,
        "x": 0,
        "y": 18
      },
      "id": 10,
      "options": {
        "legend": {
          "calcs": [],
          "displayMode": "list",
          "placement": "bottom"
        },
        "tooltip": {
          "mode": "single",
          "sort": "none"
        }
      },
      "targets": [
        {
          "datasource": {
            "type": "prometheus",
            "uid": "dAm_6PT4z"
          },
          "exemplar": true,
          "expr": "sum(rate(project_registry_errors[1m]))",
          "hide": false,
          "interval": "",
          "legendFormat": "errors",
          "refId": "B"
        },
        {
          "datasource": {
            "type": "prometheus",
            "uid": "dAm_6PT4z"
          },
          "exemplar": true,
          "expr": "sum(rate(project_registry_requests[1m]))",
          "interval": "",
          "legendFormat": "total",
          "refId": "A"
        }
      ],
      "title": "Requests / s",
      "type": "timeseries"
    },
    {
      "datasource": {
        "type": "prometheus",
        "uid": "dAm_6PT4z"
      },
      "fieldConfig": {
        "defaults": {
          "color": {
            "mode": "palette-classic"
          },
          "custom": {
            "axisLabel": "",
            "axisPlacement": "auto",
            "barAlignment": 0,
            "drawStyle": "line",
            "fillOpacity": 0,
            "gradientMode": "none",
            "hideFrom": {
              "legend": false,
              "tooltip": false,
              "viz": false
            },
            "lineInterpolation": "linear",
            "lineWidth": 1,
            "pointSize": 5,
            "scaleDistribution": {
              "type": "linear"
            },
            "showPoints": "auto",
            "spanNulls": false,
            "stacking": {
              "group": "A",
              "mode": "none"
            },
            "thresholdsStyle": {
              "mode": "off"
            }
          },
          "mappings": [],
          "thresholds": {
            "mode": "absolute",
            "steps": [
              {
                "color": "green",
                "value": null
              },
              {
                "color": "red",
                "value": 80
              }
            ]
          }
        },
        "overrides": [
          {
            "matcher": {
              "id": "byName",
              "options": "errors"
            },
            "properties": [
              {
                "id": "color",
                "value": {
                  "fixedColor": "red",
                  "mode": "fixed"
                }
              }
            ]
          },
          {
            "matcher": {
              "id": "byName",
              "options": "hits"
            },
            "properties": [
              {
                "id": "color",
                "value": {
                  "fixedColor": "green",
                  "mode": "fixed"
                }
              }
            ]
          }
        ]
      },
      "gridPos": {
        "h": 6,
        "w": 8,
        "x": 8,
        "y": 18
      },
      "id": 12,
      "options": {
        "legend": {
          "calcs": [],
          "displayMode": "list",
          "placement": "bottom"
        },
        "tooltip": {
          "mode": "single",
          "sort": "none"
        }
      },
      "targets": [
        {
          "datasource": {
            "type": "prometheus",
            "uid": "dAm_6PT4z"
          },
          "exemplar": true,
          "expr": "rate(project_registry_cache_errors[1m]) or vector(0)",
          "hide": false,
          "instant": false,
          "interval": "",
          "legendFormat": "errors",
          "refId": "C"
        },
        {
          "datasource": {
            "type": "prometheus",
            "uid": "dAm_6PT4z"
          },
          "exemplar": true,
          "expr": "sum(rate(project_registry_cache_misses[1m]))",
          "hide": false,
          "interval": "",
          "legendFormat": "misses",
          "refId": "B"
        },
        {
          "datasource": {
            "type": "prometheus",
            "uid": "dAm_6PT4z"
          },
          "exemplar": true,
          "expr": "sum(rate(project_registry_cache_hits[1m]))",
          "interval": "",
          "legendFormat": "hits",
          "refId": "A"
        }
      ],
      "title": "Cache reads / s",
      "type": "timeseries"
    },
    {
      "datasource": {
        "type": "prometheus",
        "uid": "dAm_6PT4z"
      },
      "fieldConfig": {
        "defaults": {
          "color": {
            "mode": "palette-classic"
          },
          "custom": {
            "axisLabel": "",
            "axisPlacement": "auto",
            "barAlignment": 0,
            "drawStyle": "line",
            "fillOpacity": 0,
            "gradientMode": "none",
            "hideFrom": {
              "legend": false,
              "tooltip": false,
              "viz": false
            },
            "lineInterpolation": "linear",
            "lineWidth": 1,
            "pointSize": 5,
            "scaleDistribution": {
              "type": "linear"
            },
            "showPoints": "auto",
            "spanNulls": false,
            "stacking": {
              "group": "A",
              "mode": "none"
            },
            "thresholdsStyle": {
              "mode": "off"
            }
          },
          "mappings": [],
          "thresholds": {
            "mode": "absolute",
            "steps": [
              {
                "color": "green",
                "value": null
              },
              {
                "color": "red",
                "value": 80
              }
            ]
          }
        },
        "overrides": [
          {
            "matcher": {
              "id": "byName",
              "options": "errors"
            },
            "properties": [
              {
                "id": "color",
                "value": {
                  "fixedColor": "red",
                  "mode": "fixed"
                }
              }
            ]
          },
          {
            "matcher": {
              "id": "byName",
              "options": "writes"
            },
            "properties": [
              {
                "id": "color",
                "value": {
                  "fixedColor": "green",
                  "mode": "fixed"
                }
              }
            ]
          }
        ]
      },
      "gridPos": {
        "h": 6,
        "w": 8,
        "x": 16,
        "y": 18
      },
      "id": 13,
      "options": {
        "legend": {
          "calcs": [],
          "displayMode": "list",
          "placement": "bottom"
        },
        "tooltip": {
          "mode": "single",
          "sort": "none"
        }
      },
      "targets": [
        {
          "datasource": {
            "type": "prometheus",
            "uid": "dAm_6PT4z"
          },
          "exemplar": true,
          "expr": "rate(project_registry_cache_write_errors[1m]) or vector(0)",
          "hide": false,
          "instant": false,
          "interval": "",
          "legendFormat": "errors",
          "refId": "C"
        },
        {
          "datasource": {
            "type": "prometheus",
            "uid": "dAm_6PT4z"
          },
          "exemplar": true,
          "expr": "sum(rate(project_registry_cache_writes[1m]))",
          "interval": "",
          "legendFormat": "writes",
          "refId": "A"
        }
      ],
      "title": "Cache writes / s",
      "type": "timeseries"
    },
    {
      "collapsed": false,
      "gridPos": {
        "h": 1,
        "w": 24,
        "x": 0,
        "y": 24
      },
      "id": 5,
      "panels": [],
      "title": "Redis",
      "type": "row"
    },
    {
      "datasource": {
        "type": "prometheus",
        "uid": "dAm_6PT4z"
      },
      "fieldConfig": {
        "defaults": {
          "color": {
            "mode": "palette-classic"
          },
          "custom": {
            "axisLabel": "",
            "axisPlacement": "auto",
            "barAlignment": 0,
            "drawStyle": "line",
            "fillOpacity": 0,
            "gradientMode": "none",
            "hideFrom": {
              "legend": false,
              "tooltip": false,
              "viz": false
            },
            "lineInterpolation": "linear",
            "lineWidth": 1,
            "pointSize": 5,
            "scaleDistribution": {
              "type": "linear"
            },
            "showPoints": "auto",
            "spanNulls": false,
            "stacking": {
              "group": "A",
              "mode": "none"
            },
            "thresholdsStyle": {
              "mode": "off"
            }
          },
          "mappings": [],
          "thresholds": {
            "mode": "absolute",
            "steps": [
              {
                "color": "green",
                "value": null
              },
              {
                "color": "red",
                "value": 80
              }
            ]
          }
        },
        "overrides": [
          {
            "matcher": {
              "id": "byName",
              "options": "total"
            },
            "properties": [
              {
                "id": "color",
                "value": {
                  "fixedColor": "dark-green",
                  "mode": "fixed"
                }
              }
            ]
          },
          {
            "matcher": {
              "id": "byName",
              "options": "total errors"
            },
            "properties": [
              {
                "id": "color",
                "value": {
                  "fixedColor": "dark-red",
                  "mode": "fixed"
                }
              }
            ]
          },
          {
            "matcher": {
              "id": "byFrameRefID",
              "options": "per_database"
            },
            "properties": [
              {
                "id": "color",
                "value": {
                  "fixedColor": "green",
                  "mode": "fixed"
                }
              }
            ]
          },
          {
            "matcher": {
              "id": "byFrameRefID",
              "options": "errors_per_database"
            },
            "properties": [
              {
                "id": "color",
                "value": {
                  "fixedColor": "red",
                  "mode": "fixed"
                }
              }
            ]
          }
        ]
      },
      "gridPos": {
        "h": 7,
        "w": 12,
        "x": 0,
        "y": 25
      },
      "id": 2,
      "options": {
        "legend": {
          "calcs": [],
          "displayMode": "list",
          "placement": "bottom"
        },
        "tooltip": {
          "mode": "single",
          "sort": "none"
        }
      },
      "targets": [
        {
          "datasource": {
            "type": "prometheus",
            "uid": "dAm_6PT4z"
          },
          "exemplar": true,
          "expr": "sum(rate(redis_read_errors{}[1m])) or vector(0)",
          "hide": false,
          "interval": "",
          "legendFormat": "total errors",
          "refId": "total_errors"
        },
        {
          "datasource": {
            "type": "prometheus",
            "uid": "dAm_6PT4z"
          },
          "exemplar": true,
          "expr": "sum(rate(redis_reads{}[1m]))",
          "hide": false,
          "interval": "",
          "legendFormat": "total",
          "refId": "total"
        },
        {
          "datasource": {
            "type": "prometheus",
            "uid": "dAm_6PT4z"
          },
          "exemplar": true,
          "expr": "sum(rate(redis_read_errors{}[1m]) or vector(0)) by (db)",
          "hide": false,
          "interval": "",
          "legendFormat": "{{db}} errors",
          "refId": "errors_per_database"
        },
        {
          "datasource": {
            "type": "prometheus",
            "uid": "dAm_6PT4z"
          },
          "exemplar": true,
          "expr": "sum(rate(redis_reads{}[1m])) by (db)",
          "interval": "",
          "legendFormat": "{{db}}",
          "refId": "per_database"
        }
      ],
      "title": "Reads / s",
      "type": "timeseries"
    },
    {
      "datasource": {
        "type": "prometheus",
        "uid": "dAm_6PT4z"
      },
      "fieldConfig": {
        "defaults": {
          "color": {
            "mode": "palette-classic"
          },
          "custom": {
            "axisLabel": "",
            "axisPlacement": "auto",
            "barAlignment": 0,
            "drawStyle": "line",
            "fillOpacity": 0,
            "gradientMode": "none",
            "hideFrom": {
              "legend": false,
              "tooltip": false,
              "viz": false
            },
            "lineInterpolation": "linear",
            "lineWidth": 1,
            "pointSize": 5,
            "scaleDistribution": {
              "type": "linear"
            },
            "showPoints": "auto",
            "spanNulls": false,
            "stacking": {
              "group": "A",
              "mode": "none"
            },
            "thresholdsStyle": {
              "mode": "off"
            }
          },
          "mappings": [],
          "thresholds": {
            "mode": "absolute",
            "steps": [
              {
                "color": "green",
                "value": null
              },
              {
                "color": "red",
                "value": 80
              }
            ]
          }
        },
        "overrides": [
          {
            "matcher": {
              "id": "byName",
              "options": "total"
            },
            "properties": [
              {
                "id": "color",
                "value": {
                  "fixedColor": "dark-green",
                  "mode": "fixed"
                }
              }
            ]
          },
          {
            "matcher": {
              "id": "byName",
              "options": "total errors"
            },
            "properties": [
              {
                "id": "color",
                "value": {
                  "fixedColor": "dark-red",
                  "mode": "fixed"
                }
              }
            ]
          },
          {
            "matcher": {
              "id": "byFrameRefID",
              "options": "per_database"
            },
            "properties": [
              {
                "id": "color",
                "value": {
                  "fixedColor": "green",
                  "mode": "fixed"
                }
              }
            ]
          },
          {
            "matcher": {
              "id": "byFrameRefID",
              "options": "errors_per_database"
            },
            "properties": [
              {
                "id": "color",
                "value": {
                  "fixedColor": "red",
                  "mode": "fixed"
                }
              }
            ]
          }
        ]
      },
      "gridPos": {
        "h": 7,
        "w": 12,
        "x": 12,
        "y": 25
      },
      "id": 19,
      "options": {
        "legend": {
          "calcs": [],
          "displayMode": "list",
          "placement": "bottom"
        },
        "tooltip": {
          "mode": "single",
          "sort": "none"
        }
      },
      "targets": [
        {
          "datasource": {
            "type": "prometheus",
            "uid": "dAm_6PT4z"
          },
          "exemplar": true,
          "expr": "sum(rate(redis_write_errors{}[1m])) or vector(0)",
          "hide": false,
          "interval": "",
          "legendFormat": "total errors",
          "refId": "total_errors"
        },
        {
          "datasource": {
            "type": "prometheus",
            "uid": "dAm_6PT4z"
          },
          "exemplar": true,
          "expr": "sum(rate(redis_writes{}[1m]))",
          "hide": false,
          "interval": "",
          "legendFormat": "total",
          "refId": "total"
        },
        {
          "datasource": {
            "type": "prometheus",
            "uid": "dAm_6PT4z"
          },
          "exemplar": true,
          "expr": "sum(rate(redis_write_errors{}[1m]) or vector(0)) by (db)",
          "hide": false,
          "interval": "",
          "legendFormat": "{{db}} errors",
          "refId": "errors_per_database"
        },
        {
          "datasource": {
            "type": "prometheus",
            "uid": "dAm_6PT4z"
          },
          "exemplar": true,
          "expr": "sum(rate(redis_writes{}[1m])) by (db)",
          "interval": "",
          "legendFormat": "{{db}}",
          "refId": "per_database"
        }
      ],
      "title": "Writes / s",
      "type": "timeseries"
    },
    {
      "collapsed": false,
      "gridPos": {
        "h": 1,
        "w": 24,
        "x": 0,
        "y": 32
      },
      "id": 27,
      "panels": [],
      "title": "AWS/ECS",
      "type": "row"
    },
    {
      "datasource": {
        "type": "cloudwatch",
        "uid": "joml6Eo4k"
      },
      "fieldConfig": {
        "defaults": {
          "color": {
            "mode": "palette-classic"
          },
          "custom": {
            "axisLabel": "",
            "axisPlacement": "auto",
            "barAlignment": 0,
            "drawStyle": "line",
            "fillOpacity": 0,
            "gradientMode": "none",
            "hideFrom": {
              "legend": false,
              "tooltip": false,
              "viz": false
            },
            "lineInterpolation": "linear",
            "lineWidth": 1,
            "pointSize": 5,
            "scaleDistribution": {
              "type": "linear"
            },
            "showPoints": "auto",
            "spanNulls": false,
            "stacking": {
              "group": "A",
              "mode": "none"
            },
            "thresholdsStyle": {
              "mode": "off"
            }
          },
          "decimals": 1,
          "mappings": [],
          "thresholds": {
            "mode": "absolute",
            "steps": [
              {
                "color": "green",
                "value": null
              },
              {
                "color": "red",
                "value": 80
              }
            ]
          },
          "unit": "percent"
        },
        "overrides": [
          {
            "matcher": {
              "id": "byName",
              "options": "max"
            },
            "properties": [
              {
                "id": "color",
                "value": {
                  "fixedColor": "red",
                  "mode": "fixed"
                }
              }
            ]
          },
          {
            "matcher": {
              "id": "byName",
              "options": "min"
            },
            "properties": [
              {
                "id": "color",
                "value": {
                  "fixedColor": "green",
                  "mode": "fixed"
                }
              }
            ]
          }
        ]
      },
      "gridPos": {
        "h": 8,
        "w": 12,
        "x": 0,
        "y": 33
      },
      "id": 29,
      "options": {
        "legend": {
          "calcs": [],
          "displayMode": "list",
          "placement": "bottom"
        },
        "tooltip": {
          "mode": "single",
          "sort": "none"
        }
      },
      "targets": [
        {
          "alias": "max",
          "datasource": {
            "type": "cloudwatch",
            "uid": "joml6Eo4k"
          },
          "dimensions": {
            "ClusterName": "${environment}-verify"
          },
          "expression": "",
          "id": "",
          "matchExact": false,
          "metricEditorMode": 0,
          "metricName": "CPUUtilization",
          "metricQueryType": 0,
          "namespace": "AWS/ECS",
          "period": "",
          "queryMode": "Metrics",
          "refId": "A",
          "region": "default",
          "sqlExpression": "",
          "statistic": "Maximum"
        },
        {
          "alias": "avg",
          "datasource": {
            "type": "cloudwatch",
            "uid": "joml6Eo4k"
          },
          "dimensions": {
            "ClusterName": "${environment}-verify"
          },
          "expression": "",
          "hide": false,
          "id": "",
          "matchExact": false,
          "metricEditorMode": 0,
          "metricName": "CPUUtilization",
          "metricQueryType": 0,
          "namespace": "AWS/ECS",
          "period": "",
          "queryMode": "Metrics",
          "refId": "B",
          "region": "default",
          "sqlExpression": "",
          "statistic": "Average"
        },
        {
          "alias": "min",
          "datasource": {
            "type": "cloudwatch",
            "uid": "joml6Eo4k"
          },
          "dimensions": {
            "ClusterName": "${environment}-verify"
          },
          "expression": "",
          "hide": false,
          "id": "",
          "matchExact": false,
          "metricEditorMode": 0,
          "metricName": "CPUUtilization",
          "metricQueryType": 0,
          "namespace": "AWS/ECS",
          "period": "",
          "queryMode": "Metrics",
          "refId": "C",
          "region": "default",
          "sqlExpression": "",
          "statistic": "Minimum"
        }
      ],
      "title": "CPU Utilization",
      "type": "timeseries"
    },
    {
      "datasource": {
        "type": "cloudwatch",
        "uid": "joml6Eo4k"
      },
      "fieldConfig": {
        "defaults": {
          "color": {
            "mode": "palette-classic"
          },
          "custom": {
            "axisLabel": "",
            "axisPlacement": "auto",
            "barAlignment": 0,
            "drawStyle": "line",
            "fillOpacity": 0,
            "gradientMode": "none",
            "hideFrom": {
              "legend": false,
              "tooltip": false,
              "viz": false
            },
            "lineInterpolation": "linear",
            "lineWidth": 1,
            "pointSize": 5,
            "scaleDistribution": {
              "type": "linear"
            },
            "showPoints": "auto",
            "spanNulls": false,
            "stacking": {
              "group": "A",
              "mode": "none"
            },
            "thresholdsStyle": {
              "mode": "off"
            }
          },
          "decimals": 1,
          "mappings": [],
          "thresholds": {
            "mode": "absolute",
            "steps": [
              {
                "color": "green",
                "value": null
              },
              {
                "color": "red",
                "value": 80
              }
            ]
          },
          "unit": "percent"
        },
        "overrides": [
          {
            "matcher": {
              "id": "byName",
              "options": "max"
            },
            "properties": [
              {
                "id": "color",
                "value": {
                  "fixedColor": "red",
                  "mode": "fixed"
                }
              }
            ]
          },
          {
            "matcher": {
              "id": "byName",
              "options": "min"
            },
            "properties": [
              {
                "id": "color",
                "value": {
                  "fixedColor": "green",
                  "mode": "fixed"
                }
              }
            ]
          }
        ]
      },
      "gridPos": {
        "h": 8,
        "w": 12,
        "x": 12,
        "y": 33
      },
      "id": 30,
      "options": {
        "legend": {
          "calcs": [],
          "displayMode": "list",
          "placement": "bottom"
        },
        "tooltip": {
          "mode": "single",
          "sort": "none"
        }
      },
      "targets": [
        {
          "alias": "max",
          "datasource": {
            "type": "cloudwatch",
            "uid": "joml6Eo4k"
          },
          "dimensions": {
            "ClusterName": "${environment}-verify"
          },
          "expression": "",
          "id": "",
          "matchExact": false,
          "metricEditorMode": 0,
          "metricName": "MemoryUtilization",
          "metricQueryType": 0,
          "namespace": "AWS/ECS",
          "period": "",
          "queryMode": "Metrics",
          "refId": "A",
          "region": "default",
          "sqlExpression": "",
          "statistic": "Maximum"
        },
        {
          "alias": "avg",
          "datasource": {
            "type": "cloudwatch",
            "uid": "joml6Eo4k"
          },
          "dimensions": {
            "ClusterName": "${environment}-verify"
          },
          "expression": "",
          "hide": false,
          "id": "",
          "matchExact": false,
          "metricEditorMode": 0,
          "metricName": "MemoryUtilization",
          "metricQueryType": 0,
          "namespace": "AWS/ECS",
          "period": "",
          "queryMode": "Metrics",
          "refId": "B",
          "region": "default",
          "sqlExpression": "",
          "statistic": "Average"
        },
        {
          "alias": "min",
          "datasource": {
            "type": "cloudwatch",
            "uid": "joml6Eo4k"
          },
          "dimensions": {
            "ClusterName": "${environment}-verify"
          },
          "expression": "",
          "hide": false,
          "id": "",
          "matchExact": false,
          "metricEditorMode": 0,
          "metricName": "MemoryUtilization",
          "metricQueryType": 0,
          "namespace": "AWS/ECS",
          "period": "",
          "queryMode": "Metrics",
          "refId": "C",
          "region": "default",
          "sqlExpression": "",
          "statistic": "Minimum"
        }
      ],
      "title": "Memory Utilization",
      "type": "timeseries"
    },
    {
      "collapsed": false,
      "gridPos": {
        "h": 1,
        "w": 24,
        "x": 0,
        "y": 41
      },
      "id": 33,
      "panels": [],
      "title": "AWS/ElastiCache",
      "type": "row"
    },
    {
      "datasource": {
        "type": "cloudwatch",
        "uid": "joml6Eo4k"
      },
      "fieldConfig": {
        "defaults": {
          "color": {
            "mode": "palette-classic"
          },
          "custom": {
            "axisLabel": "",
            "axisPlacement": "auto",
            "barAlignment": 0,
            "drawStyle": "line",
            "fillOpacity": 0,
            "gradientMode": "none",
            "hideFrom": {
              "legend": false,
              "tooltip": false,
              "viz": false
            },
            "lineInterpolation": "linear",
            "lineWidth": 1,
            "pointSize": 5,
            "scaleDistribution": {
              "type": "linear"
            },
            "showPoints": "auto",
            "spanNulls": false,
            "stacking": {
              "group": "A",
              "mode": "none"
            },
            "thresholdsStyle": {
              "mode": "off"
            }
          },
          "decimals": 1,
          "mappings": [],
          "thresholds": {
            "mode": "absolute",
            "steps": [
              {
                "color": "green",
                "value": null
              },
              {
                "color": "red",
                "value": 80
              }
            ]
          },
          "unit": "percent"
        },
        "overrides": [
          {
            "matcher": {
              "id": "byName",
              "options": "max"
            },
            "properties": [
              {
                "id": "color",
                "value": {
                  "fixedColor": "red",
                  "mode": "fixed"
                }
              }
            ]
          },
          {
            "matcher": {
              "id": "byName",
              "options": "min"
            },
            "properties": [
              {
                "id": "color",
                "value": {
                  "fixedColor": "green",
                  "mode": "fixed"
                }
              }
            ]
          }
        ]
      },
      "gridPos": {
        "h": 8,
        "w": 12,
        "x": 0,
        "y": 42
      },
      "id": 31,
      "options": {
        "legend": {
          "calcs": [],
          "displayMode": "list",
          "placement": "bottom"
        },
        "tooltip": {
          "mode": "single",
          "sort": "none"
        }
      },
      "targets": [
        {
          "alias": "max",
          "datasource": {
            "type": "cloudwatch",
            "uid": "joml6Eo4k"
          },
          "dimensions": {
            "CacheClusterId": "verify-${environment}-verify",
            "CacheNodeId": "*"
          },
          "expression": "",
          "hide": false,
          "id": "",
          "matchExact": false,
          "metricEditorMode": 0,
          "metricName": "CPUUtilization",
          "metricQueryType": 0,
          "namespace": "AWS/ElastiCache",
          "period": "",
          "queryMode": "Metrics",
          "refId": "A",
          "region": "default",
          "sqlExpression": "",
          "statistic": "Maximum"
        },
        {
          "alias": "avg",
          "datasource": {
            "type": "cloudwatch",
            "uid": "joml6Eo4k"
          },
          "dimensions": {
            "CacheClusterId": "verify-${environment}-verify",
            "CacheNodeId": "*"
          },
          "expression": "",
          "hide": false,
          "id": "",
          "matchExact": false,
          "metricEditorMode": 0,
          "metricName": "CPUUtilization",
          "metricQueryType": 0,
          "namespace": "AWS/ElastiCache",
          "period": "",
          "queryMode": "Metrics",
          "refId": "B",
          "region": "default",
          "sqlExpression": "",
          "statistic": "Average"
        },
        {
          "alias": "min",
          "datasource": {
            "type": "cloudwatch",
            "uid": "joml6Eo4k"
          },
          "dimensions": {
            "CacheClusterId": "verify-${environment}-verify",
            "CacheNodeId": "*"
          },
          "expression": "",
          "hide": false,
          "id": "",
          "matchExact": false,
          "metricEditorMode": 0,
          "metricName": "CPUUtilization",
          "metricQueryType": 0,
          "namespace": "AWS/ElastiCache",
          "period": "",
          "queryMode": "Metrics",
          "refId": "C",
          "region": "default",
          "sqlExpression": "",
          "statistic": "Minimum"
        }
      ],
      "title": "CPU Utilization",
      "type": "timeseries"
    },
    {
      "datasource": {
        "type": "cloudwatch",
        "uid": "joml6Eo4k"
      },
      "fieldConfig": {
        "defaults": {
          "color": {
            "mode": "palette-classic"
          },
          "custom": {
            "axisLabel": "",
            "axisPlacement": "auto",
            "barAlignment": 0,
            "drawStyle": "line",
            "fillOpacity": 0,
            "gradientMode": "none",
            "hideFrom": {
              "legend": false,
              "tooltip": false,
              "viz": false
            },
            "lineInterpolation": "linear",
            "lineWidth": 1,
            "pointSize": 5,
            "scaleDistribution": {
              "type": "linear"
            },
            "showPoints": "auto",
            "spanNulls": false,
            "stacking": {
              "group": "A",
              "mode": "none"
            },
            "thresholdsStyle": {
              "mode": "off"
            }
          },
          "decimals": 1,
          "mappings": [],
          "thresholds": {
            "mode": "absolute",
            "steps": [
              {
                "color": "green",
                "value": null
              },
              {
                "color": "red",
                "value": 80
              }
            ]
          },
          "unit": "percent"
        },
        "overrides": [
          {
            "matcher": {
              "id": "byName",
              "options": "max"
            },
            "properties": [
              {
                "id": "color",
                "value": {
                  "fixedColor": "red",
                  "mode": "fixed"
                }
              }
            ]
          },
          {
            "matcher": {
              "id": "byName",
              "options": "min"
            },
            "properties": [
              {
                "id": "color",
                "value": {
                  "fixedColor": "green",
                  "mode": "fixed"
                }
              }
            ]
          }
        ]
      },
      "gridPos": {
        "h": 8,
        "w": 12,
        "x": 12,
        "y": 42
      },
      "id": 34,
      "options": {
        "legend": {
          "calcs": [],
          "displayMode": "list",
          "placement": "bottom"
        },
        "tooltip": {
          "mode": "single",
          "sort": "none"
        }
      },
      "targets": [
        {
          "alias": "max",
          "datasource": {
            "type": "cloudwatch",
            "uid": "joml6Eo4k"
          },
          "dimensions": {
            "CacheClusterId": "verify-${environment}-verify",
            "CacheNodeId": "*"
          },
          "expression": "",
          "hide": false,
          "id": "",
          "matchExact": false,
          "metricEditorMode": 0,
          "metricName": "DatabaseMemoryUsagePercentage",
          "metricQueryType": 0,
          "namespace": "AWS/ElastiCache",
          "period": "",
          "queryMode": "Metrics",
          "refId": "A",
          "region": "default",
          "sqlExpression": "",
          "statistic": "Maximum"
        },
        {
          "alias": "avg",
          "datasource": {
            "type": "cloudwatch",
            "uid": "joml6Eo4k"
          },
          "dimensions": {
            "CacheClusterId": "verify-${environment}-verify",
            "CacheNodeId": "*"
          },
          "expression": "",
          "hide": false,
          "id": "",
          "matchExact": false,
          "metricEditorMode": 0,
          "metricName": "DatabaseMemoryUsagePercentage",
          "metricQueryType": 0,
          "namespace": "AWS/ElastiCache",
          "period": "",
          "queryMode": "Metrics",
          "refId": "B",
          "region": "default",
          "sqlExpression": "",
          "statistic": "Average"
        },
        {
          "alias": "min",
          "datasource": {
            "type": "cloudwatch",
            "uid": "joml6Eo4k"
          },
          "dimensions": {
            "CacheClusterId": "verify-${environment}-verify",
            "CacheNodeId": "*"
          },
          "expression": "",
          "hide": false,
          "id": "",
          "matchExact": false,
          "metricEditorMode": 0,
          "metricName": "DatabaseMemoryUsagePercentage",
          "metricQueryType": 0,
          "namespace": "AWS/ElastiCache",
          "period": "",
          "queryMode": "Metrics",
          "refId": "C",
          "region": "default",
          "sqlExpression": "",
          "statistic": "Minimum"
        }
      ],
      "title": "Memory Utilization",
      "type": "timeseries"
    }
  ],
  "refresh": false,
  "schemaVersion": 35,
  "style": "dark",
  "tags": [],
  "templating": {
    "list": []
  },
  "time": {
    "from": "now-6h",
    "to": "now"
  },
  "timepicker": {},
  "timezone": "",
  "title": "${environment}_verify",
  "uid": "${environment}_verify",
  "version": 10,
  "weekStart": ""
}