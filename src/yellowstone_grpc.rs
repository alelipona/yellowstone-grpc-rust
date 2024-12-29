use std::{collections::HashMap, time::Duration};

use futures::{
    channel::mpsc,
    lock::Mutex,
    sink::{Sink, SinkExt},
    stream::StreamExt,
    Stream,
};
use rustls::crypto::{ring::default_provider, CryptoProvider};
use tonic::{
    codec::{CompressionEncoding, Streaming},
    metadata::{errors::InvalidMetadataValue, AsciiMetadataValue, MetadataValue},
    service::interceptor::InterceptedService,
    transport::channel::{Channel, ClientTlsConfig, Endpoint},
    Request, Response, Status,
};
use yellowstone_grpc_client::{GeyserGrpcClient, GeyserGrpcClientResult, Interceptor};
use yellowstone_grpc_proto::geyser::{
    subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest,
    SubscribeRequestFilterTransactions, SubscribeRequestPing, SubscribeUpdate,
    SubscribeUpdateTransaction,
};

use crate::myerror::AppError;

type TransactionsFilterMap = HashMap<String, SubscribeRequestFilterTransactions>;

struct YellowstoneGrpc {
    endpoint: String,
}

impl YellowstoneGrpc {
    pub fn new(endpoint: String) -> Self {
        Self { endpoint }
    }

    async fn connect(
        &self,
        watch: Vec<String>,
    ) -> Result<
        GeyserGrpcClientResult<(
            impl Sink<SubscribeRequest, Error = mpsc::SendError>,
            impl Stream<Item = Result<SubscribeUpdate, Status>>,
        )>,
        AppError,
    > {
        if CryptoProvider::get_default().is_none() {
            default_provider()
                .install_default()
                .map_err(|e| anyhow::anyhow!("Failed to install crypto provider: {:?}", e))?;
        }

        let mut client = GeyserGrpcClient::build_from_shared(self.endpoint.clone())?
            .tls_config(ClientTlsConfig::new().with_native_roots())?
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(60))
            .connect()
            .await?;

        let mut transactions: TransactionsFilterMap = HashMap::new();

        transactions.insert(
            "client".to_string(),
            SubscribeRequestFilterTransactions {
                vote: Some(false),
                failed: Some(false),
                signature: None,
                account_include: watch,
                account_exclude: vec![],
                account_required: vec![],
            },
        );
        let subscribe_request = SubscribeRequest {
            transactions,
            commitment: Some(CommitmentLevel::Processed.into()),
            ..Default::default()
        };

        Ok(client.subscribe_with_request(Some(subscribe_request)).await)
    }
}
