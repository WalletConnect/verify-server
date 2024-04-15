use {
    crate::EventSink,
    anyhow::Result,
    aws_sdk_s3::Client as S3Client,
    parquet::record::RecordWriter,
    std::{
        net::{IpAddr, Ipv4Addr},
        time::Duration,
    },
    wc::{
        analytics::{
            AnalyticsExt,
            ArcCollector,
            AwsConfig,
            AwsExporter,
            BatchCollector,
            BatchObserver,
            CollectionObserver,
            Collector,
            CollectorConfig,
            ExportObserver,
            ParquetBatchFactory,
        },
        metrics::otel,
    },
};

const ANALYTICS_EXPORT_TIMEOUT: Duration = Duration::from_secs(30);
const DATA_QUEUE_CAPACITY: usize = 8192;

#[derive(Clone, Copy)]
enum DataKind {
    Requests,
}

impl DataKind {
    #[inline]
    fn as_str(&self) -> &'static str {
        match self {
            Self::Requests => "requests",
        }
    }

    #[inline]
    fn as_kv(&self) -> otel::KeyValue {
        otel::KeyValue::new("data_kind", self.as_str())
    }
}

fn success_kv(success: bool) -> otel::KeyValue {
    otel::KeyValue::new("success", success)
}

#[derive(Clone, Copy)]
struct Observer(DataKind);

impl<T, E> BatchObserver<T, E> for Observer
where
    E: std::error::Error,
{
    fn observe_batch_serialization(&self, elapsed: Duration, res: &Result<Vec<u8>, E>) {
        let size = res.as_deref().map(|data| data.len()).unwrap_or(0);
        let elapsed = elapsed.as_millis() as u64;

        wc::metrics::counter!("analytics_batches_finished", 1, &[
            self.0.as_kv(),
            success_kv(res.is_ok())
        ]);

        if let Err(err) = res {
            tracing::warn!(
                ?err,
                data_kind = self.0.as_str(),
                "failed to serialize analytics batch"
            );
        } else {
            tracing::info!(
                size,
                elapsed,
                data_kind = self.0.as_str(),
                "analytics data batch serialized"
            );
        }
    }
}

impl<T, E> CollectionObserver<T, E> for Observer
where
    E: std::error::Error,
{
    fn observe_collection(&self, res: &Result<(), E>) {
        wc::metrics::counter!("analytics_records_collected", 1, &[
            self.0.as_kv(),
            success_kv(res.is_ok())
        ]);

        if let Err(err) = res {
            tracing::warn!(
                ?err,
                data_kind = self.0.as_str(),
                "failed to collect analytics data"
            );
        }
    }
}

impl<E> ExportObserver<E> for Observer
where
    E: std::error::Error,
{
    fn observe_export(&self, elapsed: Duration, res: &Result<(), E>) {
        wc::metrics::counter!("analytics_batches_exported", 1, &[
            self.0.as_kv(),
            success_kv(res.is_ok())
        ]);

        let elapsed = elapsed.as_millis() as u64;

        if let Err(err) = res {
            tracing::warn!(
                ?err,
                elapsed,
                data_kind = self.0.as_str(),
                "analytics export failed"
            );
        } else {
            tracing::info!(
                elapsed,
                data_kind = self.0.as_str(),
                "analytics export failed"
            );
        }
    }
}

pub struct Adapter<Record>
where
    Record: Send + Sync + 'static,
{
    s3_writer: ArcCollector<Record>,
}

pub async fn requests_dir<Record>(
    s3_client: S3Client,
    export_bucket: String,
) -> Result<Adapter<Record>>
where
    Record: Send + Sync + 'static,
    [Record]: RecordWriter<Record>,
{
    fn make_export<T>(
        data_kind: DataKind,
        s3_client: S3Client,
        export_bucket: String,
        node_addr: IpAddr,
    ) -> ArcCollector<T>
    where
        T: Sync + Send + 'static,
        [T]: RecordWriter<T>,
    {
        let observer = Observer(data_kind);
        BatchCollector::new(
            CollectorConfig {
                data_queue_capacity: DATA_QUEUE_CAPACITY,
                ..Default::default()
            },
            ParquetBatchFactory::new(Default::default()).with_observer(observer),
            AwsExporter::new(AwsConfig {
                export_prefix: format!("verify-server/{}", data_kind.as_str()),
                export_name: data_kind.as_str().to_string(),
                node_addr,
                file_extension: "parquet".to_owned(),
                bucket_name: export_bucket.to_owned(),
                s3_client,
                upload_timeout: ANALYTICS_EXPORT_TIMEOUT,
            })
            .with_observer(observer),
        )
        .with_observer(observer)
        .boxed_shared()
    }

    let s3_writer = make_export(
        DataKind::Requests,
        s3_client.clone(),
        export_bucket,
        // Used only to build a file name
        // TODO: Change it in the `wc` repo as it's not really needed for all services
        Ipv4Addr::LOCALHOST.into(),
    );

    Ok(Adapter { s3_writer })
}

impl<Ev, R> EventSink<Ev> for Adapter<R>
where
    Ev: Into<R>,
    R: Send + Sync + 'static,
{
    fn send(&self, ev: Ev) {
        if let Err(err) = self.s3_writer.collect(ev.into()) {
            tracing::warn!(
                ?err,
                data_kind = DataKind::Requests.as_str(),
                "failed to collect analytics"
            );
        }
    }
}
