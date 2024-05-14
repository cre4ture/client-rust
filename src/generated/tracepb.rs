#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraceRecordRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraceRecord {
    #[prost(oneof = "trace_record::RecordOneof", tags = "1, 2")]
    pub record_oneof: ::core::option::Option<trace_record::RecordOneof>,
}
/// Nested message and enum types in `TraceRecord`.
pub mod trace_record {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RecordOneof {
        #[prost(message, tag = "1")]
        Report(super::Report),
        #[prost(message, tag = "2")]
        NotifyCollect(super::NotifyCollect),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteParentSpan {
    /// A unique id to identify the request. It's usually a UUID.
    #[prost(uint64, tag = "1")]
    pub trace_id: u64,
    /// The span of remote caller that is awaiting the request.
    #[prost(uint64, tag = "2")]
    pub span_id: u64,
}
/// The context of the request to be traced.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraceContext {
    #[prost(message, repeated, tag = "1")]
    pub remote_parent_spans: ::prost::alloc::vec::Vec<RemoteParentSpan>,
    /// Report the trace records only if the duration of handling the request exceeds the threshold.
    #[prost(uint32, tag = "2")]
    pub duration_threshold_ms: u32,
}
/// Report the spans collected when handling a request on a service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Report {
    #[prost(message, repeated, tag = "1")]
    pub remote_parent_spans: ::prost::alloc::vec::Vec<RemoteParentSpan>,
    #[prost(message, repeated, tag = "2")]
    pub spans: ::prost::alloc::vec::Vec<Span>,
}
/// Notify the subscriber to persis the spans of the trace.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotifyCollect {
    #[prost(uint64, tag = "1")]
    pub trace_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Span {
    /// The unique span id within the spans with the same `trace_id`.
    /// The most significant 32 bits should be random number generated by each service instance.
    #[prost(uint64, tag = "1")]
    pub span_id: u64,
    #[prost(uint64, tag = "2")]
    pub parent_id: u64,
    #[prost(uint64, tag = "3")]
    pub begin_unix_ns: u64,
    #[prost(uint64, tag = "4")]
    pub duration_ns: u64,
    #[prost(string, tag = "5")]
    pub event: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "6")]
    pub properties: ::prost::alloc::vec::Vec<Property>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Property {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod trace_record_pub_sub_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct TraceRecordPubSubClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TraceRecordPubSubClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> TraceRecordPubSubClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> TraceRecordPubSubClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            TraceRecordPubSubClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Subscribe the Trace records generated on this service. The service will periodically (e.g. per minute)
        /// publishes Trace records to clients via gRPC stream.
        pub async fn subscribe(
            &mut self,
            request: impl tonic::IntoRequest<super::TraceRecordRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::TraceRecord>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tracepb.TraceRecordPubSub/Subscribe",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("tracepb.TraceRecordPubSub", "Subscribe"));
            self.inner.server_streaming(req, path, codec).await
        }
    }
}