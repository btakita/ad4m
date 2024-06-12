#![allow(non_snake_case)]

use std::pin::Pin;

use coasys_juniper::FieldResult;
use futures::stream;
use futures::stream::Stream;

use crate::{pubsub::{
    AGENT_STATUS_CHANGED_TOPIC, AGENT_UPDATED_TOPIC, APPS_CHANGED, EXCEPTION_OCCURRED_TOPIC, get_global_pubsub, NEIGHBOURHOOD_SIGNAL_TOPIC, PERSPECTIVE_ADDED_TOPIC, PERSPECTIVE_LINK_ADDED_TOPIC, PERSPECTIVE_LINK_REMOVED_TOPIC, PERSPECTIVE_LINK_UPDATED_TOPIC, PERSPECTIVE_REMOVED_TOPIC, PERSPECTIVE_SYNC_STATE_CHANGE_TOPIC, PERSPECTIVE_UPDATED_TOPIC, RUNTIME_MESSAGED_RECEIVED_TOPIC, RUNTIME_NOTIFICATION_TRIGGERED_TOPIC, subscribe_and_process
}, types::{DecoratedLinkExpression, TriggeredNotification}};
use crate::agent::capabilities::*;

use super::graphql_types::*;

pub struct Subscription;

#[coasys_juniper::graphql_subscription(context = RequestContext)]
impl Subscription {
    async fn agent_status_changed(
        &self,
        context: &RequestContext,
    ) -> Pin<Box<dyn Stream<Item = FieldResult<AgentStatus>> + Send>> {
        match check_capability(&context.capabilities, &AGENT_SUBSCRIBE_CAPABILITY) {
            Err(e) => return Box::pin(stream::once(async move { Err(e.into()) })),
            Ok(_) => {
                let pubsub = get_global_pubsub().await;
                let topic = &AGENT_STATUS_CHANGED_TOPIC;
                subscribe_and_process::<AgentStatus>(pubsub, topic.to_string(), None).await
            }
        }
    }

    async fn agent_apps_changed(
        &self,
        context: &RequestContext,
    ) -> Pin<Box<dyn Stream<Item = FieldResult<Option<Apps>>> + Send>> {
        match check_capability(&context.capabilities, &AGENT_SUBSCRIBE_CAPABILITY) {
            Err(e) => return Box::pin(stream::once(async move { Err(e.into()) })),
            Ok(_) => {
                let pubsub = get_global_pubsub().await;
                let topic = &APPS_CHANGED;
                subscribe_and_process::<Option<Apps>>(pubsub, topic.to_string(), None).await
            }
        }
    }

    async fn agent_updated(
        &self,
        context: &RequestContext,
    ) -> Pin<Box<dyn Stream<Item = FieldResult<Agent>> + Send>> {
        match check_capability(&context.capabilities, &AGENT_SUBSCRIBE_CAPABILITY) {
            Err(e) => return Box::pin(stream::once(async move { Err(e.into()) })),
            Ok(_) => {
                let pubsub = get_global_pubsub().await;
                let topic = &AGENT_UPDATED_TOPIC;
                subscribe_and_process::<Agent>(pubsub, topic.to_string(), None).await
            }
        }
    }

    async fn exception_occurred(
        &self,
        context: &RequestContext,
    ) -> Pin<Box<dyn Stream<Item = FieldResult<ExceptionInfo>> + Send>> {
        match check_capability(
            &context.capabilities,
            &RUNTIME_EXCEPTION_SUBSCRIBE_CAPABILITY,
        ) {
            Err(e) => return Box::pin(stream::once(async move { Err(e.into()) })),
            Ok(_) => {
                let pubsub = get_global_pubsub().await;
                let topic = &EXCEPTION_OCCURRED_TOPIC;
                subscribe_and_process::<ExceptionInfo>(pubsub, topic.to_string(), None).await
            }
        }
    }

    async fn neighbourhood_signal(
        &self,
        context: &RequestContext,
        perspectiveUUID: String,
    ) -> Pin<Box<dyn Stream<Item = FieldResult<PerspectiveExpression>> + Send>> {
        match check_capability(&context.capabilities, &NEIGHBOURHOOD_READ_CAPABILITY) {
            Err(e) => return Box::pin(stream::once(async move { Err(e.into()) })),
            Ok(_) => {
                let pubsub = get_global_pubsub().await;
                let topic = &NEIGHBOURHOOD_SIGNAL_TOPIC;
                subscribe_and_process::<NeighbourhoodSignalFilter>(
                    pubsub,
                    topic.to_string(),
                    Some(perspectiveUUID),
                )
                .await
            }
        }
    }

    async fn perspective_added(
        &self,
        context: &RequestContext,
    ) -> Pin<Box<dyn Stream<Item = FieldResult<PerspectiveHandle>> + Send>> {
        match check_capability(&context.capabilities, &PERSPECTIVE_SUBSCRIBE_CAPABILITY) {
            Err(e) => return Box::pin(stream::once(async move { Err(e.into()) })),
            Ok(_) => {
                let pubsub = get_global_pubsub().await;
                let topic = &PERSPECTIVE_ADDED_TOPIC;
                subscribe_and_process::<PerspectiveHandle>(pubsub, topic.to_string(), None).await
            }
        }
    }

    async fn perspective_link_added(
        &self,
        context: &RequestContext,
        uuid: String,
    ) -> Pin<Box<dyn Stream<Item = FieldResult<DecoratedLinkExpression>> + Send>> {
        match check_capability(&context.capabilities, &PERSPECTIVE_SUBSCRIBE_CAPABILITY) {
            Err(e) => return Box::pin(stream::once(async move { Err(e.into()) })),
            Ok(_) => {
                let pubsub = get_global_pubsub().await;
                let topic = &PERSPECTIVE_LINK_ADDED_TOPIC;
                subscribe_and_process::<PerspectiveLinkFilter>(
                    pubsub,
                    topic.to_string(),
                    Some(uuid),
                )
                .await
            }
        }
    }

    async fn perspective_link_removed(
        &self,
        context: &RequestContext,
        uuid: String,
    ) -> Pin<Box<dyn Stream<Item = FieldResult<DecoratedLinkExpression>> + Send>> {
        match check_capability(&context.capabilities, &PERSPECTIVE_SUBSCRIBE_CAPABILITY) {
            Err(e) => return Box::pin(stream::once(async move { Err(e.into()) })),
            Ok(_) => {
                let pubsub = get_global_pubsub().await;
                let topic = &PERSPECTIVE_LINK_REMOVED_TOPIC;
                subscribe_and_process::<PerspectiveLinkFilter>(
                    pubsub,
                    topic.to_string(),
                    Some(uuid),
                )
                .await
            }
        }
    }

    async fn perspective_link_updated(
        &self,
        context: &RequestContext,
        uuid: String,
    ) -> Pin<Box<dyn Stream<Item = FieldResult<LinkUpdated>> + Send>> {
        match check_capability(&context.capabilities, &PERSPECTIVE_SUBSCRIBE_CAPABILITY) {
            Err(e) => return Box::pin(stream::once(async move { Err(e.into()) })),
            Ok(_) => {
                let pubsub = get_global_pubsub().await;
                let topic = &PERSPECTIVE_LINK_UPDATED_TOPIC;
                subscribe_and_process::<PerspectiveLinkUpdatedFilter>(
                    pubsub,
                    topic.to_string(),
                    Some(uuid),
                )
                .await
            }
        }
    }

    async fn perspective_removed(
        &self,
        context: &RequestContext,
    ) -> Pin<Box<dyn Stream<Item = FieldResult<String>> + Send>> {
        match check_capability(&context.capabilities, &PERSPECTIVE_SUBSCRIBE_CAPABILITY) {
            Err(e) => return Box::pin(stream::once(async move { Err(e.into()) })),
            Ok(_) => {
                let pubsub = get_global_pubsub().await;
                let topic = &PERSPECTIVE_REMOVED_TOPIC;
                subscribe_and_process::<String>(pubsub, topic.to_string(), None).await
            }
        }
    }

    async fn perspective_sync_state_change(
        &self,
        context: &RequestContext,
        uuid: String,
    ) -> Pin<Box<dyn Stream<Item = FieldResult<String>> + Send>> {
        match check_capability(&context.capabilities, &PERSPECTIVE_SUBSCRIBE_CAPABILITY) {
            Err(e) => return Box::pin(stream::once(async move { Err(e.into()) })),
            Ok(_) => {
                let pubsub = get_global_pubsub().await;
                let topic = &PERSPECTIVE_SYNC_STATE_CHANGE_TOPIC;
                subscribe_and_process::<PerspectiveStateFilter>(
                    pubsub,
                    topic.to_string(),
                    Some(uuid),
                )
                .await
            }
        }
    }

    async fn perspective_updated(
        &self,
        context: &RequestContext,
    ) -> Pin<Box<dyn Stream<Item = FieldResult<PerspectiveHandle>> + Send>> {
        match check_capability(&context.capabilities, &PERSPECTIVE_SUBSCRIBE_CAPABILITY) {
            Err(e) => return Box::pin(stream::once(async move { Err(e.into()) })),
            Ok(_) => {
                let pubsub = get_global_pubsub().await;
                let topic = &PERSPECTIVE_UPDATED_TOPIC;
                subscribe_and_process::<PerspectiveHandle>(pubsub, topic.to_string(), None).await
            }
        }
    }

    async fn runtime_message_received(
        &self,
        context: &RequestContext,
    ) -> Pin<Box<dyn Stream<Item = FieldResult<PerspectiveExpression>> + Send>> {
        match check_capability(&context.capabilities, &PERSPECTIVE_SUBSCRIBE_CAPABILITY) {
            Err(e) => return Box::pin(stream::once(async move { Err(e.into()) })),
            Ok(_) => {
                let pubsub = get_global_pubsub().await;
                let topic = &RUNTIME_MESSAGED_RECEIVED_TOPIC;
                subscribe_and_process::<PerspectiveExpression>(pubsub, topic.to_string(), None)
                    .await
            }
        }
    }

    async fn runtime_notification_triggered(
        &self,
        context: &RequestContext,
    ) -> Pin<Box<dyn Stream<Item = FieldResult<TriggeredNotification>> + Send>> {
        match check_capability(&context.capabilities, &AGENT_READ_CAPABILITY) {
            Err(e) => return Box::pin(stream::once(async move { Err(e.into()) })),
            Ok(_) => {
                let pubsub = get_global_pubsub().await;
                let topic = &RUNTIME_NOTIFICATION_TRIGGERED_TOPIC;
                subscribe_and_process::<TriggeredNotification>(pubsub, topic.to_string(), None)
                    .await
            }
        }
    }
}
