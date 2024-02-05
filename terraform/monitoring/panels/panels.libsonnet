{
  ecs: {
    availability:         (import 'ecs/availability.libsonnet'            ).new,
    cpu:                  (import 'ecs/cpu.libsonnet'                     ).new,
    memory:               (import 'ecs/memory.libsonnet'                  ).new,
  },

  http: {
    response_status:      (import 'http/response_status.libsonnet'        ).new,
    request_response:     (import 'http/request_response.libsonnet'       ).new,
    latency_quantiles:    (import 'http/latency_quantiles.libsonnet'      ).new,
    average_latency:      (import 'http/average_latency.libsonnet'        ).new,
  },

  redis: {
    reads:                (import 'redis/reads.libsonnet'                 ).new,
    writes:               (import 'redis/writes.libsonnet'                ).new,
    cpu:                  (import 'redis/cpu.libsonnet'                   ).new,
    memory:               (import 'redis/memory.libsonnet'                ).new,
  },

  registry: {
    requests:             (import 'registry/requests.libsonnet'           ).new,
    cache_read:           (import 'registry/cache_read.libsonnet'         ).new,
    cache_write:          (import 'registry/cache_write.libsonnet'        ).new,
  }
}
