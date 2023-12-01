use {
    crate::EventSink,
    anyhow::Result,
    aws_sdk_s3::Client,
    parquet::record::RecordWriter,
    wc::analytics::{
        collectors::batch::BatchOpts,
        exporters::aws::{AwsExporter, AwsOpts},
        writers::parquet::ParquetWriter,
        Analytics,
    },
};

pub struct Adapter<Record>
where
    Record: Send + Sync + 'static,
{
    s3_writer: Analytics<Record>,
}

pub async fn new<Record>(
    client: Client,
    bucket_name: String,
    export_name: &'static str,
) -> Result<Adapter<Record>>
where
    Record: Send + Sync + 'static,
    [Record]: RecordWriter<Record>,
{
    let exporter = AwsExporter::new(AwsOpts {
        export_prefix: "verify-server",
        export_name,
        file_extension: "parquet",
        bucket_name: bucket_name.into(),
        s3_client: client,
        // Used only to build a file name
        // TODO: Change it in the `wc` repo as it's not really needed for all services
        node_ip: String::new().into(),
    });

    let opts = BatchOpts {
        event_queue_limit: 8192,
        ..Default::default()
    };

    let collector = ParquetWriter::<Record>::new(opts.clone(), exporter)?;
    Ok(Adapter {
        s3_writer: Analytics::new(collector),
    })
}

impl<Ev, R> EventSink<Ev> for Adapter<R>
where
    Ev: Into<R>,
    R: Send + Sync + 'static,
{
    fn send(&self, ev: Ev) {
        self.s3_writer.collect(ev.into());
    }
}
